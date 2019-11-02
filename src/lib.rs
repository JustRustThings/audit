#![cfg_attr(feature = "cargo-clippy", allow(module_inception))]

use failure;

use netlink_proto::{sys::Protocol, Connection};

mod handle;
pub use crate::handle::*;

mod errors;
pub use crate::errors::*;

use futures::channel::mpsc::UnboundedReceiver;
pub use netlink_packet_audit as packet;
pub use netlink_proto::sys;

use packet::{AuditMessage, NetlinkMessage};
use std::io;
use sys::SocketAddr;

pub fn new_connection() -> io::Result<(
    Connection<AuditMessage>,
    Handle,
    UnboundedReceiver<(NetlinkMessage<AuditMessage>, SocketAddr)>,
)> {
    let (conn, handle, messages) = netlink_proto::new_connection(Protocol::Audit)?;
    Ok((conn, Handle::new(handle), messages))
}
