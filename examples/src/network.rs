use anyhow::{bail, Context, Result};
use log::{debug, info};
use socket2::{Domain, Protocol, SockAddr, Socket, Type};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, ToSocketAddrs};
use tokio::net::UdpSocket;

async fn lookup_host(host: &str) -> Result<Ipv4Addr> {
    debug!("Resolving host/group: {host}");
    match format!("{}:0", host).to_socket_addrs() {
        Ok(addrs) => {
            for addr in addrs {
                match addr.ip() {
                    IpAddr::V4(ip) => {
                        return Ok(ip);
                    }
                    IpAddr::V6(_) => continue,
                }
            }
        }
        Err(_) => {
            // TODO: Resolve hostname to ipv4 using DNS?
        }
    }
    bail!("Failed to resolve host: {host}")
}

pub async fn make_multicast_udp_socket(
    group: &str,
    port: u16,
    interface: &Option<String>,
    rcvbuf: &Option<usize>,
) -> Result<UdpSocket> {
    let group = match lookup_host(&group).await {
        Ok(group) => group,
        Err(_) => bail!("Unknown multicast group: {group}")
    };
    if !group.is_multicast() {
        bail!("Group '{group}' is not a multicast address");
    }
    let interface = match interface.as_ref() {
        Some(addr) => lookup_host(addr).await?,
        None => Ipv4Addr::UNSPECIFIED,
    };

    let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP)).context("Socket::new()")?;
    socket.set_reuse_address(true).context("socket.set_reuse_address()")?;
    if let Some(rcvbuf) = rcvbuf.as_ref() {
        socket.set_recv_buffer_size(*rcvbuf).context("socket.set_recv_buffer_size()")?;
    }

    let addr_port = SocketAddr::new(group.into(), port);
    info!("Joining multicast group: {addr_port} on interface: {interface}");
    socket.bind(&SockAddr::from(addr_port)).context("socket::bind()")?;
    socket.set_nonblocking(true).context("socket.set_nonblocking()")?;
    socket.join_multicast_v4(&group, &interface).context("socket.join_multicast()")?;

    // convert socket2::Socket to tokio::net::UdpSocket
    let socket = UdpSocket::from_std(socket.into()).context("UdpSocket::from_std()")?;
    Ok(socket)
}
