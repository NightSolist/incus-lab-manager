use anyhow::Result;
use std::collections::HashMap;

use crate::client::IncusClient;
use crate::config::{InstanceConfig, Lab, NetworkConfig, ProfileConfig, StoragePoolConfig};
use crate::remotes;

use crate::incus::{
    InstanceSource, InstanceType, InstancesPost, NetworksPost, ProfilesPost, StoragePoolsPost,
};

pub struct Deployer<'a> {
    client: &'a IncusClient,
}

impl<'a> Deployer<'a> {
    pub fn new(client: &'a IncusClient) -> Self {
        Self { client }
    }

    pub async fn deploy(&self, lab: &Lab) -> Result<()> {
        if let Some(desc) = &lab.description {
            println!("🚀 Deploying lab: {} ({})", lab.name, desc);
        } else {
            println!("🚀 Deploying lab: {}", lab.name);
        }

        // Порядок важен: pool → network → profile → instance
        for pool in &lab.storage_pools {
            self.deploy_storage_pool(pool).await?;
        }

        for net in &lab.networks {
            self.deploy_network(net).await?;
        }

        for profile in &lab.profiles {
            self.deploy_profile(profile).await?;
        }

        for instance in &lab.instances {
            self.deploy_instance(instance).await?;
        }

        println!("✅ Lab deployed successfully!");
        Ok(())
    }

    pub async fn destroy(&self, lab: &Lab) -> Result<()> {
        println!("🔥 Destroying lab: {}", lab.name);

        // Обратный порядок: instance → profile → network → pool
        for instance in &lab.instances {
            println!("   🗑️  Deleting instance: {}", instance.name);
            let _ = self.client.delete_instance(&instance.name).await;
        }

        for profile in &lab.profiles {
            println!("   🗑️  Deleting profile: {}", profile.name);
            let _ = self.client.delete_profile(&profile.name).await;
        }

        for net in &lab.networks {
            println!("   🗑️  Deleting network: {}", net.name);
            let _ = self.client.delete_network(&net.name).await;
        }

        for pool in &lab.storage_pools {
            println!("   🗑️  Deleting storage pool: {}", pool.name);
            let _ = self.client.delete_storage_pool(&pool.name).await;
        }

        println!("✅ Lab destroyed.");
        Ok(())
    }

    async fn deploy_storage_pool(&self, config: &StoragePoolConfig) -> Result<()> {
        println!("💾 Deploying storage pool '{}'", config.name);

        // На случай если уже существует
        let _ = self.client.delete_storage_pool(&config.name).await;

        let mut pool_config = HashMap::new();
        if let Some(src) = &config.source {
            pool_config.insert("source".to_string(), src.clone());
        }

        let req = StoragePoolsPost {
            name: config.name.clone(),
            driver: config.driver.clone(),
            config: pool_config,
            description: config.description.clone().unwrap_or_else(|| String::new()),
        };

        self.client.create_storage_pool(&req).await?;
        println!("✅ Created storage pool {}", config.name);
        Ok(())
    }

    async fn deploy_network(&self, config: &NetworkConfig) -> Result<()> {
        println!("🌐 Deploying network '{}'", config.name);

        let _ = self.client.delete_network(&config.name).await;

        let mut net_config = HashMap::new();
        if let Some(ipv4) = &config.ipv4 {
            net_config.insert("ipv4.address".to_string(), ipv4.clone());
            net_config.insert("ipv4.nat".to_string(), "true".to_string());
        }

        if let Some(ipv6) = &config.ipv6 {
            if ipv6 == "none" {
                net_config.insert("ipv6.address".to_string(), "none".to_string());
            } else {
                net_config.insert("ipv6.address".to_string(), ipv6.clone());
                net_config.insert("ipv6.nat".to_string(), "true".to_string());
            }
        } else {
            net_config.insert("ipv6.address".to_string(), "none".to_string());
        }

        if let Some(dns) = &config.dns_domain {
            net_config.insert("dns.domain".to_string(), dns.clone());
        }

        let req = NetworksPost {
            name: config.name.clone(),
            r#type: "bridge".to_string(),
            config: net_config,
            ..Default::default()
        };

        self.client.create_network(&req).await?;
        println!("✅ Created network {}", config.name);
        Ok(())
    }

    async fn deploy_profile(&self, config: &ProfileConfig) -> Result<()> {
        println!("👤 Deploying profile '{}'", config.name);

        let _ = self.client.delete_profile(&config.name).await;

        let mut devices = HashMap::new();

        // eth0 — если указан network
        if let Some(net_name) = &config.network {
            let mut eth0 = HashMap::new();
            eth0.insert("type".to_string(), "nic".to_string());
            eth0.insert("nictype".to_string(), "bridged".to_string());
            eth0.insert("parent".to_string(), net_name.clone());
            eth0.insert("name".to_string(), "eth0".to_string());
            devices.insert("eth0".to_string(), eth0);
        }

        // root — если указан storage
        if let Some(pool_name) = &config.storage {
            let mut root = HashMap::new();
            root.insert("type".to_string(), "disk".to_string());
            root.insert("path".to_string(), "/".to_string());
            root.insert("pool".to_string(), pool_name.clone());
            devices.insert("root".to_string(), root);
        }

        let req = ProfilesPost {
            name: config.name.clone(),
            config: config.config.clone().into_iter().collect(),
            description: config.description.clone().unwrap_or_else(|| String::new()),
            devices,
        };

        self.client.create_profile(&req).await?;
        println!("✅ Created profile {}", config.name);
        Ok(())
    }

    async fn deploy_instance(&self, config: &InstanceConfig) -> Result<()> {
        println!("📦 Deploying {} ({})", config.name, config.type_);
        let _ = self.client.delete_instance(&config.name).await;

        let (server, protocol, alias) = remotes::parse_image(&config.image);
        let type_enum = if config.type_ == "virtual-machine" {
            InstanceType::InstanceTypeVM
        } else {
            InstanceType::InstanceTypeContainer
        };

        let mut devices = HashMap::new();
        if let Some(net_name) = &config.network {
            let mut eth0 = HashMap::new();
            eth0.insert("type".to_string(), "nic".to_string());
            eth0.insert("nictype".to_string(), "bridged".to_string());
            eth0.insert("parent".to_string(), net_name.clone());
            eth0.insert("name".to_string(), "eth0".to_string());

            devices.insert("eth0".to_string(), eth0);
        }

        let req = InstancesPost {
            name: config.name.clone(),
            r#type: type_enum,
            source: InstanceSource {
                r#type: "image".to_string(),
                mode: Some("pull".to_string()),
                server: Some(server),
                protocol: Some(protocol),
                alias: Some(alias),
                ..Default::default()
            },
            profiles: if config.profiles.is_empty() {
                vec!["default".to_string()]
            } else {
                config.profiles.clone()
            },
            devices,
            config: config.config.clone().into_iter().collect(),
            start: config.start,
            ..Default::default()
        };

        self.client.create_instance(&req).await?;
        println!("✅ Created {}", config.name);

        Ok(())
    }
}
