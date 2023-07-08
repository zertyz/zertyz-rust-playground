//! Handles the communications between the "External Connector" and the `OgreExchange` Server, for the External Connector's side

use super::super::{
    types::Handle,
    comms::messages_model::{ExternalConnectorMessages, OgreExchangeMessagesForExternalConnectors, PROTOCOL_VERSION},
    ogre_exchange_models::ConnectorIdentification,
    rust_mt5_bridge,
};
use std::{
    cell::UnsafeCell,
    sync::{
        Arc,
        atomic::Ordering::Relaxed,
    },
};
use reactive_messaging::prelude::{ConnectionEvent,Peer,ProcessorRemoteStreamType};
use dashmap::DashMap;
use futures::{Stream,StreamExt};
use log::{debug,info,warn};


/// Session for each connected peer
struct Session {
    symbol_handle: &'static Handle,
}
unsafe impl Send for Session {}
unsafe impl Sync for Session {}

pub struct ServerProtocolProcessor {
    sessions: Arc<DashMap<u32, Arc<Session>>>,
}

impl ServerProtocolProcessor {

    pub fn new() -> Self {
        Self {
            sessions: Arc::new(DashMap::new()),
        }
    }

    pub fn server_events_callback(&self, connection_event: ConnectionEvent<ExternalConnectorMessages>) {
        match connection_event {
            ConnectionEvent::PeerConnected { peer } => {
                debug!("Connected: {:?}", peer);
                let symbol_handle = unsafe {
                    rust_mt5_bridge::HANDLES
                        .get(peer.peer_id as usize % rust_mt5_bridge::HANDLE_COUNT.load(Relaxed) as usize)
                        .expect("BUG: HANDLER with the same PEER_ID is not registered")
                };
                debug!("Connected: {:?} -- giving it symbol '{}'", peer, symbol_handle.symbol);
                self.sessions.insert(peer.peer_id, Arc::new(Session { symbol_handle }));
            },
            ConnectionEvent::PeerDisconnected { peer, stream_stats } => {
                debug!("Disconnected: {:?} -- stats: {:?}", peer, stream_stats);
                //let _ = processor_uni.try_send(|slot| *slot = ClientMessages::Quit);
                self.sessions.remove(&peer.peer_id);
            }
            ConnectionEvent::ApplicationShutdown { timeout_ms } => {
                info!("Ping-Pong server shutdown requested. Notifying all peers within {timeout_ms}ms...");
            }
        }
    }

    pub fn dialog_processor(&self, _client_addr: String, _port: u16, peer: Arc<Peer<ExternalConnectorMessages>>, client_messages_stream: ProcessorRemoteStreamType<OgreExchangeMessagesForExternalConnectors>) -> impl Stream<Item=ExternalConnectorMessages> {

        let session = self.sessions.get(&peer.peer_id)
                                               .unwrap_or_else(|| panic!("Server BUG! Peer {:?} showed up, but we don't have a session for it! It should have been created by the `connection_events()` callback", peer))
                                               .value()
                                               .clone();     // .clone() the Arc, so we are free to move it to the the next closure (and drop it after the Stream closes)

        client_messages_stream.map(move |client_message| {

            match &*client_message {

                OgreExchangeMessagesForExternalConnectors::Welcome => {
                    ExternalConnectorMessages::ConnectorIdentification(ConnectorIdentification::FullAdvisor {
                        version: PROTOCOL_VERSION.to_string(),
                        symbol: session.symbol_handle.symbol.clone(),
                        account_token: session.symbol_handle.account_token.to_string(),
                    })
                },

                OgreExchangeMessagesForExternalConnectors::ProvideAuthorizationToContinue => {
                    panic!("I didn't expect to have to ProvideAuthorizationToContinue")
                },

                OgreExchangeMessagesForExternalConnectors::Disconnected(_) => todo!(),

                OgreExchangeMessagesForExternalConnectors::KeepAliveRequest(_) => todo!(),

                OgreExchangeMessagesForExternalConnectors::KeepAliveAnswer(_) => todo!(),

                OgreExchangeMessagesForExternalConnectors::ScheduleOrder(_) => todo!(),

                OgreExchangeMessagesForExternalConnectors::CancelOrder { .. } => todo!(),

                OgreExchangeMessagesForExternalConnectors::StateOpenPositions => todo!(),

                OgreExchangeMessagesForExternalConnectors::ChartPoints { .. } => todo!(),

                OgreExchangeMessagesForExternalConnectors::NoAnswer => todo!(),

                OgreExchangeMessagesForExternalConnectors::UnknownMessage(_) => todo!(),

                OgreExchangeMessagesForExternalConnectors::ProcessorError(_) => todo!(),

                OgreExchangeMessagesForExternalConnectors::ShuttingDown => todo!(),
            }
        })
    }
}