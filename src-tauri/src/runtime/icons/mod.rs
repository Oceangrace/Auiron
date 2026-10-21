//! Background icon downloads and the read-only WebView image protocol.

mod icon_protocol;
mod icon_service;

pub(crate) use icon_protocol::IconProtocol;
pub(crate) use icon_service::IconService;
