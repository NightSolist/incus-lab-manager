//! Unit-тесты сериализации сгенерированных DTO.
//! Не требуют Incus — проверяют только корректность serde-аннотаций.
//!
//! Запуск:
//!     cargo test --test serde_dto

use incus_lab_manager::incus::{
    InstanceSource, InstancesPost, NetworkPut, NetworksPost, ProfilePut, ProfilesPost,
    StoragePoolPut, StoragePoolsPost,
};
use std::collections::HashMap;

// ============================================================
// NETWORK
// ============================================================

#[test]
fn test_network_put_round_trip() {
    let mut config = HashMap::new();
    config.insert("ipv4.address".to_string(), "10.0.0.1/24".to_string());

    let original = NetworkPut {
        config,
        description: "test".to_string(),
    };

    let json = serde_json::to_string(&original).expect("serialize");
    println!("NetworkPut JSON: {}", json);

    let parsed: NetworkPut = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(parsed.description, "test");
    assert_eq!(parsed.config.get("ipv4.address").unwrap(), "10.0.0.1/24");
}

#[test]
fn test_networks_post_serialization() {
    let mut config = HashMap::new();
    config.insert("ipv4.address".to_string(), "auto".to_string());

    let req = NetworksPost {
        name: "test-net".to_string(),
        r#type: "bridge".to_string(),
        config,
        description: "desc".to_string(),
    };

    let json = serde_json::to_string(&req).expect("serialize");
    println!("NetworksPost JSON: {}", json);

    assert!(json.contains("\"name\":\"test-net\""));
    assert!(json.contains("\"type\":\"bridge\""));
}

// ============================================================
// PROFILE
// ============================================================

#[test]
fn test_profile_put_round_trip() {
    let mut config = HashMap::new();
    config.insert("limits.memory".to_string(), "1GB".to_string());

    let original = ProfilePut {
        config,
        description: "test".to_string(),
        devices: HashMap::new(),
    };

    let json = serde_json::to_string(&original).expect("serialize");
    println!("ProfilePut JSON: {}", json);

    let parsed: ProfilePut = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(parsed.description, "test");
    assert_eq!(parsed.config.get("limits.memory").unwrap(), "1GB");
}

#[test]
fn test_profiles_post_serialization() {
    let mut devices = HashMap::new();
    let mut eth0 = HashMap::new();
    eth0.insert("type".to_string(), "nic".to_string());
    devices.insert("eth0".to_string(), eth0);

    let req = ProfilesPost {
        name: "test-profile".to_string(),
        config: HashMap::new(),
        description: "desc".to_string(),
        devices,
    };

    let json = serde_json::to_string(&req).expect("serialize");
    let parsed: ProfilesPost = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(parsed.name, "test-profile");
    assert!(parsed.devices.contains_key("eth0"));
}

// ============================================================
// STORAGE POOL
// ============================================================

#[test]
fn test_storage_pool_put_round_trip() {
    let mut config = HashMap::new();
    config.insert("source".to_string(), "/var/lib/incus/pool".to_string());

    let original = StoragePoolPut {
        config,
        description: "test".to_string(),
    };

    let json = serde_json::to_string(&original).expect("serialize");
    let parsed: StoragePoolPut = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(parsed.description, "test");
}

#[test]
fn test_storage_pools_post_serialization() {
    let req = StoragePoolsPost {
        name: "test-pool".to_string(),
        driver: "dir".to_string(),
        config: HashMap::new(),
        description: "desc".to_string(),
    };

    let json = serde_json::to_string(&req).expect("serialize");
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

    let json = serde_json::to_string(&source).expect("serialize");
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
        name: "test-vm".to_string(),
        source,
        instancetype: "container".to_string(),
        r#type: Default::default(),
        start: true,
        architecture: String::new(),
        config: HashMap::new(),
        devices: HashMap::new(),
        ephemeral: false,
        profiles: vec!["default".to_string()],
        restore: None,
        diskonly: None,
        stateful: false,
        description: "test".to_string(),
    };

    let json = serde_json::to_string(&req).expect("serialize");
    println!("InstancesPost JSON: {}", json);

    assert!(json.contains("\"name\":\"test-vm\""));
    assert!(json.contains("\"profiles\":[\"default\"]"));

    // Проверяем что None-поля исключены (omitempty работает)
    assert!(!json.contains("\"restore\""));
}
