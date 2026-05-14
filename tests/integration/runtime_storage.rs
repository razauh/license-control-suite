use license_control_suite::modules::user_reg::auth_licensing_core::{
    AccessToken, DeviceKeyPair, DevicePublicKey, LicenseKey, LocalStateStore, MaskedLicenseKey,
    ResetRequestId, SecretStore, SessionState,
};
use license_control_suite::modules::user_reg::auth_licensing_tauri::{
    AppDataStateStore, InMemorySecretStore,
};
use license_control_suite::modules::user_reg::auth_licensing_core::DeviceResetStatus;

#[tokio::test]
async fn session_state_writes_to_expected_temp_path() {
    let dir = tempfile::tempdir().unwrap();
    let store = AppDataStateStore::new(dir.path());

    let state = SessionState::ReauthRequired {
        masked_license_key: Some(MaskedLicenseKey::new("••••-1234").unwrap()),
    };
    store.save_session_state(state).await.unwrap();

    let path = dir.path().join("session_state.json");
    assert!(path.exists());
}

#[tokio::test]
async fn reset_status_writes_to_expected_temp_path() {
    let dir = tempfile::tempdir().unwrap();
    let store = AppDataStateStore::new(dir.path());
    let status = DeviceResetStatus::Pending {
        request_id: ResetRequestId::new("reset-1").unwrap(),
        created_at_ms: 10,
    };
    store.save_reset_status(status).await.unwrap();

    let path = dir.path().join("reset_status.json");
    assert!(path.exists());
}

#[tokio::test]
async fn clearing_session_removes_license_and_access_but_keeps_device_keypair() {
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
