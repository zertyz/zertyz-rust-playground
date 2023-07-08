//! Types for integration with the `OgreExchange`
//!
//! Note: some types defined here will derive both `Serialize` & `Deserialize` to ease their usage in network communications or even IPC.

use serde::{Serialize, Deserialize};


/// Info that will uniquely identify a client's software
pub type Version = String;
/// Info that will uniquely identify an asset within an exchange
pub type Symbol = String;
/// Info that will uniquely identify a client
pub type AccountToken = String;
/// Date, through the `neat-date-time` crate, based on the epoch 1979-01-22
pub type NeatDate = u16;
/// Time, through the `neat-date-time` crate, based on the 24h representation -- ~21µs precision
pub type NeatTime = u32;
/// In currency unit, multiplied by 1000 -- or in cent, multiplied by 10
pub type MonetaryMillValue = u32;


/// Auto-declarations from the "External Connectors", affecting how the `OgreExchange` server takes their messages into account
/// and what types of commands they will have permission to use
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ConnectorIdentification {

    /// Means the "External Connector" is a Market Data Bridge: it will only report trades & other market data
    /// -- the `OgreExchange` server will take these messages with a higher weight
    MarketDataBridge { version: Version, symbol: Symbol, account_token: AccountToken },
    /// Means the "External Connector" is "OMS-able" (may execute and track orders) -- it may also
    /// provide Market Data
    FullAdvisor      { version: Version, symbol: Symbol, account_token: AccountToken },
    /// Means the "External Connector" declares itself as just a "Monitor": no orders nor any Market Data
    /// may be sent by it, but it will receive important events, for accountability and monitoring purposes
    /// -- this "External Connector" doesn't undergo any "minimum ping time enforcement" as the other members of this enum do
    WatcherAdvisor   { version: Version, symbol: Symbol, account_token: AccountToken },
}

/// Details for the disconnection -- that usually is initiated by the `OgreExchange`
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DisconnectionReason {
    UnknownConnectorType,
    DeprecatedConnectorVersion { minimum_accepted_version: Version },
    UnknownAccount,
    /// Happens if the `OgreExchange` determined it has been abused
    AccountFrozen           { message: String, remaining_duration_nanos: u32 },
    /// Happens when the the account is known to perpetrate repetitive abuses
    AccountDisabled         { message: String },
    AuthenticationFailure,
    /// Informs the peer the protocol contract was broken -- possibly due to a bug
    ProtocolOffense         { message: String },
    /// Happens -- for the old connection -- if the same [ConnectorIdentification] connected again
    Reconnected             { ip: String },
    /// Happens when the connection is gracefully dropped by the "External Connector"
    ClientInitiated,
    /// Happens when the "Risk Manager" determines the connection has to end.\
    /// This might happen if the "External Connector" was found to be experiencing high latencies or if abuses were detected
    /// -- in which case some account restrictions may be applied.
    RiskManager             (RiskManagementConnectionDroppingConditions),
}

/// Reasons why orders might be cancelled / rejected by the external Exchange or Broker
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OrderCancellationReasons {
    /// The attempt to schedule the order didn't complete within the timeout
    TimeoutWhileScheduling  { order_id: u32, elapsed_nanos: u32 },
    /// The order was cancelled by the `RiskManager`
    RiskManagerInitiated    { order_id: u32, reason: RiskManagementOrderCancellationConditions },
    /// Cancelled due to a manual intervention
    UserInitiated           { order_id: u32, message: String },
    /// Cancelled due to Broker restrictions
    BrokerInitiated         { order_id: u32, message: String },
    /// Cancelled due to Exchange restrictions
    ExchangeInitiated       { order_id: u32, message: String },
    /// Cancelled for unknown reasons
    Unspecified             { order_id: u32, message: String },
}

/// All possible scenarios in which the `RiskManager` may cancel a limit order it just approved
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RiskManagementOrderCancellationConditions {
    /// Informs an unexpected negotiation status change happened (like entering into auction)
    SymbolChangedNegotiationStatus { message: String },
    /// The (late) Risk Manager just found out the symbol is no longer suitable for exchange due to being deemed "too risky"
    SymbolChangedRisk { message: String },
}

/// All possible detectable conditions that may lead to orders (scheduled by the `DecisionMaker`) to be denied by the `RiskManager` before they are scheduled on the external Broker or Exchange.\
/// Nonetheless, even if the order is not scheduled, these events are shared with the "External Connector", so it may keep track of the internal states of the mentioned components.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RiskManagementOrderRefusalConditions {
    /// Happens when the `OgreExchange` server detects [ConnectorIdentification::MarketDataBridge] faced problems and data is untrustable
    MarketDataGap               { symbol: Symbol, start_time: u32, duration_nanos: u32 },
    /// An operation was blocked due to its negotiated financial amount being above thresholds
    AmountTooHigh               { amount_millis_limit: u32 },
    /// An operation was blocked due to its negotiated quantity being above thresholds
    QuantityTooHigh             { quantity_limit: u32 },
    /// An operation was blocked (and all subsequent operations are likely to be blocked) due to the daily limit being reached
    DailyCountTooHigh           { daily_count_limit: u32 },
    /// A buying operation was blocked due to the minute limit of buy/sell pair of operations being reached
    ThroughputTooHigh           { round_transactions_per_minute_limit: u32 },
    // from RiskManagementOrderCancellationConditions... to be polished
    BadSymbolNegotiationStatus,
    BadSymbolRiskStatus,
}

/// All possible connectivity problems the Risk Manager may detect, potentially allowing for a connection to be dropped
/// in the hope the issues get better upon a reconnection.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RiskManagementConnectionDroppingConditions {
    /// Ping too high
    RoundTripTimeTooHigh { nanos: u32 },
    /// Unresponsive peer
    PingTimeout,
    /// Clocks way out of sync
    ClockSkewTooHigh { estimated_delta_nanos: u32 },
}

/// All possible `RiskManager` detectable conditions
#[derive(Debug)]
pub enum RiskManagementConditions {
    /// Informs, whoever it may concern, that an intention to place an order, from `DecisionMaker`, won't be made into an Order Event endorsed by the `RiskManager`
    OrderRefused(RiskManagementOrderRefusalConditions),
    /// Informs the network handler that the connection with the client should be dropped
    ConnectionShouldBeDropped(RiskManagementConnectionDroppingConditions),
}

/// Represents an order execution command
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OrderCommand {
    Buy(Order),
    Sell(Order),
}

/// Represents an order to be executed OR an already executed order
/// TODO: make 2 versions of this (like we do for trades): one for the client and the other one for the internal representation (and rip off Serialize/Deserialize from here)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Order {
    /// the order ID, as registered in our Robot
    pub ogre_id: u32,
    /// the party who took initiative to make the trade
    pub aggressor: Parties,
    /// the type of order -- supported by the target exchange
    pub order_type: OrderTypes,
    /// in the form YYYYMMDD
    pub date: u32,
    /// in the form HHMMSSMMM
    pub time: u32,
    /// the symbol -- as recognized by the target exchange
    pub symbol: Symbol,
    /// the unitary paper currency value multiplied by 1000 -- or cent value multiplied by 10
    pub unitary_mill_value: u32,
    /// the number of papers from `symbol`
    pub quantity: u32,
}

/// The parties in trades or to-be-trades
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Parties {
    /// ... or Buy, or the one that bids
    Buyer,
    /// ... or Sell, or the one that asks
    Seller,
}

/// The supported order types & their data
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OrderTypes {
    MarketOrder,
    LimitedOrder { price_limit_mill: u32 },
}

/// Payload for [Events.market_data] event
#[derive(Clone, Debug, PartialEq)]
pub enum MarketData {
    SymbolState(SymbolState),
    BookState(SingleBook),
    Trade(SingleTrade),
}

/// Sent by a [ConnectorIdentification::MarketDataBridge] client to update the server with the symbol information
/// -- updated after connection, sporadically or when a state change happens
#[derive(Clone, Debug, PartialEq)]
pub struct SymbolState {
    pub symbol: String,
    pub in_auction: bool,
}

/// Represents a book entry to be kept in containers that groups them by `symbol` and `side`,
/// which are omitted to preserve space (together with `date` & `time`)
#[derive(Clone, Debug, PartialEq)]
pub struct GroupedBook {
    /// the unitary paper currency value -- see [MonetaryMillValue]
    price_level_mills: MonetaryMillValue,
    /// the number of orders waiting
    n_orders: u32,
    /// the total quantity of booked orders
    available_quantity: u32,
}

/// Represents the full info of a book event -- to be yet grouped and stored (see [GroupedBook])
#[derive(Clone, Debug, PartialEq)]
pub struct SingleBook {
    date: NeatDate,
    time: NeatTime,
    symbol: Symbol,
    /// the unitary paper currency value -- see [MonetaryMillValue]
    price_level_mills: MonetaryMillValue,
    /// the number of orders waiting
    n_orders: u32,
    /// the total quantity of booked orders
    available_quantity: u32,
    /// the operation those orders want to make
    side: Parties,
}

/// Represents a trade made to be kept in containers that groups them by `symbol` and `date`,
/// which are omitted to preserve space.
#[derive(Clone, Debug, PartialEq)]
pub struct GroupedTrade {
    /// time at which this trade occurred
    time: NeatTime,
    /// the unitary paper currency value -- see [MonetaryMillValue]
    unitary_mill_value: MonetaryMillValue,
    /// how many papers of that symbol were traded
    quantity: u32,
    /// who emitted the Market Order?
    aggressor: Parties,
}

/// Represents the full info of a trade -- to be yet grouped and stored (see [GroupedTrade])
#[derive(Clone, Debug, PartialEq)]
pub struct SingleTrade {
    date: NeatDate,
    time: NeatTime,
    symbol: Symbol,
    /// the unitary paper currency value -- see [MonetaryMillValue]
    unitary_mill_value: MonetaryMillValue,
    /// how many papers of that symbol were traded
    quantity: u32,
    /// who emitted the Market Order?
    aggressor: Parties,
}