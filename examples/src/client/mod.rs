use anyhow::Result;
use std::collections::HashSet;

use quotesdirectlib::fast::Message;

use self::connection::SDSConnection;
pub use self::feeds::Feeds;

#[allow(dead_code)]
pub(crate) mod connection;
pub(crate) mod feeds;

const SEC_IDS_CAPACITY: usize = 1500000;

pub struct SDSClient {
    pub defs_count_total: u32,
    pub defs_count: u32,

    sds: SDSConnection,
    sec_ids: HashSet<(u32, u32)>,
}

impl SDSClient {
    pub fn new() -> Self {
        Self {
            sds: SDSConnection::new(),
            sec_ids: HashSet::with_capacity(SEC_IDS_CAPACITY),
            defs_count_total: 0,
            defs_count: 0,
        }
    }

    pub async fn connect(
        &mut self,
        host: &str,
        port: u16,
        user: &str,
        password: &str,
    ) -> Result<()> {
        self.sds.connect(host, port).await?;
        self.sds.login(user, password).await?;
        Ok(())
    }

    #[inline]
    pub async fn subscribe(&mut self, feed_id: u32) -> Result<()> {
        
        self.sds.request(feed_id).await
    }

    pub async fn subscribe_feeds(&mut self, feeds: Feeds) -> Result<()> {
        for feed_id in feeds {
            self.subscribe(feed_id).await?;
        }
        Ok(())
    }

    pub async fn read_message(&mut self) -> Result<Option<(Message, bool)>> {
        let message = match self.sds.read_message().await? {
            Some(m) => m,
            None => return Ok(None),
        };
        match &message {
            Message::MDSecurityDefinition(m) => {
                self.defs_count_total = m.tot_num_reports;

                let feed_id: u32 = m.appl_id.parse()?;
                let key = (feed_id, m.security_id);
                let is_update = self.sec_ids.contains(&key);
                if !is_update {
                    self.sec_ids.insert(key);
                    self.defs_count += 1;
                }
                Ok(Some((message, is_update)))
            }
            Message::MDHeartbeat(_) => Ok(Some((message, false))),
            Message::MDLogon(_) => Ok(Some((message, false))),
            Message::MDLogout(_) => Ok(Some((message, false))),
            Message::MDSecurityDefinitionRequest(_) => Ok(Some((message, false))),
            _ => unreachable!(),
        }
    }

    pub async fn logout(&mut self) -> Result<()> {
        self.sds.logout().await
    }

    #[inline]
    pub fn is_subscribed(&self) -> bool {
        self.defs_count == self.defs_count_total
    }

    #[inline]
    pub fn progress(&self) -> f64 {
        if self.defs_count_total == 0 {
            return 0.0;
        }
        (self.defs_count as f64 / self.defs_count_total as f64) * 100.0
    }
}

impl Default for SDSClient {
    fn default() -> Self {
        Self::new()
    }
}
