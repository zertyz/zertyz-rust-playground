//! Provides communications (network, IPC) for the Ogre-Exchange

mod comms;
pub use comms::*;

mod runtime;
mod messages_model;
mod external_connector_processor;
// mod server_logic;
