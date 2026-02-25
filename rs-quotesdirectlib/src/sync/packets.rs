//! Packet formats used by Quotes Direct API
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
#![allow(clippy::cast_possible_truncation)]

use std::io::{Read, Write};

use crate::{Error, Result};

/// UDP packet reader
///
/// # Examples
///
/// ```rust,ignore
/// use quotesdirectlib::packets::UDPPacket;
///
/// let datagram = vec![...];
/// let packet = UDPPacket::read(&datagram)?;
/// ```
///
#[derive(Debug)]
pub struct UDPPacket<'a> {
    pub seq_num: u32,
    pub sub_channel: u8,
    pub payload: &'a [u8],
}

impl<'a> UDPPacket<'a> {
    /// Read an `UDPPacket` from stream.
    /// # Errors
    /// Returns an error if the input stream cannot be read.
    pub fn read(buffer: &'a [u8]) -> Result<UDPPacket<'a>> {
        if buffer.len() < 5 {
            return Err(Error::InvalidPacketLength(buffer.len() as u64));
        }
        Ok(UDPPacket {
            seq_num: (u32::from(buffer[0])) << 24
                | (u32::from(buffer[1])) << 16
                | (u32::from(buffer[2])) << 8
                | (u32::from(buffer[3])),
            sub_channel: buffer[4],
            payload: &buffer[5..],
        })
    }
}

/// TCP packet reader and writer
///
/// # Examples
///
/// ```rust,ignore
/// use std::io::BufReader;
/// use quotesdirectlib::sync::packets::TCPPacket;
/// use std::net::TcpStream;
///
/// let mut stream = BufReader::new(TcpStream::connect("127.0.0.1:2345")?);
/// let packet = TCPPacket::read(&mut stream)?;
/// ```
///
#[derive(Debug)]
pub struct TCPPacket {
    pub seq_num: u32,
    pub sub_channel: u8,
    pub payload: Vec<u8>,
}

impl TCPPacket {
    /// Read a `TCPPacket` from stream.
    /// # Errors
    /// Returns an error if the input stream cannot be read.
    pub fn read(input: &mut dyn Read) -> Result<Option<TCPPacket>> {
        // read length
        let len = match read_var_uint(input)? {
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
        input.read_exact(&mut buffer)?;
        // read payload
        let mut payload = Vec::with_capacity(len as usize);
        #[allow(clippy::uninit_vec)]
        unsafe {
            payload.set_len(len as usize);
        }
        input.read_exact(&mut payload)?;

        Ok(Some(TCPPacket {
            seq_num: (u32::from(buffer[0])) << 24
                | (u32::from(buffer[1])) << 16
                | (u32::from(buffer[2])) << 8
                | (u32::from(buffer[3])),
            sub_channel: buffer[4],
            payload,
        }))
    }

    /// Write a packet to the output stream.
    /// # Errors
    /// Returns an error if the output stream cannot be written.
    pub fn write(self, output: &mut dyn Write) -> Result<()> {
        // write length
        let len = 5 + (self.payload.len() as u64);
        write_var_uint(output, len)?;
        // write seq_num + sub_channel
        output.write_all(&[
            (self.seq_num >> 24) as u8,
            (self.seq_num >> 16) as u8,
            (self.seq_num >> 8) as u8,
            self.seq_num as u8,
            self.sub_channel,
        ])?;
        // write payload
        output.write_all(&self.payload)?;
        Ok(())
    }
}

fn read_var_uint(input: &mut dyn Read) -> Result<Option<u64>> {
    let mut value: u64 = 0;

    let mut buffer = [0; 1];
    if input.read_exact(&mut buffer).is_err() {
        // failed reading the first byte means we reached end of file
        return Ok(None);
    }

    loop {
        let byte = buffer[0];
        value <<= 7;
        value |= u64::from(byte & 0x7f);
        if byte & 0x80 == 0x80 {
            return Ok(Some(value));
        }
        input.read_exact(&mut buffer)?;
    }
}

fn write_var_uint(output: &mut dyn Write, mut value: u64) -> Result<()> {
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
    output.write_all(&buf)?;
    Ok(())
}
