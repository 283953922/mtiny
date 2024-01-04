
use std::{net::SocketAddr, fmt};

#[cfg(feature="actix")]
mod actix;

#[cfg(feature = "actix")]
pub use actix::Server;

#[derive(Clone, Copy,PartialEq, PartialOrd,Eq, Ord,Hash)]
pub struct PeerAddr(pub SocketAddr);


impl std::fmt::Debug for PeerAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <SocketAddr as std::fmt::Debug>::fmt(&self.0, f)
    }
}

impl std::fmt::Display for PeerAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <SocketAddr as std::fmt::Display>::fmt(&self.0,f)
    }
}