use anyhow::{anyhow, bail, Result};
use log::debug;
use serde::de;
use serde::Deserialize;
use std::fs::File;
use std::path::Path;

pub fn read_from_file<T>(path: &Path) -> Result<T>
where
    T: de::DeserializeOwned,
{
    debug!("Loading config file: {}", path.display());
    match File::open(path) {
        Ok(rdr) => serde_yaml::from_reader(rdr).or_else(|err| Err(anyhow!(err))),
        Err(err) => {
            bail!("Failed to open config file {}: {err}", path.display());
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(default = "default_sds_client_config")]
pub struct SDSClientConfig {
    pub sds: SDSConfig,
    pub feeds: String,
    pub stop_on_disconnect: bool,
}

pub fn default_sds_client_config() -> SDSClientConfig {
    SDSClientConfig {
        sds: default_sds_config(),
        feeds: "".to_string(),
        stop_on_disconnect: true,
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(default = "default_sds_config")]
pub struct SDSConfig {
    pub host: String,
    pub port: u16,
    pub login: String,
    pub password: String,
}

pub fn default_sds_config() -> SDSConfig {
    SDSConfig {
        host: "127.0.0.1".to_string(),
        port: 2222,
        login: "test".to_string(),
        password: "test".to_string(),
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(default = "default_ffs_client_config")]
pub struct FFSClientConfig {
    pub connection: ConnectionsConfig,
    pub interface: Option<String>,
    pub rcvbuf: Option<usize>,
}

pub fn default_ffs_client_config() -> FFSClientConfig {
    FFSClientConfig {
        connection: default_connection_config(),
        interface: None,
        rcvbuf: None
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(default = "default_connection_config")]
pub struct ConnectionsConfig {
    pub mcast_group: String,
    pub mcast_port: u16,
}

pub fn default_connection_config() -> ConnectionsConfig {
    ConnectionsConfig {
        mcast_group: "".to_string(),
        mcast_port: 0,
    }
}
