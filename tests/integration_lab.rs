//! Интеграционный тест: полный сценарий деплоя лаборатории.
//!
//! Требует запущенного локального Incus.
//!
//! Запуск:
//!     cargo test --test integration_lab -- --nocapture
//!                                          --test-threads=1

mod common;

use common::make_client;
use incus_lab_manager::client::IncusClient;
use incus_lab_manager::incus::{
    ConfigMap,
    DevicesMap,
    InstancePut,
    InstanceSource,
    InstanceType,
    InstancesPost,
    NetworkPut,
    NetworksPost,
    ProfilePut,
    ProfilesPost,
    StoragePoolPut,
    StoragePoolsPost,
};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

fn short_prefix() -> String {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    format!("t{}", ts % 100000)
}

async fn cleanup(
    client: &IncusClient,
    instance: &str,
    profile: &str,
    pool: &str,
    network: &str,
) {
    println!("\n🧹 Очистка ресурсов...");
    let _ = client.stop_instance(instance).await;
    let _ = client.delete_instance(instance).await;
    let _ = client.delete_profile(profile).await;
    let _ = client.delete_storage_pool(pool).await;
    let _ = client.delete_network(network).await;
    println!("   ✓ Очистка завершена");
}

#[tokio::test]
async fn test_full_lab_deploy_lifecycle() {
    let client = make_client();
    let prefix = short_prefix();

    let network_name = format!("{}-net", prefix);
    let pool_name = format!("{}-pool", prefix);
    let profile_name = format!("{}-prof", prefix);
    let instance_name = format!("{}-inst", prefix);

    println!(
        "\n══════════════════════════════════════════════"
    );
    println!("🧪 Тест полного цикла деплоя лаборатории");
    println!("   Префикс ресурсов: {}", prefix);
    println!(
        "   network:  {} ({} симв.)",
        network_name,
        network_name.len()
    );
    println!(
        "══════════════════════════════════════════════\n"
    );

    let result = run_test_steps(
        &client,
        &network_name,
        &pool_name,
        &profile_name,
        &instance_name,
    )
    .await;

    cleanup(
        &client,
        &instance_name,
        &profile_name,
        &pool_name,
        &network_name,
    )
    .await;

    match result {
        Ok(_) => {
            println!(
                "\n══════════════════════════════════════════════"
            );
            println!(
                "✅ Тест пройден: полный сценарий деплоя работает"
            );
            println!(
                "══════════════════════════════════════════════\n"
            );
        }
        Err(e) => {
            panic!("Тест провалился: {:?}", e);
        }
    }
}

async fn run_test_steps(
    client: &IncusClient,
    network_name: &str,
    pool_name: &str,
    profile_name: &str,
    instance_name: &str,
) -> anyhow::Result<()> {
    // ====================================================
    // 1. STORAGE POOL
    // ====================================================
    println!(
        "📦 [1/5] Создаём storage pool '{}'...",
        pool_name
    );

    let mut pool_config: ConfigMap = HashMap::new();
    pool_config.insert(
        "source".to_string(),
        format!(
            "/var/lib/incus/storage-pools/{}",
            pool_name
        ),
    );

    let pool_req = StoragePoolsPost {
        storage_pool_put: StoragePoolPut {
            config: pool_config,
            description: "Test pool".to_string(),
            ..Default::default()
        },
        name: pool_name.to_string(),
        driver: "dir".to_string(),
        ..Default::default()
    };

    client.create_storage_pool(&pool_req).await?;
    println!("   ✓ Storage pool создан");

    // ====================================================
    // 2. СЕТЬ
    // ====================================================
    println!(
        "🌐 [2/5] Создаём сеть '{}'...",
        network_name
    );

    let mut net_config: ConfigMap = HashMap::new();
    net_config.insert(
        "ipv4.address".to_string(),
        "auto".to_string(),
    );
    net_config.insert(
        "ipv6.address".to_string(),
        "none".to_string(),
    );

    let net_req = NetworksPost {
        network_put: NetworkPut {
            config: net_config,
            description: "Test network".to_string(),
            ..Default::default()
        },
        name: network_name.to_string(),
        r#type: "bridge".to_string(),
        ..Default::default()
    };

    client.create_network(&net_req).await?;
    println!("   ✓ Сеть создана");

    // ====================================================
    // 3. ПРОФИЛЬ
    // ====================================================
    println!(
        "👤 [3/5] Создаём профиль '{}'...",
        profile_name
    );

    let mut devices: DevicesMap = HashMap::new();

    let mut eth0 = HashMap::new();
    eth0.insert("type".to_string(), "nic".to_string());
    eth0.insert(
        "network".to_string(),
        network_name.to_string(),
    );
    devices.insert("eth0".to_string(), eth0);

    let mut root = HashMap::new();
    root.insert("type".to_string(), "disk".to_string());
    root.insert("path".to_string(), "/".to_string());
    root.insert("pool".to_string(), pool_name.to_string());
    devices.insert("root".to_string(), root);

    let profile_req = ProfilesPost {
        profile_put: ProfilePut {
            config: HashMap::new(),
            description: "Test profile".to_string(),
            devices,
            ..Default::default()
        },
        name: profile_name.to_string(),
        ..Default::default()
    };

    client.create_profile(&profile_req).await?;
    println!("   ✓ Профиль создан");

    // ====================================================
    // 4. ИНСТАНС
    // ====================================================
    println!(
        "🚀 [4/5] Создаём инстанс '{}'...",
        instance_name
    );

    let source = InstanceSource {
        r#type: "image".to_string(),
        alias: Some("alpine-3.21".to_string()),
        ..Default::default()
    };

    let instance_req = InstancesPost {
        instance_put: InstancePut {
            architecture: String::new(),
            config: HashMap::new(),
            devices: HashMap::new(),
            ephemeral: false,
            profiles: vec![profile_name.to_string()],
            description: "Test instance".to_string(),
            ..Default::default()
        },
        name: instance_name.to_string(),
        source,
        instance_type: String::new(),
        r#type: InstanceType::InstanceTypeContainer,
        start: false,
        ..Default::default()
    };

    client.create_instance(&instance_req).await?;
    println!("   ✓ Инстанс создан");

    // ====================================================
    // 5. ПРОВЕРКА
    // ====================================================
    println!("🔍 [5/5] Все ресурсы созданы успешно");

    Ok(())
}