//! # Packet formats used by Quotes Direct API
//!
//! [Preamble and Header Formats](https://help.cqg.com/apihelp/#!Documents/preambleandheaderformats.htm)
//!
//! A preamble is sent before any FAST encoded message. It consists of 5 non-FAST encoded bytes in Big Endian format,
//! of which the first 4 bytes represent the sequence number, followed by the 1-byte sub-channel identifier,
//! which is always 0x00 at the moment.
//!
//! For FAST messages sent over TCP, a FAST encoded message length (1-3 bytes) precedes the preamble.
//! Processing of the Preamble is optional and FAST messages will not be impacted by it.
//!
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use crate::{Result, Error};
pub use crate::sync::packets::UDPPacket;

#[derive(Debug)]
pub struct TCPPacket {
    pub seq_num: u32,
    pub sub_channel: u8,
    pub payload: Vec<u8>,
}

/// TCP packet reader and writer
///
/// # Examples
///
/// ```rust,ignore
/// use tokio::net::TcpStream;
/// use tokio::io::{AsyncRead, BufStream};
/// use quotesdirectlib::packets::TCPPacket;
///
/// let mut stream = BufStream::new(
///     TcpStream::connect("127.0.0.1:2345").await?
/// );
/// let packet = TCPPacket::read(&mut stream)?;
/// ```
///
impl TCPPacket {
    pub async fn read(input: &mut (dyn AsyncRead + Unpin)) -> Result<Option<TCPPacket>> {
        // read length
        let len = match read_var_uint(input).await? {
            Some(len) => {
                if len < 5 {
                    return Err(Error::InvalidPacketLength(len));
                }
                len - 5
            }
            None => return Ok(None),
        };
        // read seq_num + sub_channel
        let mut buffer = [0; 5];
        input.read_exact(&mut buffer).await?;
        // read payload
        let mut payload = Vec::with_capacity(len as usize);
        #[allow(clippy::uninit_vec)]
        unsafe {
            payload.set_len(len as usize);
        }
        input.read_exact(&mut payload).await?;

        Ok(Some(TCPPacket {
            seq_num: (buffer[0] as u32) << 24 | (buffer[1] as u32) << 16 | (buffer[2] as u32) << 8 | (buffer[3] as u32),
            sub_channel: buffer[4],
            payload,
        }))
    }

    pub async fn write(&self, output: &mut (dyn AsyncWrite + Unpin)) -> Result<()> {
        // write length
        let len = 5 + (self.payload.len() as u64);
        write_var_uint(output, len).await?;
        // write seq_num + sub_channel
        output.write_all(&[
            (self.seq_num >> 24) as u8,
            (self.seq_num >> 16) as u8,
            (self.seq_num >> 8) as u8,
            self.seq_num as u8,
            self.sub_channel
        ]).await?;
        // write payload
        output.write_all(&self.payload).await?;
        Ok(())
    }
}

async fn read_var_uint(input: &mut (dyn AsyncRead + Unpin)) -> Result<Option<u64>> {
    let mut value: u64 = 0;

    let mut buffer = [0; 1];
    if input.read_exact(&mut buffer).await.is_err() {
        // failed reading the first byte means we reached end of file
        return Ok(None);
    }

    loop {
        let byte = buffer[0];
        value <<= 7;
        value |= (byte & 0x7f) as u64;
        if byte & 0x80 == 0x80 {
            return Ok(Some(value));
        }
        input.read_exact(&mut buffer).await?;
    }
}

async fn write_var_uint(output: &mut (dyn AsyncWrite + Unpin), mut value: u64) -> Result<()> {
    let mut buf: Vec<u8> = Vec::with_capacity(4);
    buf.push(((value & 0x7f) as u8) | 0x80);
    loop {
        value >>= 7;
        if value == 0 {
            break;
        }
        buf.push((value & 0x7f) as u8);
    }
    buf.reverse();
    output.write_all(&buf).await?;
    Ok(())
}
