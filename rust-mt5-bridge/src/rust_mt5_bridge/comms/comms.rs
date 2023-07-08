//! Some methods to control our internal server...

use super::{
    external_connector_processor::ServerProtocolProcessor,
    messages_model::OgreExchangeMessagesForExternalConnectors,
    runtime::Runtime,
};
use std::{
    thread::{self, JoinHandle},
    sync::Arc,
    future::{self, Future},
    time::Duration,
};
use std::pin::Pin;
use reactive_messaging::prelude::ProcessorRemoteStreamType;
use futures::TryFutureExt;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;
use log::{debug,warn,error};
use reactive_messaging::SocketClient;


const SERVER_IP: &str = "127.0.0.1";
const PORT:      u16  = 9758;


pub fn start_external_connector_server() {
    start_tokio(async_main);

    async fn fallible_async_main() -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
        let server_processor_ref1 = Arc::new(ServerProtocolProcessor::new());
        let server_processor_ref2 = Arc::clone(&server_processor_ref1);

        warn!("ExternalConnector: starting as a server listening at {SERVER_IP}:{PORT}");
warn!("################ 4) is RUNTIME's Mutex unlocked? {}!", RUNTIME.try_write().is_ok());

        let socket_server = SocketClient::spawn_responsive_processor(SERVER_IP.to_string(), PORT,
            move |connection_events| {
                server_processor_ref1.server_events_callback(connection_events);
                future::ready(())
            },
            move |client_addr, port, peer, client_messages_stream: ProcessorRemoteStreamType<OgreExchangeMessagesForExternalConnectors>| {
                warn!("another ExternalConnector processor started!");
                server_processor_ref2.dialog_processor(client_addr, port, peer, client_messages_stream)
            }
        ).await?;
        RUNTIME.write().await.socket_server.replace(socket_server);
        warn!("################ socket_server is in RUNTIME -- is the mutex unlocked for writing? {}", RUNTIME.try_write().is_ok());
warn!("################ socket_server RETURNED from starting");

        // TODO tokio never ends in this DLL
        loop {
            tokio::time::sleep(Duration::from_secs(60)).await;
        }
    }

    async fn async_main() {
        if let Err(err) = fallible_async_main().await {
            error!("ExternalConnector Server exited with error: {}", err);
        }
    }
}

pub fn shutdown_external_connector_server() {
    warn!("ExternalConnector: shutting down server at {SERVER_IP}:{PORT}");
    // RUNTIME.blocking_write().socket_server.take().expect("SocketServer seems to not have been started")
    //     .unpin().shutdown().expect("FAILED TO SHUTDOWN");
    std::thread::sleep(Duration::from_secs(1));
    warn!("ExternalConnector: shutdown signal sent");
}

// the Runtime with state information for our services
const RUNTIME: Lazy<RwLock<Runtime>> = Lazy::new(|| RwLock::new(Runtime {
    tokio_runtime: None,
    socket_server: None,

}));

/// code taken from the kickass-app-template
fn start_tokio<AsyncMainFutureType: Future<Output=()>>
              (async_main: impl Fn() -> AsyncMainFutureType + Send + 'static) -> JoinHandle<()> {
    thread::spawn(move || {
        std::thread::sleep(Duration::from_secs(5));
        debug!("  about to start the Tokio runtime with all available CPUs as worker threads...");
        let mut tokio_runner = tokio::runtime::Builder::new_multi_thread();
        let tokio_runtime = Arc::new(tokio_runner
            .thread_stack_size(2 * 1024 * 1024)
            .enable_all()
            .build()
            .unwrap());
warn!("################ 1) is RUNTIME's Mutex unlocked? {}!", RUNTIME.try_write().is_ok());
warn!("################ 2) is RUNTIME's Mutex unlocked? {}!", RUNTIME.try_write().is_ok());
        RUNTIME.blocking_write().tokio_runtime = Some(Arc::clone(&tokio_runtime));
warn!("################ 3) is RUNTIME's Mutex unlocked? {}!", RUNTIME.try_write().is_ok());
        tokio_runtime
            .block_on(async_main())
    })
}
