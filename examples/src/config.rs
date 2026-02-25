use anyhow::{Result, anyhow, bail};
use log::debug;
use serde::Deserialize;
use serde::de;
use std::fs::File;
use std::path::Path;

/// Read config from YAML file
/// # Errors
/// Returns an error if failed to open, read or parse the file.
pub fn read_from_file<T>(path: &Path) -> Result<T>
where
    T: de::DeserializeOwned,
{
    debug!("Loading config file: {}", path.display());
    match File::open(path) {
        Ok(rdr) => serde_yaml::from_reader(rdr).map_err(|err| anyhow!(err)),
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

#[must_use]
pub fn default_sds_client_config() -> SDSClientConfig {
    SDSClientConfig {
        sds: default_sds_config(),
        feeds: String::new(),
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

#[must_use]
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

#[must_use]
pub fn default_ffs_client_config() -> FFSClientConfig {
    FFSClientConfig {
        connection: default_connection_config(),
        interface: None,
        rcvbuf: None,
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(default = "default_connection_config")]
pub struct ConnectionsConfig {
    pub mcast_group: String,
    pub mcast_port: u16,
}

#[must_use]
pub fn default_connection_config() -> ConnectionsConfig {
    ConnectionsConfig {
        mcast_group: String::new(),
        mcast_port: 0,
    }
}
