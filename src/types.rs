#[cfg(unix)]
use libc as c;

/// Both libc and winapi define c_int as i32 `type c_int = i32;`
#[allow(non_camel_case_types)]
type c_int = i32;

/*
#[cfg(windows)]
use winapi::shared::ws2def as c;
*/

#[cfg(windows)]
use windows_sys::Win32::Networking::WinSock as c;

/// Socket Type
///
/// Cross platform enum of common Socket Types. For missing types use
/// the `libc` and `winapi` crates, depending on platform.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SockType {
    /// Sequenced, reliable, connection-based byte streams.
    Stream,
    /// Connectionless, unreliable datagrams of fixed max length.
    DGram,
    /// Raw protocol interface.
    #[cfg(not(target_os = "redox"))]
    Raw,
    /// Reliably-delivered messages.
    #[cfg(not(target_os = "redox"))]
    RDM,
}

/// |             | libc | winapi | windows_sys |
/// |-------------|------|--------|-------------|
/// | SOCK_STREAM | 1i32 | 1i32   | 1u16        |
/// | SOCK_DGRAM  | 2i32 | 2i32   | 2u16        |
/// | SOCK_RAW    | 3i32 | 3i32   | 3u16        |
/// | SOCK_RDM    | 4i32 | 4i32   | 4u16        |
impl From<SockType> for c_int {
    fn from(sock: SockType) -> c_int {
        match sock {
            SockType::Stream => c::SOCK_STREAM as i32,
            SockType::DGram => c::SOCK_DGRAM as i32,
            #[cfg(not(target_os = "redox"))]
            SockType::Raw => c::SOCK_RAW as i32,
            #[cfg(not(target_os = "redox"))]
            SockType::RDM => c::SOCK_RDM as i32,
        }
    }
}

impl PartialEq<c_int> for SockType {
    fn eq(&self, other: &c_int) -> bool {
        let int: c_int = (*self).into();
        *other == int
    }
}

impl PartialEq<SockType> for c_int {
    fn eq(&self, other: &SockType) -> bool {
        let int: c_int = (*other).into();
        *self == int
    }
}

/// Socket Protocol
///
/// Cross platform enum of common Socket Protocols. For missing types use
/// the `libc` and `winapi` crates, depending on platform.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Protocol {
    /// Internet Control Message Protocol.
    ICMP,
    /// Transmission Control Protocol.
    TCP,
    /// User Datagram Protocol.
    UDP,
}

/// |              | libc  | winapi | windows_sys |
/// |--------------|-------|--------|-------------|
/// | IPPROTO_ICMP | 1i32  | 1u32   | 1i32        |
/// | IPPROTO_TCP  | 6i32  | 6u32   | 6i32        |
/// | IPPROTO_UDP  | 17i32 | 17u32  | 17i32       |
impl From<Protocol> for c_int {
    fn from(sock: Protocol) -> c_int {
        match sock {
            Protocol::ICMP => c::IPPROTO_ICMP,
            Protocol::TCP => c::IPPROTO_TCP,
            Protocol::UDP => c::IPPROTO_UDP,
        }
    }
}

impl PartialEq<c_int> for Protocol {
    fn eq(&self, other: &c_int) -> bool {
        let int: c_int = (*self).into();
        *other == int
    }
}

impl PartialEq<Protocol> for c_int {
    fn eq(&self, other: &Protocol) -> bool {
        let int: c_int = (*other).into();
        *self == int
    }
}

/// Address Family
///
/// Cross platform enum of common Address Families. For missing types use
/// the `libc` and `winapi` crates, depending on platform.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AddrFamily {
    /// Local to host (pipes and file-domain)
    Unix,
    /// IP protocol family.
    Inet,
    /// IP version 6.
    Inet6,
}
/// |          | libc  | winapi | windows_sys |
/// |----------|-------|--------|-------------|
/// | AF_UNIX  | 1i32  | 1i32   | 1u16        |
/// | AF_INET  | 2i32  | 2i32   | 2u32        |
/// | AF_INET6 | 10i32 | 23i32  | 23u32       |
impl From<AddrFamily> for c_int {
    fn from(sock: AddrFamily) -> c_int {
        match sock {
            AddrFamily::Unix => c::AF_UNIX as i32,
            AddrFamily::Inet => c::AF_INET as i32,
            AddrFamily::Inet6 => c::AF_INET6 as i32,
        }
    }
}

impl PartialEq<c_int> for AddrFamily {
    fn eq(&self, other: &c_int) -> bool {
        let int: c_int = (*self).into();
        *other == int
    }
}

impl PartialEq<AddrFamily> for c_int {
    fn eq(&self, other: &AddrFamily) -> bool {
        let int: c_int = (*other).into();
        *self == int
    }
}
