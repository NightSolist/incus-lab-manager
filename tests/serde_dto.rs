//! Unit-тесты сериализации сгенерированных DTO.
//! Не требуют Incus — проверяют корректность
//! serde-аннотаций и helper-функций.
//!
//! Запуск:
//!     cargo test --test serde_dto -- --nocapture

use incus_lab_manager::incus::config_map::deserialize_config_map;
use incus_lab_manager::incus::devices_map::deserialize_devices_map;
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

// ============================================================
// NETWORK
// ============================================================

#[test]
fn test_network_put_round_trip() {
    let mut config: ConfigMap = HashMap::new();
    config.insert(
        "ipv4.address".to_string(),
        "10.0.0.1/24".to_string(),
    );

    let original = NetworkPut {
        config,
        description: "test".to_string(),
        ..Default::default()
    };

    let json = serde_json::to_string(&original)
        .expect("serialize");
    println!("NetworkPut JSON: {}", json);

    let parsed: NetworkPut =
        serde_json::from_str(&json).expect("deserialize");
    assert_eq!(parsed.description, "test");
    assert_eq!(
        parsed.config.get("ipv4.address").unwrap(),
        "10.0.0.1/24"
    );
}

#[test]
fn test_networks_post_serialization() {
    let mut config: ConfigMap = HashMap::new();
    config.insert(
        "ipv4.address".to_string(),
        "auto".to_string(),
    );

    let req = NetworksPost {
        network_put: NetworkPut {
            config,
            description: "desc".to_string(),
            ..Default::default()
        },
        name: "test-net".to_string(),
        r#type: "bridge".to_string(),
        ..Default::default()
    };

    let json = serde_json::to_string(&req)
        .expect("serialize");
    println!("NetworksPost JSON: {}", json);

    assert!(json.contains("\"name\":\"test-net\""));
    assert!(json.contains("\"type\":\"bridge\""));
    assert!(json.contains("\"ipv4.address\""));
}

// ============================================================
// PROFILE
// ============================================================

#[test]
fn test_profile_put_round_trip() {
    let mut config: ConfigMap = HashMap::new();
    config.insert(
        "limits.memory".to_string(),
        "1GB".to_string(),
    );

    let original = ProfilePut {
        config,
        description: "test".to_string(),
        devices: HashMap::new(),
        ..Default::default()
    };

    let json = serde_json::to_string(&original)
        .expect("serialize");
    println!("ProfilePut JSON: {}", json);

    let parsed: ProfilePut =
        serde_json::from_str(&json).expect("deserialize");
    assert_eq!(parsed.description, "test");
    assert_eq!(
        parsed.config.get("limits.memory").unwrap(),
        "1GB"
    );
}

#[test]
fn test_profiles_post_serialization() {
    let mut devices: DevicesMap = HashMap::new();
    let mut eth0 = HashMap::new();
    eth0.insert("type".to_string(), "nic".to_string());
    devices.insert("eth0".to_string(), eth0);

    let req = ProfilesPost {
        profile_put: ProfilePut {
            config: HashMap::new(),
            description: "desc".to_string(),
            devices,
            ..Default::default()
        },
        name: "test-profile".to_string(),
        ..Default::default()
    };

    let json = serde_json::to_string(&req)
        .expect("serialize");
    let parsed: ProfilesPost =
        serde_json::from_str(&json).expect("deserialize");

    assert_eq!(parsed.name, "test-profile");
    assert!(
        parsed.profile_put.devices.contains_key("eth0")
    );
}

// ============================================================
// STORAGE POOL
// ============================================================

#[test]
fn test_storage_pool_put_round_trip() {
    let mut config: ConfigMap = HashMap::new();
    config.insert(
        "source".to_string(),
        "/var/lib/incus/pool".to_string(),
    );

    let original = StoragePoolPut {
        config,
        description: "test".to_string(),
        ..Default::default()
    };

    let json = serde_json::to_string(&original)
        .expect("serialize");
    let parsed: StoragePoolPut =
        serde_json::from_str(&json).expect("deserialize");

    assert_eq!(parsed.description, "test");
    assert_eq!(
        parsed.config.get("source").unwrap(),
        "/var/lib/incus/pool"
    );
}

#[test]
fn test_storage_pools_post_serialization() {
    let req = StoragePoolsPost {
        storage_pool_put: StoragePoolPut {
            config: HashMap::new(),
            description: "desc".to_string(),
            ..Default::default()
        },
        name: "test-pool".to_string(),
        driver: "dir".to_string(),
        ..Default::default()
    };

    let json = serde_json::to_string(&req)
        .expect("serialize");

    assert!(json.contains("\"name\":\"test-pool\""));
    assert!(json.contains("\"driver\":\"dir\""));
}

// ============================================================
// INSTANCE
// ============================================================

#[test]
fn test_instance_source_serialization() {
    let source = InstanceSource {
        r#type: "image".to_string(),
        alias: Some("alpine-3.21".to_string()),
        ..Default::default()
    };

    let json = serde_json::to_string(&source)
        .expect("serialize");
    println!("InstanceSource JSON: {}", json);

    assert!(json.contains("\"type\":\"image\""));
    assert!(json.contains("\"alias\":\"alpine-3.21\""));
}

#[test]
fn test_instances_post_serialization() {
    let source = InstanceSource {
        r#type: "image".to_string(),
        alias: Some("alpine-3.21".to_string()),
        ..Default::default()
    };

    let req = InstancesPost {
        instance_put: InstancePut {
            architecture: String::new(),
            config: HashMap::new(),
            devices: HashMap::new(),
            ephemeral: false,
            profiles: vec!["default".to_string()],
            description: "test".to_string(),
            ..Default::default()
        },
        name: "test-vm".to_string(),
        source,
        instance_type: String::new(),
        r#type: InstanceType::InstanceTypeContainer,
        start: true,
        ..Default::default()
    };

    let json = serde_json::to_string(&req)
        .expect("serialize");
    println!("InstancesPost JSON: {}", json);

    assert!(json.contains("\"name\":\"test-vm\""));
    assert!(json.contains("\"profiles\":[\"default\"]"));

    // None-поля должны быть исключены
    assert!(!json.contains("\"restore\""));
    assert!(!json.contains("\"disk_only\""));
}

// ============================================================
// CUSTOM DESERIALIZATION
// ============================================================

#[test]
fn test_config_map_bool_deserialization() {
    let json = r#"{
        "boot.autostart": true,
        "limits.cpu": 2,
        "limits.memory": "4GiB",
        "volatile.apply_template": null
    }"#;

    let mut deserializer =
        serde_json::Deserializer::from_str(json);
    let config: ConfigMap =
        deserialize_config_map(&mut deserializer)
            .expect("deserialize");

    assert_eq!(
        config.get("boot.autostart").unwrap(),
        "true"
    );
    assert_eq!(
        config.get("limits.cpu").unwrap(),
        "2"
    );
    assert_eq!(
        config.get("limits.memory").unwrap(),
        "4GiB"
    );
    assert_eq!(
        config.get("volatile.apply_template").unwrap(),
        ""
    );
}

#[test]
fn test_devices_map_deserialization() {
    let json = r#"{
        "eth0": {
            "type": "nic",
            "mtu": 1500,
            "connected": true
        }
    }"#;

    let mut deserializer =
        serde_json::Deserializer::from_str(json);
    let devices: DevicesMap =
        deserialize_devices_map(&mut deserializer)
            .expect("deserialize");

    let eth0 = devices.get("eth0").unwrap();
    assert_eq!(eth0.get("type").unwrap(), "nic");
    assert_eq!(eth0.get("mtu").unwrap(), "1500");
    assert_eq!(eth0.get("connected").unwrap(), "true");
}