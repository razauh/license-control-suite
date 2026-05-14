use async_trait::async_trait;
use crate::modules::user_reg::auth_licensing_core::{
    AccessToken, AuthError, DeviceKeyPair, DeviceResetStatus, LicenseKey, LocalStateStore,
    SecretStore, SessionState,
};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tauri::Manager;

#[derive(Clone, Default)]
pub struct InMemorySecretStore {
    license_key: Arc<Mutex<Option<LicenseKey>>>,
    access_token: Arc<Mutex<Option<AccessToken>>>,
    device_keypair: Arc<Mutex<Option<DeviceKeyPair>>>,
}

#[async_trait]
impl SecretStore for InMemorySecretStore {
    async fn put_license_key(&self, value: LicenseKey) -> Result<(), AuthError> {
        *self.license_key.lock().unwrap() = Some(value);
        Ok(())
    }

    async fn get_license_key(&self) -> Result<Option<LicenseKey>, AuthError> {
        Ok(self.license_key.lock().unwrap().clone())
    }

    async fn put_access_token(&self, value: AccessToken) -> Result<(), AuthError> {
        *self.access_token.lock().unwrap() = Some(value);
        Ok(())
    }

    async fn get_access_token(&self) -> Result<Option<AccessToken>, AuthError> {
        Ok(self.access_token.lock().unwrap().clone())
    }

    async fn put_device_keypair(&self, value: DeviceKeyPair) -> Result<(), AuthError> {
        *self.device_keypair.lock().unwrap() = Some(value);
        Ok(())
    }

    async fn get_device_keypair(&self) -> Result<Option<DeviceKeyPair>, AuthError> {
        Ok(self.device_keypair.lock().unwrap().clone())
    }

    async fn clear_session_secrets(&self) -> Result<(), AuthError> {
        *self.license_key.lock().unwrap() = None;
        *self.access_token.lock().unwrap() = None;
        Ok(())
    }
}

#[derive(Clone)]
pub struct AppDataStateStore {
    root: PathBuf,
}

impl AppDataStateStore {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn from_app_handle<R>(app: &tauri::AppHandle<R>) -> Result<Self, AuthError>
    where
        R: tauri::Runtime,
    {
        let root = app
            .path()
            .app_data_dir()
            .map_err(|err| AuthError::Storage(err.to_string()))?;
        Ok(Self::new(root))
    }

    fn session_path(&self) -> PathBuf {
        self.root.join("session_state.json")
    }

    fn reset_path(&self) -> PathBuf {
        self.root.join("reset_status.json")
    }

    fn read_json<T: for<'de> serde::Deserialize<'de>>(path: &Path) -> Result<Option<T>, AuthError> {
        if !path.exists() {
            return Ok(None);
        }
        let bytes = std::fs::read(path).map_err(|err| AuthError::Storage(err.to_string()))?;
        serde_json::from_slice(&bytes)
            .map(Some)
            .map_err(|err| AuthError::Serialization(err.to_string()))
    }

    fn write_json<T: serde::Serialize>(&self, path: &Path, value: &T) -> Result<(), AuthError> {
        std::fs::create_dir_all(&self.root).map_err(|err| AuthError::Storage(err.to_string()))?;
        let bytes = serde_json::to_vec_pretty(value)
            .map_err(|err| AuthError::Serialization(err.to_string()))?;
        std::fs::write(path, bytes).map_err(|err| AuthError::Storage(err.to_string()))
    }
}

#[async_trait]
impl LocalStateStore for AppDataStateStore {
    async fn save_session_state(&self, state: SessionState) -> Result<(), AuthError> {
        self.write_json(&self.session_path(), &state)
    }

    async fn load_session_state(&self) -> Result<SessionState, AuthError> {
        Ok(Self::read_json(&self.session_path())?.unwrap_or_default())
    }

    async fn save_reset_status(&self, status: DeviceResetStatus) -> Result<(), AuthError> {
        self.write_json(&self.reset_path(), &status)
    }

    async fn load_reset_status(&self) -> Result<Option<DeviceResetStatus>, AuthError> {
        Self::read_json(&self.reset_path())
    }
}

#[derive(Clone, Default)]
pub struct KeychainSecretStore {
    service_name: String,
}

impl KeychainSecretStore {
    pub fn new_for_app(service_name: impl Into<String>) -> Self {
        Self {
            service_name: service_name.into(),
        }
    }

    fn entry(&self, key: &str) -> Result<keyring::Entry, AuthError> {
        keyring::Entry::new(&self.service_name, key)
            .map_err(|err| AuthError::Storage(err.to_string()))
    }

    fn set(&self, key: &str, value: &str) -> Result<(), AuthError> {
        self.entry(key)?
            .set_password(value)
            .map_err(|err| AuthError::Storage(err.to_string()))
    }

    fn get(&self, key: &str) -> Result<Option<String>, AuthError> {
        match self.entry(key)?.get_password() {
            Ok(value) => Ok(Some(value)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(err) => Err(AuthError::Storage(err.to_string())),
        }
    }

    fn delete(&self, key: &str) -> Result<(), AuthError> {
        match self.entry(key)?.delete_credential() {
            Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
            Err(err) => Err(AuthError::Storage(err.to_string())),
        }
    }
}

#[async_trait]
impl SecretStore for KeychainSecretStore {
    async fn put_license_key(&self, value: LicenseKey) -> Result<(), AuthError> {
        self.set("license_key", value.expose_secret())
    }

    async fn get_license_key(&self) -> Result<Option<LicenseKey>, AuthError> {
        self.get("license_key")?.map(LicenseKey::new).transpose()
    }

    async fn put_access_token(&self, value: AccessToken) -> Result<(), AuthError> {
        self.set("access_token", value.expose_secret())
    }

    async fn get_access_token(&self) -> Result<Option<AccessToken>, AuthError> {
        self.get("access_token")?.map(AccessToken::new).transpose()
    }

    async fn put_device_keypair(&self, value: DeviceKeyPair) -> Result<(), AuthError> {
        let json = serde_json::to_string(&value)
            .map_err(|err| AuthError::Serialization(err.to_string()))?;
        self.set("device_keypair", &json)
    }

    async fn get_device_keypair(&self) -> Result<Option<DeviceKeyPair>, AuthError> {
        self.get("device_keypair")?
            .map(|json| {
                serde_json::from_str(&json).map_err(|err| AuthError::Serialization(err.to_string()))
            })
            .transpose()
    }

    async fn clear_session_secrets(&self) -> Result<(), AuthError> {
        self.delete("license_key")?;
        self.delete("access_token")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::user_reg::auth_licensing_core::{DevicePublicKey, MaskedLicenseKey, ResetRequestId};

    #[tokio::test]
    async fn app_data_store_missing_state_defaults_to_unauthenticated() {
        let dir = tempfile::tempdir().unwrap();
        let store = AppDataStateStore::new(dir.path());
        assert_eq!(
            store.load_session_state().await.unwrap(),
            SessionState::Unauthenticated
        );
    }

    #[tokio::test]
    async fn app_data_store_persists_reset_status() {
        let dir = tempfile::tempdir().unwrap();
        let store = AppDataStateStore::new(dir.path());
        let status = DeviceResetStatus::Pending {
            request_id: ResetRequestId::new("reset-1").unwrap(),
            created_at_ms: 10,
        };
        store.save_reset_status(status.clone()).await.unwrap();
        assert_eq!(store.load_reset_status().await.unwrap(), Some(status));
    }

    #[tokio::test]
    async fn corrupt_app_data_state_is_recoverable_error() {
        let dir = tempfile::tempdir().unwrap();
        let store = AppDataStateStore::new(dir.path());
        std::fs::write(dir.path().join("session_state.json"), b"not-json").unwrap();
        assert!(matches!(
            store.load_session_state().await.unwrap_err(),
            AuthError::Serialization(_)
        ));
    }

    #[tokio::test]
    async fn clearing_session_removes_secrets() {
        let store = InMemorySecretStore::default();
        store
            .put_license_key(LicenseKey::new("key").unwrap())
            .await
            .unwrap();
        store
            .put_access_token(AccessToken::new("token").unwrap())
            .await
            .unwrap();
        store
            .put_device_keypair(
                DeviceKeyPair::new(DevicePublicKey::new("public").unwrap(), "private").unwrap(),
            )
            .await
            .unwrap();
        store.clear_session_secrets().await.unwrap();
        assert!(store.get_license_key().await.unwrap().is_none());
        assert!(store.get_access_token().await.unwrap().is_none());
        assert!(store.get_device_keypair().await.unwrap().is_some());
    }

    #[tokio::test]
    #[ignore = "manual smoke test; requires an available OS keychain backend"]
    async fn manual_keychain_round_trip() {
        let store = KeychainSecretStore::new_for_app("auth-licensing-test");
        store
            .put_license_key(LicenseKey::new("key").unwrap())
            .await
            .unwrap();
        assert!(store.get_license_key().await.unwrap().is_some());
    }

    #[tokio::test]
    async fn app_data_store_persists_non_secret_session_state() {
        let dir = tempfile::tempdir().unwrap();
        let store = AppDataStateStore::new(dir.path());
        let state = SessionState::ReauthRequired {
            masked_license_key: Some(MaskedLicenseKey::new("••••-1234").unwrap()),
        };
        store.save_session_state(state.clone()).await.unwrap();
        assert_eq!(store.load_session_state().await.unwrap(), state);
    }
}
