//! Общие хелперы для интеграционных тестов.
//! Создаёт IncusClient, подключается к локальному Incus.

use incus_lab_manager::client::IncusClient;
use std::path::PathBuf;

pub const INCUS_URL: &str = "https://127.0.0.1:8443";

pub fn cert_path() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir).join("certs/client.crt")
}

pub fn key_path() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir).join("certs/client.key")
}

/// Создаёт клиент Incus, подключённый к локальному серверу.
pub fn make_client() -> IncusClient {
    let cert = cert_path();
    let key = key_path();

    IncusClient::new(INCUS_URL, &cert, &key)
        .expect("Не удалось создать IncusClient. Проверь сертификаты в ./certs/")
}

/// Уникальный префикс для имён ресурсов в тесте.
pub fn test_prefix() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    format!("test-{}", ts)
}
