
/// Holds all symbol information -- struct crafted from MT5's `SymbolInfoInteger()`, `SymbolInfoDouble()` and `SymbolInfoString()`
#[repr(C)]
#[derive(Debug)]
struct SymbolInfoBridge {
	/// Symbol data arrives with a delay. The property can be requested only for symbols selected in MarketWatch (SYMBOL_SELECT = true). The ERR_MARKET_NOT_SELECTED (4302) error is generated for other symbols
	symbol_subscription_delay: bool,
	/// The sector of the economy to which the asset belongs
	symbol_sector: EnumSymbolSector,
	/// The industry or the economy branch to which the symbol belongs
	symbol_industry: EnumSymbolIndustry,
	/// It is a custom symbol – the symbol has been created synthetically based on other symbols from the Market Watch and/or external data sources
	symbol_custom: bool,
	/// The color of the background used for the symbol in Market Watch
	symbol_background_color: color,
	/// The price type used for generating symbols bars, i.e. Bid or Last
	symbol_chart_mode: EnumSymbolChartMode,
	/// Symbol with this name exists
	symbol_exist: bool,
	/// Symbol is selected in Market Watch
	symbol_select: bool,
	/// Symbol is visible in Market Watch.
	symbol_visible: bool,
	/// Number of deals in the current session
	symbol_session_deals: long,
	/// Number of Buy orders at the moment
	symbol_session_buy_orders: long,
	/// Number of Sell orders at the moment
	symbol_session_sell_orders: long,
	/// Volume of the last deal
	symbol_volume: long,
	/// Maximal day volume
	symbol_volumehigh: long,
	/// Minimal day volume
	symbol_volumelow: long,
	/// Time of the last quote
	symbol_time: datetime,
	/// Time of the last quote in milliseconds since 1970.01.01
	symbol_time_msc: long,
	/// Digits after a decimal point
	symbol_digits: int,
	/// Indication of a floating spread
	symbol_spread_float: bool,
	/// Spread value in points
	symbol_spread: int,
	/// Maximal number of requests shown in Depth of Market. For symbols that have no queue of requests, the value is equal to zero.
	symbol_ticks_bookdepth: int,
	/// Contract price calculation mode
	symbol_trade_calc_mode: EnumSymbolCalcMode,
	/// Order execution type
	symbol_trade_mode: EnumSymbolTradeMode,
	/// Date of the symbol trade beginning (usually used for futures)
	symbol_start_time: datetime,
	/// Date of the symbol trade end (usually used for futures)
	symbol_expiration_time: datetime,
	/// Minimal indention in points from the current close price to place Stop orders
	symbol_trade_stops_level: int,
	/// Distance to freeze trade operations in points
	symbol_trade_freeze_level: int,
	/// Deal execution mode
	symbol_trade_exemode: EnumSymbolTradeExecution,
	/// Swap calculation model
	symbol_swap_mode: EnumSymbolSwapMode,
	/// The day of week to charge 3-day swap rollover
	symbol_swap_rollover3days: EnumDayOfWeek,
	/// Calculating hedging margin using the larger leg (Buy or Sell)
	symbol_margin_hedged_use_leg: bool,
	/// Flags of allowed order expiration modes
	symbol_expiration_mode : int,
	/// Flags of allowed order filling modes
	symbol_filling_mode: int,
	/// Flags of allowed order types
	symbol_order_mode: int,
	/// Expiration of Stop Loss and Take Profit orders, if SYMBOL_EXPIRATION_MODE=SYMBOL_EXPIRATION_GTC (Good till canceled)
	symbol_order_gtc_mode: EnumSymbolOrderGtcMode,
	/// Option type
	symbol_option_mode: EnumSymbolOptionMode,
	/// Option right (Call/Put)
	symbol_option_right: EnumSymbolOptionRight,
	/// Bid - best sell offer
	symbol_bid: double,
	/// Maximal Bid of the day
	symbol_bidhigh: double,
	/// Minimal Bid of the day
	symbol_bidlow: double,
	/// Ask - best buy offer
	symbol_ask: double,
	/// Maximal Ask of the day
	symbol_askhigh: double,
	/// Minimal Ask of the day
	symbol_asklow: double,
	/// Price of the last deal
	symbol_last: double,
	/// Maximal Last of the day
	symbol_lasthigh: double,
	/// Minimal Last of the day
	symbol_lastlow: double,
	/// Volume of the last deal
	symbol_volume_real: double,
	/// Maximum Volume of the day
	symbol_volumehigh_real: double,
	/// Minimum Volume of the day
	symbol_volumelow_real: double,
	/// The strike price of an option. The price at which an option buyer can buy (in a Call option) or sell (in a Put option) the underlying asset, and the option seller is obliged to sell or buy the appropriate amount of the underlying asset.
	symbol_option_strike: double,
	/// Symbol point value
	symbol_point: double,
	/// Value of SYMBOL_TRADE_TICK_VALUE_PROFIT
	symbol_trade_tick_value: double,
	/// Calculated tick price for a profitable position
	symbol_trade_tick_value_profit: double,
	/// Calculated tick price for a losing position
	symbol_trade_tick_value_loss: double,
	/// Minimal price change
	symbol_trade_tick_size: double,
	/// Trade contract size
	symbol_trade_contract_size: double,
	/// Accrued interest – accumulated coupon interest, i.e. part of the coupon interest calculated in proportion to the number of days since the coupon bond issuance or the last coupon interest payment
	symbol_trade_accrued_interest: double,
	/// Face value – initial bond value set by the issuer
	symbol_trade_face_value: double,
	/// Liquidity Rate is the share of the asset that can be used for the margin.
	symbol_trade_liquidity_rate: double,
	/// Minimal volume for a deal
	symbol_volume_min: double,
	/// Maximal volume for a deal
	symbol_volume_max: double,
	/// Minimal volume change step for deal execution
	symbol_volume_step: double,
	/// Maximum allowed aggregate volume of an open position and pending orders in one direction (buy or sell) for the symbol. For example, with the limitation of 5 lots, you can have an open buy position with the volume of 5 lots and place a pending order Sell Limit with the volume of 5 lots. But in this case you cannot place a Buy Limit pending order (since the total volume in one direction will exceed the limitation) or place Sell Limit with the volume more than 5 lots.
	symbol_volume_limit: double,
	/// Long swap value
	symbol_swap_long: double,
	/// Short swap value
	symbol_swap_short: double,
	/// Swap calculation ratio (SYMBOL_SWAP_LONG or SYMBOL_SWAP_SHORT) for overnight positions rolled over from SUNDAY to the next day. There following values are supported:
	symbol_swap_sunday: double,
	/// Swap calculation ratio (SYMBOL_SWAP_LONG or SYMBOL_SWAP_SHORT) for overnight positions rolled over from Monday to Tuesday
	symbol_swap_monday: double,
	/// Swap calculation ratio (SYMBOL_SWAP_LONG or SYMBOL_SWAP_SHORT) for overnight positions rolled over from Tuesday to Wednesday
	symbol_swap_tuesday: double,
	/// Swap calculation ratio (SYMBOL_SWAP_LONG or SYMBOL_SWAP_SHORT) for overnight positions rolled over from Wednesday to Thursday
	symbol_swap_wednesday: double,
	/// Swap calculation ratio (SYMBOL_SWAP_LONG or SYMBOL_SWAP_SHORT) for overnight positions rolled over from Thursday to Friday
	symbol_swap_thursday: double,
	/// Swap calculation ratio (SYMBOL_SWAP_LONG or SYMBOL_SWAP_SHORT) for overnight positions rolled over from Friday to Saturday
	symbol_swap_friday: double,
	/// Swap calculation ratio (SYMBOL_SWAP_LONG or SYMBOL_SWAP_SHORT) for overnight positions rolled over from Saturday to Sunday
	symbol_swap_saturday: double,
	/// Initial margin means the amount in the margin currency required for opening a position with the volume of one lot. It is used for checking a client's assets when he or she enters the market.
	symbol_margin_initial: double,
	/// The maintenance margin. If it is set, it sets the margin amount in the margin currency of the symbol, charged from one lot. It is used for checking a client's assets when his/her account state changes. If the maintenance margin is equal to 0, the initial margin is used.
	symbol_margin_maintenance: double,
	/// Summary volume of current session deals
	symbol_session_volume: double,
	/// Summary turnover of the current session
	symbol_session_turnover: double,
	/// Summary open interest
	symbol_session_interest: double,
	/// Current volume of Buy orders
	symbol_session_buy_orders_volume: double,
	/// Current volume of Sell orders
	symbol_session_sell_orders_volume: double,
	/// Open price of the current session
	symbol_session_open: double,
	/// Close price of the current session
	symbol_session_close: double,
	/// Average weighted price of the current session
	symbol_session_aw: double,
	/// Settlement price of the current session
	symbol_session_price_settlement: double,
	/// Minimal price of the current session
	symbol_session_price_limit_min: double,
	/// Maximal price of the current session
	symbol_session_price_limit_max: double,
	/// Contract size or margin value per one lot of hedged positions (oppositely directed positions of one symbol). Two margin calculation methods are possible for hedged positions. The calculation method is defined by the broker. 
	symbol_margin_hedged: double,
	/// Change of the current price relative to the end of the previous trading day in %
	symbol_price_change: double,
	/// Price volatility in %
	symbol_price_volatility: double,
	/// Theoretical option price
	symbol_price_theoretical: double,
	/// Option/warrant delta shows the value the option price changes by, when the underlying asset price changes by 1
	symbol_price_delta: double,
	/// Option/warrant theta shows the number of points the option price is to lose every day due to a temporary breakup, i.e. when the expiration date approaches
	symbol_price_theta: double,
	/// Option/warrant gamma shows the change rate of delta – how quickly or slowly the option premium changes
	symbol_price_gamma: double,
	/// Option/warrant vega shows the number of points the option price changes by when the volatility changes by 1%
	symbol_price_vega: double,
	/// Option/warrant rho reflects the sensitivity of the theoretical option price to the interest rate changing by 1%
	symbol_price_rho: double,
	/// Option/warrant omega. Option elasticity shows a relative percentage change of the option price by the percentage change of the underlying asset price
	symbol_price_omega: double,
	/// Option/warrant sensitivity shows by how many points the price of the option's underlying asset should change so that the price of the option changes by one point
	symbol_price_sensitivity: double,
	/// The underlying asset of a derivative
	symbol_basis: string,
	/// The name of the sector or category to which the financial symbol belongs
	symbol_category: string,
	/// The country to which the financial symbol belongs
	symbol_country: string,
	/// The sector of the economy to which the financial symbol belongs
	symbol_sector_name: string,
	/// The industry branch or the industry to which the financial symbol belongs
	symbol_industry_name: string,
	/// Basic currency of a symbol
	symbol_currency_base: string,
	/// Profit currency
	symbol_currency_profit: string,
	/// Margin currency
	symbol_currency_margin: string,
	/// Feeder of the current quote
	symbol_bank: string,
	/// Symbol description
	symbol_description: string,
	/// The name of the exchange in which the financial symbol is traded
	symbol_exchange: string,
	/// The formula used for the custom symbol pricing. If the name of a financial symbol used in the formula starts with a digit or contains a special character (&quot;&gt;&quot; &quot;, &quot;.&quot;, &quot;-&quot;, &quot;&amp;&quot;, &quot;#&quot; and so on) quotation marks should be used around this symbol name.
	symbol_formula: string,
	/// The name of a symbol in the ISIN system (International Securities Identification Number). The International Securities Identification Number is a 12-digit alphanumeric code that uniquely identifies a security. The presence of this symbol property is determined on the side of a trade server.
	symbol_isin: string,
	/// The address of the web page containing symbol information. This address will be displayed as a link when viewing symbol properties in the terminal
	symbol_page: string,
	/// Path in the symbol tree
	symbol_path: string,
}

/// Values of the ENUM_DAY_OF_WEEK enumeration are used for specifying days of week./
/// auto-generated from https://www.mql5.com/en/docs/constants/environment_state/marketinfoconstants
#[repr(C)]
#[derive(Debug)]
enum EnumDayOfWeek {
	/// Sunday
	Sunday = 0i32,
	/// Monday
	Monday,
	/// Tuesday
	Tuesday,
	/// Wednesday
	Wednesday,
	/// Thursday
	Thursday,
	/// Friday
	Friday,
	/// Saturday
	Saturday,
}

/// The ENUM_SYMBOL_CALC_MODE enumeration is used for obtaining information about how the margin requirements for a symbol are calculated./
/// auto-generated from https://www.mql5.com/en/docs/constants/environment_state/marketinfoconstants
#[repr(C)]
#[derive(Debug)]
enum EnumSymbolCalcMode {
	/// Forex mode - calculation of profit and margin for Forex
	SymbolCalcModeForex = 0i32,
	/// Forex No Leverage mode – calculation of profit and margin for Forex symbols without taking into account the leverage
	SymbolCalcModeForexNoLeverage,
	/// Futures mode - calculation of margin and profit for futures
	SymbolCalcModeFutures,
	/// CFD mode - calculation of margin and profit for CFD
	SymbolCalcModeCfd,
	/// CFD index mode - calculation of margin and profit for CFD by indexes
	SymbolCalcModeCfdindex,
	/// CFD Leverage mode - calculation of margin and profit for CFD at leverage trading
	SymbolCalcModeCfdleverage,
	/// Exchange mode – calculation of margin and profit for trading securities on a stock exchange
	SymbolCalcModeExchStocks,
	/// Futures mode –  calculation of margin and profit for trading futures contracts on a stock exchange
	SymbolCalcModeExchFutures,
	/// FORTS Futures mode –  calculation of margin and profit for trading futures contracts on FORTS. The margin may be reduced by the amount of MarginDiscount deviation according to the following rules:/
	/// /
	/// 1. If the price of a long position (buy order) is less than the estimated price, MarginDiscount = Lots*((PriceSettle-PriceOrder)*TickPrice/TickSize)/
	/// /
	/// 2. If the price of a short position (sell order) exceeds the estimated price, MarginDiscount = Lots*((PriceOrder-PriceSettle)*TickPrice/TickSize)/
	/// /
	/// where:/
	/// /
	/// /
	/// PriceSettle – estimated (clearing) price of the previous session;/
	/// PriceOrder – average weighted position price or open price set in the order (request);/
	/// TickPrice – tick price (cost of the price change by one point)/
	/// TickSize – tick size (minimum price change step)/
	/// /
	/// /
	/// /
	/// /
	/// Margin: Lots * InitialMargin * Margin_Rate or Lots * MaintenanceMargin * Margin_Rate * Margin_Rate/
	/// /
	///  /
	/// /
	/// Profit:  (close_price - open_price) * Lots * TickPrice / TickSize
	SymbolCalcModeExchFuturesForts,
	/// Exchange Bonds mode – calculation of margin and profit for trading bonds on a stock exchange
	SymbolCalcModeExchBonds,
	/// Exchange MOEX Stocks mode – calculation of margin and profit for trading securities on MOEX
	SymbolCalcModeExchStocksMoex,
	/// Exchange MOEX Bonds mode – calculation of margin and profit for trading bonds on MOEX
	SymbolCalcModeExchBondsMoex,
	/// Collateral mode - a symbol is used as a non-tradable asset on a trading account. The market value of an open position is calculated based on the volume, current market price, contract size and liquidity ratio. The value is included into Assets, which are added to Equity. Open positions of such symbols increase the Free Margin amount and are used as additional margin (collateral) for open positions of tradable instruments.
	SymbolCalcModeServCollateral,
}

/// A symbol price chart can be based on Bid or Last prices. The price selected for symbol charts also affects the generation and display of bars in the terminal. Possible values of the SYMBOL_CHART_MODE property are described in ENUM_SYMBOL_CHART_MODE/
/// auto-generated from https://www.mql5.com/en/docs/constants/environment_state/marketinfoconstants
#[repr(C)]
#[derive(Debug)]
enum EnumSymbolChartMode {
	/// Bars are based on Bid prices
	SymbolChartModeBid = 0i32,
	/// Bars are based on Last prices
	SymbolChartModeLast,
}

/// Each financial instrument can be assigned to a specific type of industry or economy branch. An industry is a branch of an economy that produces a closely related set of raw materials, goods, or services. ENUM_SYMBOL_INDUSTRY lists industries which a trading instrument can belong to./
/// auto-generated from https://www.mql5.com/en/docs/constants/environment_state/marketinfoconstants
#[repr(C)]
#[derive(Debug)]
enum EnumSymbolIndustry {
	/// Undefined
	IndustryUndefined = 0i32,
	/// Agricultural inputs
	IndustryAgriculturalInputs,
	/// Aluminium
	IndustryAluminium,
	/// Building materials
	IndustryBuildingMaterials,
	/// Chemicals
	IndustryChemicals,
	/// Coking coal
	IndustryCokingCoal,
	/// Copper
	IndustryCopper,
	/// Gold
	IndustryGold,
	/// Lumber and wood production
	IndustryLumberWood,
	/// Other industrial metals and mining
	IndustryIndustrialMetals,
	/// Other precious metals and mining
	IndustryPreciousMetals,
	/// Paper and paper products
	IndustryPaper,
	/// Silver
	IndustrySilver,
	/// Specialty chemicals
	IndustrySpecialtyChemicals,
	/// Steel
	IndustrySteel,
	/// Advertising agencies
	IndustryAdvertising,
	/// Broadcasting
	IndustryBroadcasting,
	/// Electronic gaming and multimedia
	IndustryGamingMultimedia,
	/// Entertainment
	IndustryEntertainment,
	/// Internet content and information
	IndustryInternetContent,
	/// Publishing
	IndustryPublishing,
	/// Telecom services
	IndustryTelecom,
	/// Apparel manufacturing
	IndustryApparelManufacturing,
	/// Apparel retail
	IndustryApparelRetail,
	/// Auto manufacturers
	IndustryAutoManufacturers,
	/// Auto parts
	IndustryAutoParts,
	/// Auto and truck dealerships
	IndustryAutoDealership,
	/// Department stores
	IndustryDepartmentStores,
	/// Footwear and accessories
	IndustryFootwearAccessories,
	/// Furnishing, fixtures and appliances
	IndustryFurnishings,
	/// Gambling
	IndustryGambling,
	/// Home improvement retail
	IndustryHomeImprovRetail,
	/// Internet retail
	IndustryInternetRetail,
	/// Leisure
	IndustryLeisure,
	/// Lodging
	IndustryLodging,
	/// Luxury goods
	IndustryLuxuryGoods,
	/// Packaging and containers
	IndustryPackagingContainers,
	/// Personal services
	IndustryPersonalServices,
	/// Recreational vehicles
	IndustryRecreationalVehicles,
	/// Residential construction
	IndustryResidentConstruction,
	/// Resorts and casinos
	IndustryResortsCasinos,
	/// Restaurants
	IndustryRestaurants,
	/// Specialty retail
	IndustrySpecialtyRetail,
	/// Textile manufacturing
	IndustryTextileManufacturing,
	/// Travel services
	IndustryTravelServices,
	/// Beverages - Brewers
	IndustryBeveragesBrewers,
	/// Beverages - Non-alcoholic
	IndustryBeveragesNonAlco,
	/// Beverages - Wineries and distilleries
	IndustryBeveragesWineries,
	/// Confectioners
	IndustryConfectioners,
	/// Discount stores
	IndustryDiscountStores,
	/// Education and training services
	IndustryEducationTrainig,
	/// Farm products
	IndustryFarmProducts,
	/// Food distribution
	IndustryFoodDistribution,
	/// Grocery stores
	IndustryGroceryStores,
	/// Household and personal products
	IndustryHouseholdProducts,
	/// Packaged foods
	IndustryPackagedFoods,
	/// Tobacco
	IndustryTobacco,
	/// Oil and gas drilling
	IndustryOilGasDrilling,
	/// Oil and gas extraction and processing
	IndustryOilGasEp,
	/// Oil and gas equipment and services
	IndustryOilGasEquipment,
	/// Oil and gas integrated
	IndustryOilGasIntegrated,
	/// Oil and gas midstream
	IndustryOilGasMidstream,
	/// Oil and gas refining and marketing
	IndustryOilGasRefining,
	/// Thermal coal
	IndustryThermalCoal,
	/// Uranium
	IndustryUranium,
	/// Exchange traded fund
	IndustryExchangeTradedFund,
	/// Assets management
	IndustryAssetsManagement,
	/// Banks - Diversified
	IndustryBanksDiversified,
	/// Banks - Regional
	IndustryBanksRegional,
	/// Capital markets
	IndustryCapitalMarkets,
	/// Closed-End fund - Debt
	IndustryCloseEndFundDebt,
	/// Closed-end fund - Equity
	IndustryCloseEndFundEquity,
	/// Closed-end fund - Foreign
	IndustryCloseEndFundForeign,
	/// Credit services
	IndustryCreditServices,
	/// Financial conglomerates
	IndustryFinancialConglomerate,
	/// Financial data and stock exchange
	IndustryFinancialDataExchange,
	/// Insurance brokers
	IndustryInsuranceBrokers,
	/// Insurance - Diversified
	IndustryInsuranceDiversified,
	/// Insurance - Life
	IndustryInsuranceLife,
	/// Insurance - Property and casualty
	IndustryInsuranceProperty,
	/// Insurance - Reinsurance
	IndustryInsuranceReinsurance,
	/// Insurance - Specialty
	IndustryInsuranceSpecialty,
	/// Mortgage finance
	IndustryMortgageFinance,
	/// Shell companies
	IndustryShellCompanies,
	/// Biotechnology
	IndustryBiotechnology,
	/// Diagnostics and research
	IndustryDiagnosticsResearch,
	/// Drugs manufacturers - general
	IndustryDrugsManufacturers,
	/// Drugs manufacturers - Specialty and generic
	IndustryDrugsManufacturersSpec,
	/// Healthcare plans
	IndustryHealthcarePlans,
	/// Health information services
	IndustryHealthInformation,
	/// Medical care facilities
	IndustryMedicalFacilities,
	/// Medical devices
	IndustryMedicalDevices,
	/// Medical distribution
	IndustryMedicalDistribution,
	/// Medical instruments and supplies
	IndustryMedicalInstruments,
	/// Pharmaceutical retailers
	IndustryPharmRetailers,
	/// Aerospace and defense
	IndustryAerospaceDefense,
	/// Airlines
	IndustryAirlines,
	/// Airports and air services
	IndustryAirportsServices,
	/// Building products and equipment
	IndustryBuildingProducts,
	/// Business equipment and supplies
	IndustryBusinessEquipment,
	/// Conglomerates
	IndustryConglomerates,
	/// Consulting services
	IndustryConsultingServices,
	/// Electrical equipment and parts
	IndustryElectricalEquipment,
	/// Engineering and construction
	IndustryEngineeringConstruction,
	/// Farm and heavy construction machinery
	IndustryFarmHeavyMachinery,
	/// Industrial distribution
	IndustryIndustrialDistribution,
	/// Infrastructure operations
	IndustryInfrastructureOperations,
	/// Integrated freight and logistics
	IndustryFreightLogistics,
	/// Marine shipping
	IndustryMarineShipping,
	/// Metal fabrication
	IndustryMetalFabrication,
	/// Pollution and treatment controls
	IndustryPollutionControl,
	/// Railroads
	IndustryRailroads,
	/// Rental and leasing services
	IndustryRentalLeasing,
	/// Security and protection services
	IndustrySecurityProtection,
	/// Specialty business services
	IndustrySpealityBusinessServices,
	/// Specialty industrial machinery
	IndustrySpealityMachinery,
	/// Stuffing and employment services
	IndustryStuffingEmployment,
	/// Tools and accessories
	IndustryToolsAccessories,
	/// Trucking
	IndustryTrucking,
	/// Waste management
	IndustryWasteManagement,
	/// Real estate - Development
	IndustryRealEstateDevelopment,
	/// Real estate - Diversified
	IndustryRealEstateDiversified,
	/// Real estate services
	IndustryRealEstateServices,
	/// REIT - Diversified
	IndustryReitDiversified,
	/// REIT - Healthcase facilities
	IndustryReitHealtcare,
	/// REIT - Hotel and motel
	IndustryReitHotelMotel,
	/// REIT - Industrial
	IndustryReitIndustrial,
	/// REIT - Mortgage
	IndustryReitMortage,
	/// REIT - Office
	IndustryReitOffice,
	/// REIT - Residential
	IndustryReitResidental,
	/// REIT - Retail
	IndustryReitRetail,
	/// REIT - Specialty
	IndustryReitSpeciality,
	/// Communication equipment
	IndustryCommunicationEquipment,
	/// Computer hardware
	IndustryComputerHardware,
	/// Consumer electronics
	IndustryConsumerElectronics,
	/// Electronic components
	IndustryElectronicComponents,
	/// Electronics and computer distribution
	IndustryElectronicDistribution,
	/// Information technology services
	IndustryItServices,
	/// Scientific and technical instruments
	IndustryScientificInstruments,
	/// Semiconductor equipment and materials
	IndustrySemiconductorEquipment,
	/// Semiconductors
	IndustrySemiconductors,
	/// Software - Application
	IndustrySoftwareApplication,
	/// Software - Infrastructure
	IndustrySoftwareInfrastructure,
	/// Solar
	IndustrySolar,
	/// Utilities - Diversified
	IndustryUtilitiesDiversified,
	/// Utilities - Independent power producers
	IndustryUtilitiesPowerproducers,
	/// Utilities - Renewable
	IndustryUtilitiesRenewable,
	/// Utilities - Regulated electric
	IndustryUtilitiesRegulatedElectric,
	/// Utilities - Regulated gas
	IndustryUtilitiesRegulatedGas,
	/// Utilities - Regulated water
	IndustryUtilitiesRegulatedWater,
	/// Start of the utilities services types enumeration. Corresponds to INDUSTRY_UTILITIES_DIVERSIFIED.
	IndustryUtilitiesFirst,
	/// End of the utilities services types enumeration. Corresponds to INDUSTRY_UTILITIES_REGULATED_WATER.
	IndustryUtilitiesLast,
}

/// /
/// auto-generated from https://www.mql5.com/en/docs/constants/environment_state/marketinfoconstants
#[repr(C)]
#[derive(Debug)]
enum EnumSymbolOptionMode {
	/// European option may only be exercised on a specified date (expiration, execution date, delivery date)
	SymbolOptionModeEuropean = 0i32,
	/// American option may be exercised on any trading day or before expiry. The period within which a buyer can exercise the option is specified for it
	SymbolOptionModeAmerican ,
}

/// An option is a contract, which gives the right, but not the obligation, to buy or sell an underlying asset (goods, stocks, futures, etc.) at a specified price on or before a specific date. The following enumerations describe option properties, including the option type and the right arising from it. /
/// auto-generated from https://www.mql5.com/en/docs/constants/environment_state/marketinfoconstants
#[repr(C)]
#[derive(Debug)]
enum EnumSymbolOptionRight {
	/// A call option gives you the right to buy an asset at a specified price
	SymbolOptionRightCall = 0i32,
	/// A put option gives you the right to sell an asset at a specified price
	SymbolOptionRightPut,
}

/// If the SYMBOL_EXPIRATION_MODE property is set to SYMBOL_EXPIRATION_GTC (good till canceled), the expiration of pending orders, as well as of Stop Loss/Take Profit orders should be additionally set using the ENUM_SYMBOL_ORDER_GTC_MODE enumeration./
/// auto-generated from https://www.mql5.com/en/docs/constants/environment_state/marketinfoconstants
#[repr(C)]
#[derive(Debug)]
enum EnumSymbolOrderGtcMode {
	/// Pending orders and Stop Loss/Take Profit levels are valid for an unlimited period until their explicit cancellation
	SymbolOrdersGtc = 0i32,
	/// Orders are valid during one trading day. At the end of the day, all Stop Loss and Take Profit levels, as well as pending orders are deleted.
	SymbolOrdersDaily,
	/// When a trade day changes, only pending orders are deleted, while Stop Loss and Take Profit levels are preserved.
	SymbolOrdersDailyExcludingStops,
}

/// Financial instruments are categorized by sectors of the economy. An economic sector is a part of economic activity which has specific characteristics, economic goals, functions and behavior, which allow separating this sector from other parts of the economy. ENUM_SYMBOL_SECTOR lists the economic sectors which a trading instruments can belong to./
/// auto-generated from https://www.mql5.com/en/docs/constants/environment_state/marketinfoconstants
#[repr(C)]
#[derive(Debug)]
enum EnumSymbolSector {
	/// Undefined
	SectorUndefined = 0i32,
	/// Basic materials
	SectorBasicMaterials,
	/// Communication services
	SectorCommunicationServices,
	/// Consumer cyclical
	SectorConsumerCyclical,
	/// Consumer defensive
	SectorConsumerDefensive,
	/// Currencies
	SectorCurrency,
	/// Cryptocurrencies
	SectorCurrencyCrypto,
	/// Energy
	SectorEnergy,
	/// Finance
	SectorFinancial,
	/// Healthcare
	SectorHealthcare,
	/// Industrials
	SectorIndustrials,
	/// Real estate
	SectorRealEstate,
	/// Technology
	SectorTechnology,
	/// Utilities
	SectorUtilities,
}

/// Methods of swap calculation at position transfer are specified in enumeration ENUM_SYMBOL_SWAP_MODE. The method of swap calculation determines the units of measure of the SYMBOL_SWAP_LONG and SYMBOL_SWAP_SHORT parameters. For example, if swaps are charged in the client deposit currency, then the values of those parameters are specified as an amount of money in the client deposit currency./
/// auto-generated from https://www.mql5.com/en/docs/constants/environment_state/marketinfoconstants
#[repr(C)]
#[derive(Debug)]
enum EnumSymbolSwapMode {
	/// Swaps disabled (no swaps)
	SymbolSwapModeDisabled = 0i32,
	/// Swaps are charged in points
	SymbolSwapModePoints,
	/// Swaps are charged in money in base currency of the symbol
	SymbolSwapModeCurrencySymbol,
	/// Swaps are charged in money in margin currency of the symbol
	SymbolSwapModeCurrencyMargin,
	/// Swaps are charged in money, in client deposit currency
	SymbolSwapModeCurrencyDeposit,
	/// Swaps are charged as the specified annual interest from the instrument price at calculation of swap (standard bank year is 360 days)
	SymbolSwapModeInterestCurrent,
	/// Swaps are charged as the specified annual interest from the open price of position (standard bank year is 360 days)
	SymbolSwapModeInterestOpen,
	/// Swaps are charged by reopening positions. At the end of a trading day the position is closed. Next day it is reopened by the close price +/- specified number of points (parameters SYMBOL_SWAP_LONG and SYMBOL_SWAP_SHORT)
	SymbolSwapModeReopenCurrent,
	/// Swaps are charged by reopening positions. At the end of a trading day the position is closed. Next day it is reopened by the current Bid price +/- specified number of points (parameters SYMBOL_SWAP_LONG and SYMBOL_SWAP_SHORT)
	SymbolSwapModeReopenBid,
}

/// Possible deal execution modes for a certain symbol are defined in enumeration ENUM_SYMBOL_TRADE_EXECUTION./
/// auto-generated from https://www.mql5.com/en/docs/constants/environment_state/marketinfoconstants
#[repr(C)]
#[derive(Debug)]
enum EnumSymbolTradeExecution {
	///  Execution by request
	SymbolTradeExecutionRequest = 0i32,
	/// Instant execution
	SymbolTradeExecutionInstant,
	/// Market execution
	SymbolTradeExecutionMarket,
	/// Exchange execution
	SymbolTradeExecutionExchange,
}

/// There are several symbol trading modes. Information about trading modes of a certain symbol is reflected in the values of enumeration ENUM_SYMBOL_TRADE_MODE./
/// auto-generated from https://www.mql5.com/en/docs/constants/environment_state/marketinfoconstants
#[repr(C)]
#[derive(Debug)]
enum EnumSymbolTradeMode {
	/// Trade is disabled for the symbol
	SymbolTradeModeDisabled = 0i32,
	/// Allowed only long positions
	SymbolTradeModeLongonly,
	/// Allowed only short positions
	SymbolTradeModeShortonly,
	/// Allowed only position close operations
	SymbolTradeModeCloseonly,
	/// No trade restrictions
	SymbolTradeModeFull,
}

/// It provides information about the market depth data.\
/// auto-generated from https://www.mql5.com/en/docs/constants/structures/mqlbookinfo
#[repr(C)]
#[derive(Debug)]
struct MqlBookInfo {
	/// Order type from ENUM_BOOK_TYPE enumeration
	type: EnumBookType,
	/// Price
	price: f64,
	/// Volume
	volume: i64,
	/// Volume with greater accuracy
	volume_real: f64,
}

/// To obtain information about the current state of the DOM by MQL5 means, the MarketBookGet() function is used, which places the DOM &quot;screen shot&quot; into the MqlBookInfo array of structures. Each element of the array in the type field contains information about the direction of the order - the value of the ENUM_BOOK_TYPE enumeration./
/// auto-generated from https://www.mql5.com/en/docs/constants/tradingconstants/enum_book_type
#[repr(C)]
#[derive(Debug)]
enum EnumBookType {
	/// Sell order (Offer)
	BookTypeSell = 0i32,
	/// Buy order (Bid)
	BookTypeBuy,
	/// Sell order by Market
	BookTypeSellMarket,
	/// Buy order by Market
	BookTypeBuyMarket,
}

/// This is a structure for storing the latest prices of the symbol. It is designed for fast retrieval of the most requested information about current prices.\
/// auto-generated from https://www.mql5.com/en/docs/constants/structures/mqltick
#[repr(C)]
#[derive(Debug)]
struct MqlTick {
	/// Time of the last prices update
	time: MQ5DateTime,
	/// Current Bid price
	bid: f64,
	/// Current Ask price
	ask: f64,
	/// Price of the last deal (Last)
	last: f64,
	/// Volume for the current Last price
	volume: u64,
	/// Time of a price last update in milliseconds
	time_msc: i64,
	/// Tick flags
	flags: u32,
	/// Volume for the current Last price with greater accuracy
	volume_real: f64,
}

