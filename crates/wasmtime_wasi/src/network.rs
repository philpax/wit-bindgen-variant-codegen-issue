use crate::{wasi_default_network, wasi_network, WasiCtx};
use crate::{
    //wasi_network::{IpSocketAddress, Ipv4SocketAddress, Ipv6SocketAddress},
    wasi_tcp,
    wasi_udp,
};
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

impl wasi_network::Host for WasiCtx {
    fn drop_network(&mut self, _network: wasi_network::Network) -> anyhow::Result<()> {
        todo!()
    }
}

impl wasi_default_network::Host for WasiCtx {
    fn default_network(&mut self) -> anyhow::Result<wasi_network::Network> {
        todo!()
    }
}

impl From<SocketAddr> for wasi_tcp::IpSocketAddress {
    fn from(addr: SocketAddr) -> Self {
        match addr {
            SocketAddr::V4(v4) => Self::Ipv4(v4.into()),
            SocketAddr::V6(v6) => Self::Ipv6(v6.into()),
        }
    }
}

impl From<SocketAddr> for wasi_udp::IpSocketAddress {
    fn from(addr: SocketAddr) -> Self {
        match addr {
            SocketAddr::V4(v4) => Self::Ipv4(v4.into()),
            SocketAddr::V6(v6) => Self::Ipv6(v6.into()),
        }
    }
}

impl From<SocketAddrV4> for wasi_tcp::Ipv4SocketAddress {
    fn from(addr: SocketAddrV4) -> Self {
        Self {
            address: MyIpv4Addr::from(addr.ip()).0,
            port: addr.port(),
        }
    }
}

impl From<SocketAddrV4> for wasi_udp::Ipv4SocketAddress {
    fn from(addr: SocketAddrV4) -> Self {
        Self {
            address: MyIpv4Addr::from(addr.ip()).0,
            port: addr.port(),
        }
    }
}

impl From<SocketAddrV6> for wasi_tcp::Ipv6SocketAddress {
    fn from(addr: SocketAddrV6) -> Self {
        Self {
            address: MyIpv6Addr::from(addr.ip()).0,
            port: addr.port(),
            flow_info: addr.flowinfo(),
            scope_id: addr.scope_id(),
        }
    }
}

impl From<SocketAddrV6> for wasi_udp::Ipv6SocketAddress {
    fn from(addr: SocketAddrV6) -> Self {
        Self {
            address: MyIpv6Addr::from(addr.ip()).0,
            port: addr.port(),
            flow_info: addr.flowinfo(),
            scope_id: addr.scope_id(),
        }
    }
}

// Newtypes to guide conversions.
struct MyIpv4Addr((u8, u8, u8, u8));
struct MyIpv6Addr((u16, u16, u16, u16, u16, u16, u16, u16));

impl From<&Ipv4Addr> for MyIpv4Addr {
    fn from(addr: &Ipv4Addr) -> Self {
        let octets = addr.octets();
        Self((octets[0], octets[1], octets[2], octets[3]))
    }
}

impl From<&Ipv6Addr> for MyIpv6Addr {
    fn from(addr: &Ipv6Addr) -> Self {
        let segments = addr.segments();
        Self((
            segments[0],
            segments[1],
            segments[2],
            segments[3],
            segments[4],
            segments[5],
            segments[6],
            segments[7],
        ))
    }
}
