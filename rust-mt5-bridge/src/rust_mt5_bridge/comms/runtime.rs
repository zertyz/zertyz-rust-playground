//! Concept borrowed from the kickass-app-template

use std::pin::Pin;
use std::sync::Arc;
use reactive_messaging::SocketClient;


pub struct Runtime {
    pub tokio_runtime: Option<Arc<tokio::runtime::Runtime>>,
    pub socket_server: Option<SocketClient>,
}