//! Defines the `reactive-messaging` models for inter-process communications

use std::error::Error;
use reactive_messaging::{ron_deserializer, ron_serializer, SocketServerDeserializer, SocketServerSerializer};
use super::super::ogre_exchange_models::*;
use serde::{Serialize, Deserialize};


/// Here so history, possibly... dates while in alpha, semantic versioning from there on
pub const PROTOCOL_VERSION: &str = "2023-07-04";


/// Messages sent by agents integrated on external Exchanges or Brokers, sharing information
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ExternalConnectorMessages {

    /// Upon connection, this is the first message "External Connectors" must send -- in order for the `OgreExchange` server to classify their hole in the system
    /// and determine if logging in is necessary
    ConnectorIdentification(ConnectorIdentification),

    /// If the `OgreExchange` server sent [OgreExchangeMessagesForExternalConnectors::ProvideAuthorizationToContinue], this "External Connector" must send an
    /// authorization token to continue -- possibly a double factor challenge sent by Telegram
    UserAuthorization(String),

    /// Asks the `OgreExchange` server to return, as soon as possible, the given number + 1 with a [OgreExchangeMessagesForExternalConnectors::KeepAliveAnswer]
    /// -- may be used to measure round trip times, as well as to check the peer's liveliness
    KeepAliveRequest(u32),

    /// Similar to [ExternalConnectorMessages::KeepAliveRequest], but sent in response to [OgreExchangeMessagesForExternalConnectors::KeepAliveRequest]
    KeepAliveAnswer(u32),

    /// The "External Connector" sends this message when there is Market Data updates to share regarding negotiable symbols
    MarketData(ExternalConnectorMarketData),

    /// A [ConnectorIdentification::FullAdvisor] "External Connector" informs the `OgreExchange` server that one of his orders was executed
    /// -- an independent [ExternalConnectorMessages::Trade] notification (for the same event) is likely to be also reported
    ExecutedOrder {
        /// in the form YYYYMMDD
        date: u32,
        /// in the form HHMMSSMMM
        time: u32,
        /// the symbol, for double checking -- must match the one stated when identifying the client
        symbol: String,
        /// the unitary paper currency value multiplied by 1000 -- or cent value multiplied by 10
        unitary_mill_value: u32,
        /// how many papers of that symbol were traded
        quantity: u32,
        /// if true, means the order is not totally filled -- it remains active within the Exchange
        /// until it is fully executed
        partial: bool,
        /// the order ID, as issued by the Exchange
        order_id: u32,
        /// the order ID, if added by the server
        ogre_id: u32,
    },

    /// A [ConnectorIdentification::FullAdvisor] client informs the server that his previous scheduled order
    /// has been cancelled
    CancelledOrder(ConnectorIdentificationOrderCancellationReasons),

    /// Client informs of an order scheduled for execution (but still not executed)
    /// -- either because the user just manually added it
    /// or because the server requested a list of orders awaiting execution
    /// TODO this should be an array -- so the server may know of orders it lost track cancelling
    PendingOrder {
        ogre_id:     u32,
        exchange_id: u32,
        // order_data...
    },

    /// A [ConnectorIdentification::FullAdvisor] client asks for any new drawable events (after `sequential`) to be sent back
    ChartPoints { sequential: u32 },

    /// Common messages to all protocols
    /// ////////////////////////////////

    /// If the processor answers with this message, nothing will be sent back to the peer
    NoAnswer,

    /// Whenever a message is not understood, this will be answered, along with the received message
    UnknownMessage(String),

    /// If the processor results in `Err`, this will be sent along with the error description
    ProcessorError(String),

    /// Tells the server we are disconnecting on purpose -- and that it should not expect to see this client trying to reconnect.\
    /// If the disconnection has been done in the middle of a transaction, the server should cancel them (possibly also issuing warnings & alarms).\
    /// The `String` param contains a textual explanation for the disconnection reason.
    GoodBye(String),

}

/// Response/reactions/inquiries the `OgreExchange` may re-act or pro-act when interacting with "External Connectors"
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum OgreExchangeMessagesForExternalConnectors {

    /// `OgreExchange` server greeting, sent as soon as the connection with the client is accepted.\
    /// From here, client must send [ExternalConnectorMessages::ConnectorIdentification]
    Welcome,

    /// Depending on the [ConnectorIdentification], the `OgreExchange` server may require [UserAuthorization] to continue.\
    /// Upon receiving this, the "External Connector" must answer with [ConnectorIdentification::UserAuthorization]
    ProvideAuthorizationToContinue,

    /// If something goes unexpected, the `OgreExchange` server may decide to drop the "External Connector"
    /// -- wrong/missing login, wrong protocol, server being shutdown, ...
    Disconnected(DisconnectionReason),

    /// Asks the "External Connector" to return, as soon as possible, the given number + 1 with a [ExternalConnectorMessages::KeepAliveAnswer]
    /// -- may be used to measure round trip times, as well as to check the peer's liveliness.\
    /// If a [ConnectorIdentification::MarketDataBridge] times out responding, its asset's symbol is
    /// marked as `stale` and precautions are taken to minimize risks -- no new buying orders (cancelling un-executed ones),
    /// sound an alarm if there are open positions, sell all open positions at market price, etc.
    KeepAliveRequest(u32),

    /// Similar to [OgreExchangeMessagesForExternalConnectors::KeepAliveRequest], but sent in response to
    /// [ExternalConnectorMessages::KeepAliveRequest]
    KeepAliveAnswer(u32),

    /// Asks the "External Connector" to execute an order within the "External Exchange".\
    /// The "External Connector" is expected not to answer to this message if all is fine,
    /// however, [ExternalConnectorMessages::CancelledOrder] may happen if scheduling
    /// was not possible.
    ScheduleOrder(OrderCommand),

    /// Asks the "External Connector" to cancel an order within the "External Exchange"
    CancelOrder {
        ogre_id: u32,
        reason: OrderCancellationReasons,
    },

    /// Asks the "External Connector" to report all its scheduled orders -- either added by `OgreExchange` requests or not
    StateOpenPositions,

    /// `OgreExchange` server informs drawable events, as asked by [ExternalConnectorMessages::ChartPoints]
    ChartPoints {

    },

    /// Common messages to all protocols
    /// ////////////////////////////////

    /// If the processor answers with this message, nothing will be sent back to the peer
    NoAnswer,

    /// Whenever a message is not understood, this will be answered, along with the received message
    UnknownMessage(String),

    /// If the processor results in `Err`, this will be sent along with the error description
    ProcessorError(String),

    /// Server sends this to connected clients once it has decided it is time to quit
    ShuttingDown,
}


// Implementations of the RON SerDe
///////////////////////////////////

// TODO 2023-07-04: rename "SocketServer" Serialize/Deserializer to "ReactiveMessaging*"
impl SocketServerSerializer<ExternalConnectorMessages> for ExternalConnectorMessages {

    #[inline(always)]
    fn serialize(remote_message: &ExternalConnectorMessages, buffer: &mut Vec<u8>) {
        ron_serializer(remote_message, buffer)
            .expect("`ron_serializer()` for `ExternalConnectorMessages`");
    }

    #[inline(always)]
    fn processor_error_message(err: String) -> ExternalConnectorMessages {
        ExternalConnectorMessages::ProcessorError(err)
    }

    #[inline(always)]
    fn is_disconnect_message(processor_answer: &ExternalConnectorMessages) -> bool {
        matches!(processor_answer, ExternalConnectorMessages::GoodBye(_))
    }

    #[inline(always)]
    fn is_no_answer_message(processor_answer: &ExternalConnectorMessages) -> bool {
        matches!(processor_answer, ExternalConnectorMessages::NoAnswer)
    }
}

impl SocketServerDeserializer<ExternalConnectorMessages> for ExternalConnectorMessages {
    #[inline(always)]
    fn deserialize(local_message: &[u8]) -> Result<ExternalConnectorMessages, Box<dyn Error + Sync + Send>> {
        ron_deserializer(local_message)
    }
}

impl SocketServerSerializer<OgreExchangeMessagesForExternalConnectors> for OgreExchangeMessagesForExternalConnectors {

    #[inline(always)]
    fn serialize(remote_message: &OgreExchangeMessagesForExternalConnectors, buffer: &mut Vec<u8>) {
        ron_serializer(remote_message, buffer)
            .expect("`ron_serializer()` for `OgreExchangeMessagesForExternalConnectors`");
    }

    #[inline(always)]
    fn processor_error_message(err: String) -> OgreExchangeMessagesForExternalConnectors {
        OgreExchangeMessagesForExternalConnectors::ProcessorError(err)
    }

    #[inline(always)]
    fn is_disconnect_message(processor_answer: &OgreExchangeMessagesForExternalConnectors) -> bool {
        matches!(processor_answer, OgreExchangeMessagesForExternalConnectors::ShuttingDown)
    }

    #[inline(always)]
    fn is_no_answer_message(processor_answer: &OgreExchangeMessagesForExternalConnectors) -> bool {
        matches!(processor_answer, OgreExchangeMessagesForExternalConnectors::NoAnswer)
    }
}

impl SocketServerDeserializer<OgreExchangeMessagesForExternalConnectors> for OgreExchangeMessagesForExternalConnectors {
    #[inline(always)]
    fn deserialize(local_message: &[u8]) -> Result<OgreExchangeMessagesForExternalConnectors, Box<dyn Error + Sync + Send>> {
        ron_deserializer(local_message)
    }
}


/// Market data, as informed by the client -- with easy to generate info (when compared to the
/// heavily optimized internal versions of the same data that we use).\
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ExternalConnectorMarketData {

    /// Sent by a [ClientIdentification::MarketDataBridge] client to update the server with the symbol information
    /// -- updated after connection, sporadically or when an information change happens.\
    /// See [SymbolState] for the internal version of the same info, after being treated
    SymbolState {
        symbol: String,
        in_auction: bool,
    },

    /// Client informs of a book event -- either a new entry or an update
    /// -- see [SingleBook] for the internal version of the same info, after being treated
    Book {
        /// in the form YYYYMMDD
        date: u32,
        /// in the form HHMMSSMMM
        time: u32,
        /// the symbol, for double checking -- must match the one stated when identifying the client
        symbol: String,
        /// the price
        price_level_mills: u32,
        /// the number of orders waiting
        n_orders: u32,
        /// the total quantity of booked orders
        available_quantity: u32,
        /// the operation those orders want to make
        side: Parties,
    },

    /// a [ClientIdentification::MarketDataBridge] or even [ClientIdentification::FullAdvisor] informs the server that an external trade happened
    /// -- see [SingleTrade] for the internal version of the same info, after being treated
    Trade {
        /// in the form YYYYMMDD
        date: u32,
        /// in the form HHMMSSMMM
        time: u32,
        /// the symbol, for double checking -- must match the one stated when identifying the client
        symbol: String,
        /// the unitary paper currency value multiplied by 1000 -- or cent value multiplied by 10
        unitary_mill_value: u32,
        /// how many papers of that symbol were traded
        quantity: u32,
        /// who emitted the Market Order?
        aggressor: Parties,
    },
}
impl Into<MarketData> for ExternalConnectorMarketData {
    fn into(self) -> MarketData {
        match self {
            ExternalConnectorMarketData::SymbolState { symbol, in_auction } => MarketData::SymbolState(SymbolState { symbol, in_auction }),
            ExternalConnectorMarketData::Book { .. } => todo!("Please, go ahead and develop the conversion from ExternalConnector Book Into SingleBook"),
            ExternalConnectorMarketData::Trade { .. } => todo!("Please, go ahead and develop the conversion from ExternalConnector Trade Into SingleTrade"),
        }
    }
}

/// Reasons for the "External Connector" to have aborted executing one of orders the `OgreExchange` had scheduled for execution
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ConnectorIdentificationOrderCancellationReasons {
    /// The user -- a human operator -- had, for some reason, manually cancelled the order
    UserInitiated { order_id: u32, message: String },
    /// The broker didn't respond in time if the order was accepted or not.\
    /// Real status is unknown and the server should inquire about the pending orders and either
    /// cancel it or schedule it again.
    TimeoutWhileScheduling { order_id: u32, elapsed_nanos: u32 },
    /// The broker didn't accept the order
    BrokerInitiated { order_id: u32, message: String },
    /// The Exchange didn't accept the order, after all
    ExchangeInitiated { order_id: u32, message: String },
}
impl Into<OrderCancellationReasons> for ConnectorIdentificationOrderCancellationReasons {
    fn into(self) -> OrderCancellationReasons {
        match self {
            ConnectorIdentificationOrderCancellationReasons::UserInitiated { .. } => {todo!("todo...")}
            ConnectorIdentificationOrderCancellationReasons::TimeoutWhileScheduling { .. } => {todo!("todo...")}
            ConnectorIdentificationOrderCancellationReasons::BrokerInitiated { .. } => {todo!("todo...")}
            ConnectorIdentificationOrderCancellationReasons::ExchangeInitiated { .. } => {todo!("todo...")}
        }
    }
}


/// Unit tests the [protocol](self) module
#[cfg(any(test,doc))]
mod tests {
    use super::*;


    /// assures serialization / deserialization works for all external connector messages
    #[cfg_attr(not(doc),test)]
    fn serde_for_external_connector_messages() {
        // keep this in sync with all available `ExternalConnectorMessages` variants, in the order they are declared there
        let external_connector_messages = vec![
            ExternalConnectorMessages::ConnectorIdentification(ConnectorIdentification::FullAdvisor      { version: format!("v.1.2.3"), symbol: format!("PETR3"), account_token: format!("AkD9jH7BcgH68Js7") }),
            ExternalConnectorMessages::ConnectorIdentification(ConnectorIdentification::MarketDataBridge { version: format!("v.1.2.3"), symbol: format!("PETR3"), account_token: format!("AkD9jH7BcgH68Js7") }),
            ExternalConnectorMessages::ConnectorIdentification(ConnectorIdentification::WatcherAdvisor   { version: format!("v.1.2.3"), symbol: format!("PETR3"), account_token: format!("AkD9jH7BcgH68Js7") }),
            ExternalConnectorMessages::UserAuthorization(format!("PaSsD321")),
            ExternalConnectorMessages::KeepAliveRequest(1),
            ExternalConnectorMessages::KeepAliveAnswer(2),
            ExternalConnectorMessages::MarketData(ExternalConnectorMarketData::SymbolState { symbol: format!("PETR3"), in_auction: false }),
            ExternalConnectorMessages::MarketData(ExternalConnectorMarketData::Book        { date: 22011979, time: 213214001, symbol: format!("PETR3"), price_level_mills: 32120, n_orders: 100, available_quantity: 1000, side: Parties::Buyer }),
            ExternalConnectorMessages::MarketData(ExternalConnectorMarketData::Trade       { date: 22011979, time: 213214001, symbol: format!("PETR3"), unitary_mill_value: 32120, quantity: 100, aggressor: Parties::Buyer }),
            ExternalConnectorMessages::ExecutedOrder { date: 22011979, time: 213214001, symbol: format!("PETR3"), unitary_mill_value: 32120, quantity: 100, partial: false, order_id: 1, ogre_id: 1 },
            ExternalConnectorMessages::CancelledOrder(ConnectorIdentificationOrderCancellationReasons::UserInitiated          { order_id: 1, message: format!("MT5 was closed") } ),
            ExternalConnectorMessages::CancelledOrder(ConnectorIdentificationOrderCancellationReasons::TimeoutWhileScheduling { order_id: 2, elapsed_nanos: 1234567890 } ),
            ExternalConnectorMessages::CancelledOrder(ConnectorIdentificationOrderCancellationReasons::BrokerInitiated        { order_id: 3, message: format!("You didn't provide enough warranties for that operation") } ),
            ExternalConnectorMessages::PendingOrder { ogre_id: 1, exchange_id: 1 },
            ExternalConnectorMessages::ChartPoints { sequential: 1 },
            ExternalConnectorMessages::GoodBye(format!("done for today! the sea has awesome waves! time for body surfing!")),
            ExternalConnectorMessages::UnknownMessage(format!("Not sure where this is used...")),
        ];
        let mut serializer_buffer = vec![];
        for message in external_connector_messages {
            <ExternalConnectorMessages as SocketServerSerializer<ExternalConnectorMessages>>::serialize(&message, &mut serializer_buffer);
            let serialized = String::from_utf8_lossy(&serializer_buffer);
            let reconstructed = <ExternalConnectorMessages as SocketServerDeserializer<ExternalConnectorMessages>>::deserialize(serialized.as_bytes())
                .expect(&format!("deserialization failed for input '{}'", serialized));
            assert_eq!(reconstructed, message, "an 'External Connector' message couldn't resist serde. It was serialized to '{}'", serialized);
            println!("✓ {}", serialized.trim_end());
        }
    }

    /// assures serialization / deserialization works for all server messages
    #[cfg_attr(not(doc),test)]
    fn serde_for_server_messages() {
        // keep this in sync with all available `OgreExchangeMessagesForExternalConnectors` variants, in the order they are declared there
        let ogre_exchange_messages = vec![
            OgreExchangeMessagesForExternalConnectors::Welcome,
            OgreExchangeMessagesForExternalConnectors::ProvideAuthorizationToContinue,
            OgreExchangeMessagesForExternalConnectors::Disconnected(DisconnectionReason::UnknownConnectorType),
            OgreExchangeMessagesForExternalConnectors::Disconnected(DisconnectionReason::DeprecatedConnectorVersion {minimum_accepted_version: format!("v.1.2.3")}),
            OgreExchangeMessagesForExternalConnectors::Disconnected(DisconnectionReason::UnknownAccount),
            OgreExchangeMessagesForExternalConnectors::Disconnected(DisconnectionReason::AccountFrozen           {message: format!("too many wrong authorization attempts"), remaining_duration_nanos: 1234567890}),
            OgreExchangeMessagesForExternalConnectors::Disconnected(DisconnectionReason::AccountDisabled         {message: format!("to enable it back, ask Luiz for a new password")}),
            OgreExchangeMessagesForExternalConnectors::Disconnected(DisconnectionReason::AuthenticationFailure),
            OgreExchangeMessagesForExternalConnectors::Disconnected(DisconnectionReason::ProtocolOffense         {message: format!("for instance... trying to provide two client identifications...")}),
            OgreExchangeMessagesForExternalConnectors::Disconnected(DisconnectionReason::Reconnected             {ip:      format!("127.0.0.1")}),
            OgreExchangeMessagesForExternalConnectors::Disconnected(DisconnectionReason::ClientInitiated),
            OgreExchangeMessagesForExternalConnectors::Disconnected(DisconnectionReason::RiskManager(RiskManagementConnectionDroppingConditions::RoundTripTimeTooHigh { nanos: 1234 })),
            OgreExchangeMessagesForExternalConnectors::Disconnected(DisconnectionReason::RiskManager(RiskManagementConnectionDroppingConditions::PingTimeout)),
            OgreExchangeMessagesForExternalConnectors::Disconnected(DisconnectionReason::RiskManager(RiskManagementConnectionDroppingConditions::ClockSkewTooHigh { estimated_delta_nanos: 1234 })),
            OgreExchangeMessagesForExternalConnectors::KeepAliveRequest(1),
            OgreExchangeMessagesForExternalConnectors::KeepAliveAnswer(2),
            OgreExchangeMessagesForExternalConnectors::ScheduleOrder(OrderCommand::Buy  (Order { ogre_id: 1, aggressor: Parties::Buyer,  order_type: OrderTypes::MarketOrder, date: 22011979, time: 213214001, symbol: format!("PETR3"), unitary_mill_value: 32120, quantity: 100 })),
            OgreExchangeMessagesForExternalConnectors::ScheduleOrder(OrderCommand::Sell (Order { ogre_id: 1, aggressor: Parties::Seller, order_type: OrderTypes::MarketOrder, date: 22011979, time: 213214001, symbol: format!("PETR3"), unitary_mill_value: 32120, quantity: 100 })),
            OgreExchangeMessagesForExternalConnectors::CancelOrder { ogre_id: 1, reason: OrderCancellationReasons::UserInitiated          { order_id: 123, message: format!("Slipt into that button...") } },
            OgreExchangeMessagesForExternalConnectors::CancelOrder { ogre_id: 1, reason: OrderCancellationReasons::TimeoutWhileScheduling { order_id: 123, elapsed_nanos: 1234567890 } },
            OgreExchangeMessagesForExternalConnectors::StateOpenPositions,
            OgreExchangeMessagesForExternalConnectors::ChartPoints {},
            OgreExchangeMessagesForExternalConnectors::NoAnswer,
            OgreExchangeMessagesForExternalConnectors::UnknownMessage(format!("Client, you've sent something I don't understand")),
            OgreExchangeMessagesForExternalConnectors::ProcessorError(format!("Client, something went wrong when I was calculating your answer... efforts dropped")),
            OgreExchangeMessagesForExternalConnectors::ShuttingDown,
        ];
        let mut serializer_buffer = vec![];
        for message in ogre_exchange_messages {
            <OgreExchangeMessagesForExternalConnectors as SocketServerSerializer<OgreExchangeMessagesForExternalConnectors>>::serialize(&message, &mut serializer_buffer);
            let serialized = String::from_utf8_lossy(&serializer_buffer);
            let reconstructed = <OgreExchangeMessagesForExternalConnectors  as SocketServerDeserializer<OgreExchangeMessagesForExternalConnectors>>::deserialize(serialized.as_bytes())
                .expect(&format!("deserialization failed for input '{}'", serialized));
            assert_eq!(reconstructed, message, "an 'OgreExchange for ExternalConnectors' message couldn't resist serde. It was serialized to '{}'", serialized);
            println!("✓ {}", serialized.trim_end());
        }
    }

}