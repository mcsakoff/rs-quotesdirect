use anyhow::{Result, bail};
use bytes::Bytes;
use fastlib::Decoder;
use log::{debug, error};
use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncRead, AsyncWriteExt, BufReader, BufStream};
use tokio::net::TcpStream;

use quotesdirectlib::{
    fast::{Message, TEMPLATES_XML},
    fix::{login, logout, request},
    packets::TCPPacket,
};

enum DataSource {
    Tcp(BufStream<TcpStream>),
    File(BufReader<File>),
}

pub struct SDSConnection {
    source: Option<DataSource>,
    decoder: Decoder,
    in_seq_num_pkt: u32,
    in_seq_num_msg: u32,
    out_seq_num: u32,
    buff: Bytes,
}

impl SDSConnection {
    pub fn new() -> Self {
        debug!("Creating new SDSConnection");
        Self {
            source: None,
            decoder: Decoder::new_from_xml(TEMPLATES_XML).unwrap(),
            in_seq_num_pkt: 1,
            in_seq_num_msg: 1,
            out_seq_num: 1,
            buff: Bytes::new(),
        }
    }

    pub async fn connect(&mut self, host: &str, port: u16) -> Result<()> {
        debug!("Connecting to {host}:{port}");
        let stream = TcpStream::connect(format!("{host}:{port}")).await?;
        self.source = Some(DataSource::Tcp(BufStream::new(stream)));
        Ok(())
    }

    pub async fn read_file(&mut self, path: &Path) -> Result<()> {
        let file = File::open(path).await?;
        self.source = Some(DataSource::File(BufReader::new(file)));
        Ok(())
    }

    pub async fn login(&mut self, user: &str, password: &str) -> Result<()> {
        debug!("Logging in as {user}");
        if let Some(DataSource::Tcp(stream)) = &mut self.source {
            let msg = login(self.out_seq_num, user, password, 60);
            stream.write_all(&msg).await?;
            stream.flush().await?;
            self.out_seq_num += 1;
        }
        Ok(())
    }

    pub async fn request(&mut self, feed_id: u32) -> Result<()> {
        if let Some(DataSource::Tcp(stream)) = &mut self.source {
            let msg = request(self.out_seq_num, feed_id);
            stream.write_all(&msg).await?;
            stream.flush().await?;
            self.out_seq_num += 1;
        }
        Ok(())
    }

    pub async fn logout(&mut self) -> Result<()> {
        if let Some(DataSource::Tcp(stream)) = &mut self.source {
            let msg = logout(self.out_seq_num, "Logout");
            stream.write_all(&msg).await?;
            stream.flush().await?;
            self.out_seq_num += 1;
        }
        Ok(())
    }

    pub async fn read_message(&mut self) -> Result<Option<Message>> {
        if self.buff.is_empty() {
            // read next packet
            let source: &mut (dyn AsyncRead + Unpin) = match self.source {
                Some(DataSource::Tcp(ref mut stream)) => stream,
                Some(DataSource::File(ref mut file)) => file,
                None => bail!("source not initialized"),
            };
            let Some(packet) = TCPPacket::read(source).await? else {
                return Ok(None);
            };

            // check packet's sequence number
            if packet.seq_num != self.in_seq_num_pkt {
                error!(
                    "expected packet seq_num={} but got={}",
                    self.in_seq_num_pkt, packet.seq_num
                );
                self.in_seq_num_pkt = packet.seq_num + 1; // reset sequence number
            }
            self.in_seq_num_pkt += 1;

            // fill-in internal buffer
            self.buff = bytes::Bytes::from(packet.payload);
        }

        // decode message
        let msg: Message = fastlib::from_bytes(&mut self.decoder, &mut self.buff)?;

        // check message's sequence number
        let seq_num = match &msg {
            Message::MDSecurityDefinition(m) => m.msg_header.msg_seq_num,
            Message::MDHeartbeat(m) => m.msg_header.msg_seq_num,
            Message::MDLogon(m) => m.msg_header.msg_seq_num,
            Message::MDLogout(m) => m.msg_header.msg_seq_num,
            Message::MDSecurityDefinitionRequest(m) => m.msg_header.msg_seq_num,
            _ => {
                bail!("unexpected SDS message: {msg:#?}");
            }
        };
        if seq_num != self.in_seq_num_msg {
            error!(
                "expected msg seq_num={} but got={}",
                self.in_seq_num_msg, seq_num
            );
            self.in_seq_num_msg = seq_num + 1; // reset sequence number
        }
        self.in_seq_num_msg += 1;

        Ok(Some(msg))
    }

    pub fn reset(&mut self) {
        self.decoder.reset();
    }
}
