// Tests our DLL through special feedback functions exposed for that very purpose

#property copyright "Copyright OgreRobot.com"
#property link      "https://www.OgreRobot.com"
#property version   "1.00"
#property strict

#include "RustDll.mqh"


void OnStart() {

    string test_name;
    string expected;
    string observed;
    StringReserve(observed, 16384);  // this will contain the Rust ==> MT5 test results. The `string` should have enough capacity, since it cannot be allocated on the Rust side
                                     // -- unfortunately, if not enough size is provided, memory corruption will happen, since Rust don't receive the buffer size when strings
                                     //    are passed as function parameters (just the pointer to the buffer is passed)


 /*   test_name = "Testing MqlTick serialization...";
    MqlTick mql_tick;
    mql_tick.time        = 12344321;
    mql_tick.bid         = 97.58;
    mql_tick.ask         = 11.75;
    mql_tick.last        = 11.71;   
    mql_tick.volume      = 9814989;
    mql_tick.time_msc    = 4321001;
    mql_tick.flags       = 82;
    mql_tick.volume_real = 3.14159;
    expected = "Mq5MqlTick { time: 12344321, bid: 97.58, ask: 11.75, last: 11.71, volume: 9814989, time_msc: 4321001, flags: 82, volume_real: 3.14159 }";
    test_on_tick(observed, mql_tick);
    assert(observed, expected, test_name);

    Print("MqlTick constants -- assure those values are exposed to Rust:");
    Print("// flags for `Mt5MqlTick::flags`");
    Print("////////////////////////////////");
    Print("// from https://www.mql5.com/en/docs/constants/structures/mqltick");
    expose_constant("TICK_FLAG_BID",    TICK_FLAG_BID,    "tick has changed a Bid price");
    expose_constant("TICK_FLAG_ASK",    TICK_FLAG_ASK,    "a tick has changed an Ask price");
    expose_constant("TICK_FLAG_LAST",   TICK_FLAG_LAST,   "a tick has changed the last deal price");
    expose_constant("TICK_FLAG_VOLUME", TICK_FLAG_VOLUME, "a tick has changed a volume");
    expose_constant("TICK_FLAG_BUY",    TICK_FLAG_BUY,    "a tick is a result of a buy deal");
    expose_constant("TICK_FLAG_SELL",   TICK_FLAG_SELL,   "a tick is a result of a sell deal");
*/

/*   Print("ENUM_SYMBOL_SECTOR:");
   Print("    SECTOR_BASIC_MATERIALS: ", SECTOR_BASIC_MATERIALS);
   Print("    SECTOR_COMMUNICATION_SERVICES: ", SECTOR_COMMUNICATION_SERVICES);
   Print("    SECTOR_CONSUMER_CYCLICAL: ", SECTOR_CONSUMER_CYCLICAL);
   Print("    SECTOR_CONSUMER_DEFENSIVE: ", SECTOR_CONSUMER_DEFENSIVE);
   Print("    SECTOR_CURRENCY: ", SECTOR_CURRENCY);
   Print("    SECTOR_CURRENCY_CRYPTO: ", SECTOR_CURRENCY_CRYPTO);
   Print("    SECTOR_ENERGY: ", SECTOR_ENERGY);
   Print("    SECTOR_FINANCIAL: ", SECTOR_FINANCIAL);
   Print("    SECTOR_HEALTHCARE: ", SECTOR_HEALTHCARE);
   Print("    SECTOR_INDUSTRIALS: ", SECTOR_INDUSTRIALS);
   Print("    SECTOR_REAL_ESTATE: ", SECTOR_REAL_ESTATE);
   Print("    SECTOR_TECHNOLOGY: ", SECTOR_TECHNOLOGY);
   Print("    SECTOR_UTILITIES: ", SECTOR_UTILITIES);
   
   Print("ENUM_SYMBOL_INDUSTRY:");
    Print("    INDUSTRY_UNDEFINED: 	", INDUSTRY_UNDEFINED);
    Print("    INDUSTRY_AGRICULTURAL_INPUTS: 	", INDUSTRY_AGRICULTURAL_INPUTS);
    Print("    INDUSTRY_ALUMINIUM: 	", INDUSTRY_ALUMINIUM);
    Print("    INDUSTRY_BUILDING_MATERIALS: 	", INDUSTRY_BUILDING_MATERIALS);
    Print("    INDUSTRY_CHEMICALS: 	", INDUSTRY_CHEMICALS);
    Print("    INDUSTRY_COKING_COAL: 	", INDUSTRY_COKING_COAL);
    Print("    INDUSTRY_COPPER: 	", INDUSTRY_COPPER);
    Print("    INDUSTRY_GOLD: 	", INDUSTRY_GOLD);
    Print("    INDUSTRY_LUMBER_WOOD: 	", INDUSTRY_LUMBER_WOOD);
    Print("    INDUSTRY_INDUSTRIAL_METALS: 	", INDUSTRY_INDUSTRIAL_METALS);
    Print("    INDUSTRY_PRECIOUS_METALS: 	", INDUSTRY_PRECIOUS_METALS);
    Print("    INDUSTRY_PAPER: 	", INDUSTRY_PAPER);
    Print("    INDUSTRY_SILVER: 	", INDUSTRY_SILVER);
    Print("    INDUSTRY_SPECIALTY_CHEMICALS: 	", INDUSTRY_SPECIALTY_CHEMICALS);
    Print("    INDUSTRY_STEEL: 	", INDUSTRY_STEEL);
    Print("    INDUSTRY_ADVERTISING: 	", INDUSTRY_ADVERTISING);
    Print("    INDUSTRY_BROADCASTING: 	", INDUSTRY_BROADCASTING);
    Print("    INDUSTRY_GAMING_MULTIMEDIA: 	", INDUSTRY_GAMING_MULTIMEDIA);
    Print("    INDUSTRY_ENTERTAINMENT: 	", INDUSTRY_ENTERTAINMENT);
    Print("    INDUSTRY_INTERNET_CONTENT: 	", INDUSTRY_INTERNET_CONTENT);
    Print("    INDUSTRY_PUBLISHING: 	", INDUSTRY_PUBLISHING);
    Print("    INDUSTRY_TELECOM: 	", INDUSTRY_TELECOM);
    Print("    INDUSTRY_APPAREL_MANUFACTURING: 	", INDUSTRY_APPAREL_MANUFACTURING);
    Print("    INDUSTRY_APPAREL_RETAIL: 	", INDUSTRY_APPAREL_RETAIL);
    Print("    INDUSTRY_AUTO_MANUFACTURERS: 	", INDUSTRY_AUTO_MANUFACTURERS);
    Print("    INDUSTRY_AUTO_PARTS: 	", INDUSTRY_AUTO_PARTS);
    Print("    INDUSTRY_AUTO_DEALERSHIP: 	", INDUSTRY_AUTO_DEALERSHIP);
    Print("    INDUSTRY_DEPARTMENT_STORES: 	", INDUSTRY_DEPARTMENT_STORES);
    Print("    INDUSTRY_FOOTWEAR_ACCESSORIES: 	", INDUSTRY_FOOTWEAR_ACCESSORIES);
    Print("    INDUSTRY_FURNISHINGS: 	", INDUSTRY_FURNISHINGS);
    Print("    INDUSTRY_GAMBLING: 	", INDUSTRY_GAMBLING);
    Print("    INDUSTRY_HOME_IMPROV_RETAIL: 	", INDUSTRY_HOME_IMPROV_RETAIL);
    Print("    INDUSTRY_INTERNET_RETAIL: 	", INDUSTRY_INTERNET_RETAIL);
    Print("    INDUSTRY_LEISURE: 	", INDUSTRY_LEISURE);
    Print("    INDUSTRY_LODGING: 	", INDUSTRY_LODGING);
    Print("    INDUSTRY_LUXURY_GOODS: 	", INDUSTRY_LUXURY_GOODS);
    Print("    INDUSTRY_PACKAGING_CONTAINERS: 	", INDUSTRY_PACKAGING_CONTAINERS);
    Print("    INDUSTRY_PERSONAL_SERVICES: 	", INDUSTRY_PERSONAL_SERVICES);
    Print("    INDUSTRY_RECREATIONAL_VEHICLES: 	", INDUSTRY_RECREATIONAL_VEHICLES);
    Print("    INDUSTRY_RESIDENT_CONSTRUCTION: 	", INDUSTRY_RESIDENT_CONSTRUCTION);
    Print("    INDUSTRY_RESORTS_CASINOS: 	", INDUSTRY_RESORTS_CASINOS);
    Print("    INDUSTRY_RESTAURANTS: 	", INDUSTRY_RESTAURANTS);
    Print("    INDUSTRY_SPECIALTY_RETAIL: 	", INDUSTRY_SPECIALTY_RETAIL);
    Print("    INDUSTRY_TEXTILE_MANUFACTURING: 	", INDUSTRY_TEXTILE_MANUFACTURING);
    Print("    INDUSTRY_TRAVEL_SERVICES: 	", INDUSTRY_TRAVEL_SERVICES);
    Print("    INDUSTRY_BEVERAGES_BREWERS: 	", INDUSTRY_BEVERAGES_BREWERS);
    Print("    INDUSTRY_BEVERAGES_NON_ALCO: 	", INDUSTRY_BEVERAGES_NON_ALCO);
    Print("    INDUSTRY_BEVERAGES_WINERIES: 	", INDUSTRY_BEVERAGES_WINERIES);
    Print("    INDUSTRY_CONFECTIONERS: 	", INDUSTRY_CONFECTIONERS);
    Print("    INDUSTRY_DISCOUNT_STORES: 	", INDUSTRY_DISCOUNT_STORES);
    Print("    INDUSTRY_EDUCATION_TRAINIG: 	", INDUSTRY_EDUCATION_TRAINIG);
    Print("    INDUSTRY_FARM_PRODUCTS: 	", INDUSTRY_FARM_PRODUCTS);
    Print("    INDUSTRY_FOOD_DISTRIBUTION: 	", INDUSTRY_FOOD_DISTRIBUTION);
    Print("    INDUSTRY_GROCERY_STORES: 	", INDUSTRY_GROCERY_STORES);
    Print("    INDUSTRY_HOUSEHOLD_PRODUCTS: 	", INDUSTRY_HOUSEHOLD_PRODUCTS);
    Print("    INDUSTRY_PACKAGED_FOODS: 	", INDUSTRY_PACKAGED_FOODS);
    Print("    INDUSTRY_TOBACCO: 	", INDUSTRY_TOBACCO);
    Print("    INDUSTRY_OIL_GAS_DRILLING: 	", INDUSTRY_OIL_GAS_DRILLING);
    Print("    INDUSTRY_OIL_GAS_EP: 	", INDUSTRY_OIL_GAS_EP);
    Print("    INDUSTRY_OIL_GAS_EQUIPMENT: 	", INDUSTRY_OIL_GAS_EQUIPMENT);
    Print("    INDUSTRY_OIL_GAS_INTEGRATED: 	", INDUSTRY_OIL_GAS_INTEGRATED);
    Print("    INDUSTRY_OIL_GAS_MIDSTREAM: 	", INDUSTRY_OIL_GAS_MIDSTREAM);
    Print("    INDUSTRY_OIL_GAS_REFINING: 	", INDUSTRY_OIL_GAS_REFINING);
    Print("    INDUSTRY_THERMAL_COAL: 	", INDUSTRY_THERMAL_COAL);
    Print("    INDUSTRY_URANIUM: 	", INDUSTRY_URANIUM);
    Print("    INDUSTRY_EXCHANGE_TRADED_FUND: 	", INDUSTRY_EXCHANGE_TRADED_FUND);
    Print("    INDUSTRY_ASSETS_MANAGEMENT: 	", INDUSTRY_ASSETS_MANAGEMENT);
    Print("    INDUSTRY_BANKS_DIVERSIFIED: 	", INDUSTRY_BANKS_DIVERSIFIED);
    Print("    INDUSTRY_BANKS_REGIONAL: 	", INDUSTRY_BANKS_REGIONAL);
    Print("    INDUSTRY_CAPITAL_MARKETS: 	", INDUSTRY_CAPITAL_MARKETS);
    Print("    INDUSTRY_CLOSE_END_FUND_DEBT: 	", INDUSTRY_CLOSE_END_FUND_DEBT);
    Print("    INDUSTRY_CLOSE_END_FUND_EQUITY: 	", INDUSTRY_CLOSE_END_FUND_EQUITY);
    Print("    INDUSTRY_CLOSE_END_FUND_FOREIGN: 	", INDUSTRY_CLOSE_END_FUND_FOREIGN);
    Print("    INDUSTRY_CREDIT_SERVICES: 	", INDUSTRY_CREDIT_SERVICES);
    Print("    INDUSTRY_FINANCIAL_CONGLOMERATE: 	", INDUSTRY_FINANCIAL_CONGLOMERATE);
    Print("    INDUSTRY_FINANCIAL_DATA_EXCHANGE: 	", INDUSTRY_FINANCIAL_DATA_EXCHANGE);
    Print("    INDUSTRY_INSURANCE_BROKERS: 	", INDUSTRY_INSURANCE_BROKERS);
    Print("    INDUSTRY_INSURANCE_DIVERSIFIED: 	", INDUSTRY_INSURANCE_DIVERSIFIED);
    Print("    INDUSTRY_INSURANCE_LIFE: 	", INDUSTRY_INSURANCE_LIFE);
    Print("    INDUSTRY_INSURANCE_PROPERTY: 	", INDUSTRY_INSURANCE_PROPERTY);
    Print("    INDUSTRY_INSURANCE_REINSURANCE: 	", INDUSTRY_INSURANCE_REINSURANCE);
    Print("    INDUSTRY_INSURANCE_SPECIALTY: 	", INDUSTRY_INSURANCE_SPECIALTY);
    Print("    INDUSTRY_MORTGAGE_FINANCE: 	", INDUSTRY_MORTGAGE_FINANCE);
    Print("    INDUSTRY_SHELL_COMPANIES: 	", INDUSTRY_SHELL_COMPANIES);
    Print("    INDUSTRY_BIOTECHNOLOGY: 	", INDUSTRY_BIOTECHNOLOGY);
    Print("    INDUSTRY_DIAGNOSTICS_RESEARCH: 	", INDUSTRY_DIAGNOSTICS_RESEARCH);
    Print("    INDUSTRY_DRUGS_MANUFACTURERS: 	", INDUSTRY_DRUGS_MANUFACTURERS);
    Print("    INDUSTRY_DRUGS_MANUFACTURERS_SPEC: 	", INDUSTRY_DRUGS_MANUFACTURERS_SPEC);
    Print("    INDUSTRY_HEALTHCARE_PLANS: 	", INDUSTRY_HEALTHCARE_PLANS);
    Print("    INDUSTRY_HEALTH_INFORMATION: 	", INDUSTRY_HEALTH_INFORMATION);
    Print("    INDUSTRY_MEDICAL_FACILITIES: 	", INDUSTRY_MEDICAL_FACILITIES);
    Print("    INDUSTRY_MEDICAL_DEVICES: 	", INDUSTRY_MEDICAL_DEVICES);
    Print("    INDUSTRY_MEDICAL_DISTRIBUTION: 	", INDUSTRY_MEDICAL_DISTRIBUTION);
    Print("    INDUSTRY_MEDICAL_INSTRUMENTS: 	", INDUSTRY_MEDICAL_INSTRUMENTS);
    Print("    INDUSTRY_PHARM_RETAILERS: 	", INDUSTRY_PHARM_RETAILERS);
    Print("    INDUSTRY_AEROSPACE_DEFENSE: 	", INDUSTRY_AEROSPACE_DEFENSE);
    Print("    INDUSTRY_AIRLINES: 	", INDUSTRY_AIRLINES);
    Print("    INDUSTRY_AIRPORTS_SERVICES: 	", INDUSTRY_AIRPORTS_SERVICES);
    Print("    INDUSTRY_BUILDING_PRODUCTS: 	", INDUSTRY_BUILDING_PRODUCTS);
    Print("    INDUSTRY_BUSINESS_EQUIPMENT: 	", INDUSTRY_BUSINESS_EQUIPMENT);
    Print("    INDUSTRY_CONGLOMERATES: 	", INDUSTRY_CONGLOMERATES);
    Print("    INDUSTRY_CONSULTING_SERVICES: 	", INDUSTRY_CONSULTING_SERVICES);
    Print("    INDUSTRY_ELECTRICAL_EQUIPMENT: 	", INDUSTRY_ELECTRICAL_EQUIPMENT);
    Print("    INDUSTRY_ENGINEERING_CONSTRUCTION: 	", INDUSTRY_ENGINEERING_CONSTRUCTION);
    Print("    INDUSTRY_FARM_HEAVY_MACHINERY: 	", INDUSTRY_FARM_HEAVY_MACHINERY);
    Print("    INDUSTRY_INDUSTRIAL_DISTRIBUTION: 	", INDUSTRY_INDUSTRIAL_DISTRIBUTION);
    Print("    INDUSTRY_INFRASTRUCTURE_OPERATIONS: 	", INDUSTRY_INFRASTRUCTURE_OPERATIONS);
    Print("    INDUSTRY_FREIGHT_LOGISTICS: 	", INDUSTRY_FREIGHT_LOGISTICS);
    Print("    INDUSTRY_MARINE_SHIPPING: 	", INDUSTRY_MARINE_SHIPPING);
    Print("    INDUSTRY_METAL_FABRICATION: 	", INDUSTRY_METAL_FABRICATION);
    Print("    INDUSTRY_POLLUTION_CONTROL: 	", INDUSTRY_POLLUTION_CONTROL);
    Print("    INDUSTRY_RAILROADS: 	", INDUSTRY_RAILROADS);
    Print("    INDUSTRY_RENTAL_LEASING: 	", INDUSTRY_RENTAL_LEASING);
    Print("    INDUSTRY_SECURITY_PROTECTION: 	", INDUSTRY_SECURITY_PROTECTION);
    Print("    INDUSTRY_SPEALITY_BUSINESS_SERVICES: 	", INDUSTRY_SPEALITY_BUSINESS_SERVICES);
    Print("    INDUSTRY_SPEALITY_MACHINERY: 	", INDUSTRY_SPEALITY_MACHINERY);
    Print("    INDUSTRY_STUFFING_EMPLOYMENT: 	", INDUSTRY_STUFFING_EMPLOYMENT);
    Print("    INDUSTRY_TOOLS_ACCESSORIES: 	", INDUSTRY_TOOLS_ACCESSORIES);
    Print("    INDUSTRY_TRUCKING: 	", INDUSTRY_TRUCKING);
    Print("    INDUSTRY_WASTE_MANAGEMENT: 	", INDUSTRY_WASTE_MANAGEMENT);
    Print("    INDUSTRY_REAL_ESTATE_DEVELOPMENT: 	", INDUSTRY_REAL_ESTATE_DEVELOPMENT);
    Print("    INDUSTRY_REAL_ESTATE_DIVERSIFIED: 	", INDUSTRY_REAL_ESTATE_DIVERSIFIED);
    Print("    INDUSTRY_REAL_ESTATE_SERVICES: 	", INDUSTRY_REAL_ESTATE_SERVICES);
    Print("    INDUSTRY_REIT_DIVERSIFIED: 	", INDUSTRY_REIT_DIVERSIFIED);
    Print("    INDUSTRY_REIT_HEALTCARE: 	", INDUSTRY_REIT_HEALTCARE);
    Print("    INDUSTRY_REIT_HOTEL_MOTEL: 	", INDUSTRY_REIT_HOTEL_MOTEL);
    Print("    INDUSTRY_REIT_INDUSTRIAL: 	", INDUSTRY_REIT_INDUSTRIAL);
    Print("    INDUSTRY_REIT_MORTAGE: 	", INDUSTRY_REIT_MORTAGE);
    Print("    INDUSTRY_REIT_OFFICE: 	", INDUSTRY_REIT_OFFICE);
    Print("    INDUSTRY_REIT_RESIDENTAL: 	", INDUSTRY_REIT_RESIDENTAL);
    Print("    INDUSTRY_REIT_RETAIL: 	", INDUSTRY_REIT_RETAIL);
    Print("    INDUSTRY_REIT_SPECIALITY: 	", INDUSTRY_REIT_SPECIALITY);
    Print("    INDUSTRY_COMMUNICATION_EQUIPMENT: 	", INDUSTRY_COMMUNICATION_EQUIPMENT);
    Print("    INDUSTRY_COMPUTER_HARDWARE: 	", INDUSTRY_COMPUTER_HARDWARE);
    Print("    INDUSTRY_CONSUMER_ELECTRONICS: 	", INDUSTRY_CONSUMER_ELECTRONICS);
    Print("    INDUSTRY_ELECTRONIC_COMPONENTS: 	", INDUSTRY_ELECTRONIC_COMPONENTS);
    Print("    INDUSTRY_ELECTRONIC_DISTRIBUTION: 	", INDUSTRY_ELECTRONIC_DISTRIBUTION);
    Print("    INDUSTRY_IT_SERVICES: 	", INDUSTRY_IT_SERVICES);
    Print("    INDUSTRY_SCIENTIFIC_INSTRUMENTS: 	", INDUSTRY_SCIENTIFIC_INSTRUMENTS);
    Print("    INDUSTRY_SEMICONDUCTOR_EQUIPMENT: 	", INDUSTRY_SEMICONDUCTOR_EQUIPMENT);
    Print("    INDUSTRY_SEMICONDUCTORS: 	", INDUSTRY_SEMICONDUCTORS);
    Print("    INDUSTRY_SOFTWARE_APPLICATION: 	", INDUSTRY_SOFTWARE_APPLICATION);
    Print("    INDUSTRY_SOFTWARE_INFRASTRUCTURE: 	", INDUSTRY_SOFTWARE_INFRASTRUCTURE);
    Print("    INDUSTRY_SOLAR: 	", INDUSTRY_SOLAR);
    Print("    INDUSTRY_UTILITIES_DIVERSIFIED: 	", INDUSTRY_UTILITIES_DIVERSIFIED);
    Print("    INDUSTRY_UTILITIES_POWERPRODUCERS: 	", INDUSTRY_UTILITIES_POWERPRODUCERS);
    Print("    INDUSTRY_UTILITIES_RENEWABLE: 	", INDUSTRY_UTILITIES_RENEWABLE);
    Print("    INDUSTRY_UTILITIES_REGULATED_ELECTRIC: 	", INDUSTRY_UTILITIES_REGULATED_ELECTRIC);
    Print("    INDUSTRY_UTILITIES_REGULATED_GAS: 	", INDUSTRY_UTILITIES_REGULATED_GAS);
    Print("    INDUSTRY_UTILITIES_REGULATED_WATER: 	", INDUSTRY_UTILITIES_REGULATED_WATER);*/
    
    Print("ENUM_SYMBOL_CALC_MODE:");
    Print("SYMBOL_CALC_MODE_FOREX: 	", SYMBOL_CALC_MODE_FOREX);
    Print("SYMBOL_CALC_MODE_FOREX_NO_LEVERAGE: 	", SYMBOL_CALC_MODE_FOREX_NO_LEVERAGE);
    Print("SYMBOL_CALC_MODE_FUTURES: 	", SYMBOL_CALC_MODE_FUTURES);
    Print("SYMBOL_CALC_MODE_CFD: 	", SYMBOL_CALC_MODE_CFD);
    Print("SYMBOL_CALC_MODE_CFDINDEX: 	", SYMBOL_CALC_MODE_CFDINDEX);
    Print("SYMBOL_CALC_MODE_CFDLEVERAGE: 	", SYMBOL_CALC_MODE_CFDLEVERAGE);
    Print("SYMBOL_CALC_MODE_EXCH_STOCKS: 	", SYMBOL_CALC_MODE_EXCH_STOCKS);
    Print("SYMBOL_CALC_MODE_EXCH_FUTURES: 	", SYMBOL_CALC_MODE_EXCH_FUTURES);
    Print("SYMBOL_CALC_MODE_EXCH_FUTURES_FORTS: 	", SYMBOL_CALC_MODE_EXCH_FUTURES_FORTS);
    Print("SYMBOL_CALC_MODE_EXCH_BONDS: 	", SYMBOL_CALC_MODE_EXCH_BONDS);
    Print("SYMBOL_CALC_MODE_EXCH_STOCKS_MOEX: 	", SYMBOL_CALC_MODE_EXCH_STOCKS_MOEX);
    Print("SYMBOL_CALC_MODE_EXCH_BONDS_MOEX: 	", SYMBOL_CALC_MODE_EXCH_BONDS_MOEX);
    Print("SYMBOL_CALC_MODE_SERV_COLLATERAL: 	", SYMBOL_CALC_MODE_SERV_COLLATERAL);


    test_name = "Testing SymbolInfoBridge serialization...";
    SymbolInfoBridge symbol_info_bridge;
    symbol_info_bridge.symbol_sector = SECTOR_UNDEFINED;
    symbol_info_bridge.symbol_industry = INDUSTRY_UNDEFINED;
    symbol_info_bridge.symbol_background_color = 8421504;   // #808080
    symbol_info_bridge.symbol_chart_mode = SYMBOL_CHART_MODE_LAST;
    symbol_info_bridge.symbol_session_deals = 547;
    symbol_info_bridge.symbol_session_buy_orders = 745;
    symbol_info_bridge.symbol_session_sell_orders = 574;
    symbol_info_bridge.symbol_volume = 112233;
    symbol_info_bridge.symbol_volumehigh = 332211;
    symbol_info_bridge.symbol_volumelow = 111111;
    symbol_info_bridge.symbol_time = 987654321;
    symbol_info_bridge.symbol_time_msc = 987654321001;
    symbol_info_bridge.symbol_digits = 3;
    symbol_info_bridge.symbol_spread = 47;
    symbol_info_bridge.symbol_ticks_bookdepth = 74;
    symbol_info_bridge.symbol_trade_calc_mode = SYMBOL_CALC_MODE_CFD;
    symbol_info_bridge.symbol_trade_mode = SYMBOL_TRADE_MODE_FULL;
    symbol_info_bridge.symbol_start_time = 7447;
    symbol_info_bridge.symbol_expiration_time = 32777;
    symbol_info_bridge.symbol_trade_stops_level = 9182;
    symbol_info_bridge.symbol_trade_freeze_level = 1928;
    symbol_info_bridge.symbol_trade_exemode = SYMBOL_TRADE_EXECUTION_MARKET;
    symbol_info_bridge.symbol_swap_mode = SYMBOL_SWAP_MODE_INTEREST_OPEN;
    symbol_info_bridge.symbol_swap_rollover3days = FRIDAY;
    symbol_info_bridge.symbol_expiration_mode  = 1982;
    symbol_info_bridge.symbol_filling_mode = 1289;
    symbol_info_bridge.symbol_order_mode = 9821;
    symbol_info_bridge.symbol_order_gtc_mode = SYMBOL_ORDERS_DAILY_EXCLUDING_STOPS;
    symbol_info_bridge.symbol_option_mode = SYMBOL_OPTION_MODE_AMERICAN;
    symbol_info_bridge.symbol_option_right = SYMBOL_OPTION_RIGHT_PUT;
    symbol_info_bridge.symbol_bid = 3.14159;
    symbol_info_bridge.symbol_bidhigh = 3.14160;
    symbol_info_bridge.symbol_bidlow = 3.14161;
    symbol_info_bridge.symbol_ask = 3.14162;
    symbol_info_bridge.symbol_askhigh = 3.14163;
    symbol_info_bridge.symbol_asklow = 3.14164;
    symbol_info_bridge.symbol_last = 3.14165;
    symbol_info_bridge.symbol_lasthigh = 3.14166;
    symbol_info_bridge.symbol_lastlow = 3.14167;
    symbol_info_bridge.symbol_volume_real = 3.14168;
    symbol_info_bridge.symbol_volumehigh_real = 3.14169;
    symbol_info_bridge.symbol_volumelow_real = 3.14170;
    symbol_info_bridge.symbol_option_strike = 3.14171;
    symbol_info_bridge.symbol_point = 3.14172;
    symbol_info_bridge.symbol_trade_tick_value = 3.14173;
    symbol_info_bridge.symbol_trade_tick_value_profit = 3.14174;
    symbol_info_bridge.symbol_trade_tick_value_loss = 3.14175;
    symbol_info_bridge.symbol_trade_tick_size = 3.14176;
    symbol_info_bridge.symbol_trade_contract_size = 3.14177;
    symbol_info_bridge.symbol_trade_accrued_interest = 3.14178;
    symbol_info_bridge.symbol_trade_face_value = 3.14179;
    symbol_info_bridge.symbol_trade_liquidity_rate = 3.14180;
    symbol_info_bridge.symbol_volume_min = 3.14181;
    symbol_info_bridge.symbol_volume_max = 3.14182;
    symbol_info_bridge.symbol_volume_step = 3.14183;
    symbol_info_bridge.symbol_volume_limit = 3.14184;
    symbol_info_bridge.symbol_swap_long = 3.14185;
    symbol_info_bridge.symbol_swap_short = 3.14186;
    symbol_info_bridge.symbol_swap_sunday = 3.14187;
    symbol_info_bridge.symbol_swap_monday = 3.14188;
    symbol_info_bridge.symbol_swap_tuesday = 3.14189;
    symbol_info_bridge.symbol_swap_wednesday = 3.14190;
    symbol_info_bridge.symbol_swap_thursday = 3.14191;
    symbol_info_bridge.symbol_swap_friday = 3.14192;
    symbol_info_bridge.symbol_swap_saturday = 3.14193;
    symbol_info_bridge.symbol_margin_initial = 3.14194;
    symbol_info_bridge.symbol_margin_maintenance = 3.14195;
    symbol_info_bridge.symbol_session_volume = 3.14196;
    symbol_info_bridge.symbol_session_turnover = 3.14197;
    symbol_info_bridge.symbol_session_interest = 3.14198;
    symbol_info_bridge.symbol_session_buy_orders_volume = 3.14199;
    symbol_info_bridge.symbol_session_sell_orders_volume = 3.14200;
    symbol_info_bridge.symbol_session_open = 3.14201;
    symbol_info_bridge.symbol_session_close = 3.14202;
    symbol_info_bridge.symbol_session_aw = 3.14203;
    symbol_info_bridge.symbol_session_price_settlement = 3.14204;
    symbol_info_bridge.symbol_session_price_limit_min = 3.14205;
    symbol_info_bridge.symbol_session_price_limit_max = 3.14206;
    symbol_info_bridge.symbol_margin_hedged = 3.14207;
    symbol_info_bridge.symbol_price_change = 3.14208;
    symbol_info_bridge.symbol_price_volatility = 3.14209;
    symbol_info_bridge.symbol_price_theoretical = 3.14210;
    symbol_info_bridge.symbol_price_delta = 3.14211;
    symbol_info_bridge.symbol_price_theta = 3.14212;
    symbol_info_bridge.symbol_price_gamma = 3.14213;
    symbol_info_bridge.symbol_price_vega = 3.14214;
    symbol_info_bridge.symbol_price_rho = 3.14215;
    symbol_info_bridge.symbol_price_omega = 3.14216;
    symbol_info_bridge.symbol_price_sensitivity = 3.14217;
    symbol_info_bridge.symbol_basis = "a";
    symbol_info_bridge.symbol_category = "b";
    symbol_info_bridge.symbol_country = "c";
    symbol_info_bridge.symbol_sector_name = "d";
    symbol_info_bridge.symbol_industry_name = "e";
    symbol_info_bridge.symbol_currency_base = "f";
    symbol_info_bridge.symbol_currency_profit = "g";
    symbol_info_bridge.symbol_currency_margin = "h";
    symbol_info_bridge.symbol_bank = "i";
    symbol_info_bridge.symbol_description = "j";
    symbol_info_bridge.symbol_exchange = "k";
    symbol_info_bridge.symbol_formula = "l";
    symbol_info_bridge.symbol_isin = "m";
    symbol_info_bridge.symbol_page = "n";
    symbol_info_bridge.symbol_path = "o";
    symbol_info_bridge.symbol_subscription_delay = true;
    symbol_info_bridge.symbol_custom = false;
    symbol_info_bridge.symbol_exist = true;
    symbol_info_bridge.symbol_select = false;
    symbol_info_bridge.symbol_visible = true;
    symbol_info_bridge.symbol_spread_float = false;
    symbol_info_bridge.symbol_margin_hedged_use_leg = true;
    expected = "SymbolInfoRust { " +
    "symbol_sector: SectorUndefined, " +
    "symbol_industry: IndustryUndefined, " +
    "symbol_background_color: (128, 128, 128), " +
    "symbol_chart_mode: SymbolChartModeLast, " +
    "symbol_session_deals: 547, " +
    "symbol_session_buy_orders: 745, " +
    "symbol_session_sell_orders: 574, " +
    "symbol_volume: 112233, " +
    "symbol_volumehigh: 332211, " +
    "symbol_volumelow: 111111, " +
    "symbol_time: 2001-04-19T04:25:21.001, " +
    "symbol_digits: 3, " +
    "symbol_spread: 47, " +
    "symbol_ticks_bookdepth: 74, " +
    "symbol_trade_calc_mode: SymbolCalcModeFutures, " +
    "symbol_trade_mode: SymbolTradeModeFull, " +
    "symbol_start_time: 1970-01-01T02:04:07, " +
    "symbol_expiration_time: 1970-01-01T09:06:17, " +
    "symbol_trade_stops_level: 9182, " +
    "symbol_trade_freeze_level: 1928, " +
    "symbol_trade_exemode: SymbolTradeExecutionMarket, " +
    "symbol_swap_mode: SymbolSwapModeInterestOpen, " +
    "symbol_swap_rollover3days: Friday, " +
    "symbol_expiration_mode: 1982, " +
    "symbol_filling_mode: 1289, " +
    "symbol_order_mode: 9821, " +
    "symbol_order_gtc_mode: SymbolOrdersDailyExcludingStops, " +
    "symbol_option_mode: SymbolOptionModeAmerican, " +
    "symbol_option_right: SymbolOptionRightPut, " +
    "symbol_bid: 3.14159, " +
    "symbol_bidhigh: 3.1416, " +
    "symbol_bidlow: 3.14161, " +
    "symbol_ask: 3.14162, " +
    "symbol_askhigh: 3.14163, " +
    "symbol_asklow: 3.14164, " +
    "symbol_last: 3.14165, " +
    "symbol_lasthigh: 3.14166, " +
    "symbol_lastlow: 3.14167, " +
    "symbol_volume_real: 3.14168, " +
    "symbol_volumehigh_real: 3.14169, " +
    "symbol_volumelow_real: 3.1417, " +
    "symbol_option_strike: 3.14171, " +
    "symbol_point: 3.14172, " +
    "symbol_trade_tick_value: 3.14173, " +
    "symbol_trade_tick_value_profit: 3.14174, " +
    "symbol_trade_tick_value_loss: 3.14175, " +
    "symbol_trade_tick_size: 3.14176, " +
    "symbol_trade_contract_size: 3.14177, " +
    "symbol_trade_accrued_interest: 3.14178, " +
    "symbol_trade_face_value: 3.14179, " +
    "symbol_trade_liquidity_rate: 3.1418, " +
    "symbol_volume_min: 3.14181, " +
    "symbol_volume_max: 3.14182, " +
    "symbol_volume_step: 3.14183, " +
    "symbol_volume_limit: 3.14184, " +
    "symbol_swap_long: 3.14185, " +
    "symbol_swap_short: 3.14186, " +
    "symbol_swap_sunday: 3.14187, " +
    "symbol_swap_monday: 3.14188, " +
    "symbol_swap_tuesday: 3.14189, " +
    "symbol_swap_wednesday: 3.1419, " +
    "symbol_swap_thursday: 3.14191, " +
    "symbol_swap_friday: 3.14192, " +
    "symbol_swap_saturday: 3.14193, " +
    "symbol_margin_initial: 3.14194, " +
    "symbol_margin_maintenance: 3.14195, " +
    "symbol_session_volume: 3.14196, " +
    "symbol_session_turnover: 3.14197, " +
    "symbol_session_interest: 3.14198, " +
    "symbol_session_buy_orders_volume: 3.14199, " +
    "symbol_session_sell_orders_volume: 3.142, " +
    "symbol_session_open: 3.14201, " +
    "symbol_session_close: 3.14202, " +
    "symbol_session_aw: 3.14203, " +
    "symbol_session_price_settlement: 3.14204, " +
    "symbol_session_price_limit_min: 3.14205, " +
    "symbol_session_price_limit_max: 3.14206, " +
    "symbol_margin_hedged: 3.14207, " +
    "symbol_price_change: 3.14208, " +
    "symbol_price_volatility: 3.14209, " +
    "symbol_price_theoretical: 3.1421, " +
    "symbol_price_delta: 3.14211, " +
    "symbol_price_theta: 3.14212, " +
    "symbol_price_gamma: 3.14213, " +
    "symbol_price_vega: 3.14214, " +
    "symbol_price_rho: 3.14215, " +
    "symbol_price_omega: 3.14216, " +
    "symbol_price_sensitivity: 3.14217, " +
    "symbol_basis: \"a\", " +
    "symbol_category: \"b\", " +
    "symbol_country: \"c\", " +
    "symbol_sector_name: \"d\", " +
    "symbol_industry_name: \"e\", " +
    "symbol_currency_base: \"f\", " +
    "symbol_currency_profit: \"g\", " +
    "symbol_currency_margin: \"h\", " +
    "symbol_bank: \"i\", " +
    "symbol_description: \"j\", " +
    "symbol_exchange: \"k\", " +
    "symbol_formula: \"l\", " +
    "symbol_isin: \"m\", " +
    "symbol_page: \"n\", " +
    "symbol_path: \"o\", " +
    "symbol_subscription_delay: true, " +
    "symbol_custom: false, " +
    "symbol_exist: true, " +
    "symbol_select: false, " +
    "symbol_visible: true, " +
    "symbol_spread_float: false, " +
    "symbol_margin_hedged_use_leg: true }";
    test_report_symbol_info(observed, symbol_info_bridge);
    assert(observed, expected, test_name);

    test_name = "Testing AccountInfoBridge serialization...";
    AccountInfoBridge account_info_bridge;
    account_info_bridge.account_balance = 0;
    account_info_bridge.account_credit = 0;
    account_info_bridge.account_profit = 0;
    account_info_bridge.account_equity = 0;
    account_info_bridge.account_margin = 0;
    account_info_bridge.account_margin_free = 0;
    account_info_bridge.account_margin_level = 0;
    account_info_bridge.account_margin_so_call = 0;
    account_info_bridge.account_margin_so_so = 0;
    account_info_bridge.account_margin_initial = 0;
    account_info_bridge.account_margin_maintenance = 0;
    account_info_bridge.account_assets = 0;
    account_info_bridge.account_liabilities = 0;
    account_info_bridge.account_commission_blocked = 0;
    account_info_bridge.account_login = 0;
    account_info_bridge.account_leverage = 0;
    account_info_bridge.account_name = 0;
    account_info_bridge.account_server = 0;
    account_info_bridge.account_currency = 0;
    account_info_bridge.account_company = 0;
    account_info_bridge.account_trade_mode = 0;
    account_info_bridge.account_limit_orders = 0;
    account_info_bridge.account_margin_so_mode = 0;
    account_info_bridge.account_margin_mode = 0;
    account_info_bridge.account_currency_digits = 0;
    account_info_bridge.account_trade_allowed = 0;
    account_info_bridge.account_trade_expert = 0;
    account_info_bridge.account_fifo_close = 0;
    account_info_bridge.account_hedge_allowed = 0;
    expected = "AccountInfoBridge { // some values // }";
    test_report_account_info(observed, account_info_bridge);
    assert(observed, expected, test_name);


    test_name = "Testing DealPropertiesBridge serialization...";
    DealPropertiesBridge deal_properties_bridge;
    deal_properties_bridge.deal_volume = 0;
    deal_properties_bridge.deal_price = 0;
    deal_properties_bridge.deal_commission = 0;
    deal_properties_bridge.deal_swap = 0;
    deal_properties_bridge.deal_profit = 0;
    deal_properties_bridge.deal_fee = 0;
    deal_properties_bridge.deal_sl = 0;
    deal_properties_bridge.deal_tp = 0;
    deal_properties_bridge.deal_symbol = 0;
    deal_properties_bridge.deal_comment = 0;
    deal_properties_bridge.deal_external_id = 0;
    deal_properties_bridge.deal_ticket = 0;
    deal_properties_bridge.deal_order = 0;
    deal_properties_bridge.deal_time_msc = 0;
    deal_properties_bridge.deal_magic = 0;
    deal_properties_bridge.deal_position_id = 0;
    deal_properties_bridge.deal_time = 0;
    deal_properties_bridge.deal_type = 0;
    deal_properties_bridge.deal_entry = 0;
    deal_properties_bridge.deal_reason = 0;
    expected = "DealPropertiesBridge { // some values // }";
    test_report_deal_properties(observed, deal_properties_bridge);
    assert(observed, expected, test_name);

    test_name = "Testing 'MqlTradeTransaction', 'MqlTradeRequest', & 'MqlTradeResult' serialization for OnTradeTransaction()...";
    MqlTradeTransaction trade_transaction;
    trade_transaction.deal = 0;
    trade_transaction.order = 0;
    trade_transaction.symbol = 0;
    trade_transaction.type = 0;
    trade_transaction.order_type = 0;
    trade_transaction.order_state = 0;
    trade_transaction.deal_type = 0;
    trade_transaction.time_type = 0;
    trade_transaction.time_expiration = 0;
    trade_transaction.price = 0;
    trade_transaction.price_trigger = 0;
    trade_transaction.price_sl = 0;
    trade_transaction.price_tp = 0;
    trade_transaction.volume = 0;
    trade_transaction.position = 0;
    trade_transaction.position_by = 0;
    MqlTradeRequest trade_request;
    trade_request.action = TRADE_ACTION_DEAL;
    trade_request.magic = 0;
    trade_request.order = 0;
    trade_request.symbol = 0;
    trade_request.volume = 0;
    trade_request.price = 0;
    trade_request.stoplimit = 0;
    trade_request.sl = 0;
    trade_request.tp = 0;
    trade_request.deviation = 0;
    trade_request.type = 0;
    trade_request.type_filling = 0;
    trade_request.type_time = 0;
    trade_request.expiration = 0;
    trade_request.comment = 0;
    trade_request.position = 0;
    trade_request.position_by = 0;
    MqlTradeResult trade_result;
    trade_result.retcode = 0;
    trade_result.deal = 0;
    trade_result.order = 0;
    trade_result.volume = 0;
    trade_result.price = 0;
    trade_result.bid = 0;
    trade_result.ask = 0;
    trade_result.comment = 0;
    trade_result.request_id = 0;
    trade_result.retcode_external = 0;
    expected = "MqlTradeTransaction { // some values // }; MqlTradeRequest { // some other values// }; MqlTradeResult { // yet some other values // }";
    test_on_trade_transaction(observed, trade_transaction, trade_request, trade_result);
    assert(observed, expected, test_name);

    test_name = "Testing 'MqlBookInfo' serialization...";
    MarketBookAdd(_Symbol);
    Sleep(1500);
    MqlBookInfo  book_info[];
    MarketBookGet(_Symbol, book_info);
    Print("book_info len is " + ArraySize(book_info));
    test_on_book(observed, book_info, ArraySize(book_info));
    Print("Returned string length is " + observed.Length());
    Print("    & buffer is " + observed.BufferSize());
    Print(observed);
    
    Print("Verify ENUM values for 'ENUM_BOOK_TYPE'-- assure those values start with 0:");
    Print("BOOK_TYPE_SELL="+BOOK_TYPE_SELL);
    Print("BOOK_TYPE_BUY="+BOOK_TYPE_BUY);
    Print("BOOK_TYPE_SELL_MARKET="+BOOK_TYPE_SELL_MARKET);
    Print("BOOK_TYPE_BUY_MARKET="+BOOK_TYPE_BUY_MARKET);
    
    Print("ENUM_DAY_OF_THE_WEEK::SUNDAY = " + SUNDAY);
    Print("ENUM_SYMBOL_CALC_MODE::SYMBOL_CALC_MODE_FOREX = " + SYMBOL_CALC_MODE_FOREX);
    Print("ENUM_SYMBOL_CHART_MODE::SYMBOL_CHART_MODE_BID = " + SYMBOL_CHART_MODE_BID);
    Print("ENUM_SYMBOL_INDUSTRY::INDUSTRY_UNDEFINED = " + INDUSTRY_UNDEFINED);
    Print("ENUM_SYMBOL_OPTION_MODE::SYMBOL_OPTION_MODE_EUROPEAN = " + SYMBOL_OPTION_MODE_EUROPEAN);
    Print("ENUM_SYMBOL_OPTION_RIGHT::SYMBOL_OPTION_RIGHT_CALL = " + SYMBOL_OPTION_RIGHT_CALL);
    Print("ENUM_SYMBOL_ORDER_GTC_MODE::SYMBOL_ORDERS_GTC = " + SYMBOL_ORDERS_GTC);
    Print("ENUM_SYMBOL_SECTOR::SECTOR_UNDEFINED = " + SECTOR_UNDEFINED);
    Print("ENUM_SYMBOL_SWAP_MODE::SYMBOL_SWAP_MODE_DISABLED = " + SYMBOL_SWAP_MODE_DISABLED);
    Print("ENUM_SYMBOL_TRADE_EXECUTION::SYMBOL_TRADE_EXECUTION_REQUEST = " + SYMBOL_TRADE_EXECUTION_REQUEST);
    Print("ENUM_SYMBOL_TRADE_MODE::SYMBOL_TRADE_MODE_DISABLED = " + SYMBOL_TRADE_MODE_DISABLED);
    Print("ENUM_ACCOUNT_MARGIN_MODE::ACCOUNT_MARGIN_MODE_RETAIL_NETTING = " + ACCOUNT_MARGIN_MODE_RETAIL_NETTING);
    Print("ENUM_ACCOUNT_STOPOUT_MODE::ACCOUNT_STOPOUT_MODE_PERCENT = " + ACCOUNT_STOPOUT_MODE_PERCENT);
    Print("ENUM_ACCOUNT_TRADE_MODE_DEMO::ACCOUNT_TRADE_MODE_DEMO = " + ACCOUNT_TRADE_MODE_DEMO);
    Print("ENUM_DEAL_ENTRY::DEAL_ENTRY_IN = " + DEAL_ENTRY_IN);
    Print("ENUM_DEAL_REASON::DEAL_REASON_CLIENT = " + DEAL_REASON_CLIENT);
    Print("ENUM_DEAL_TYPE::DEAL_TYPE_BUY = " + DEAL_TYPE_BUY);

    Print("EnumTradeTransactionType::TRADE_TRANSACTION_ORDER_ADD = " + TRADE_TRANSACTION_ORDER_ADD);
    Print("EnumOrderType::ORDER_TYPE_BUY = " + ORDER_TYPE_BUY);
    Print("EnumOrderState::ORDER_STATE_STARTED = " + ORDER_STATE_STARTED);
    Print("EnumDealType::DEAL_TYPE_BUY = " + DEAL_TYPE_BUY);
    Print("EnumOrderTypeTime::ORDER_TIME_GTC = " + ORDER_TIME_GTC);
    Print("EnumOrderTypeFilling::ORDER_FILLING_FOK = " + ORDER_FILLING_FOK);
    Print("EnumTradeRequestActions::TRADE_ACTION_DEAL = " + TRADE_ACTION_DEAL);
    Print("EnumOrderType::ORDER_TYPE_BUY = " + ORDER_TYPE_BUY);


}

void expose_constant(string constant_name, long constant_value, string comment) {
    Print("/// " + comment);
    Print("pub const " + constant_name + ": u32 = " + constant_value + ";");
}

void assert(string observed, string expected, string message) {
    Print("");
    if (observed == expected) {
        Print(message + " OK");
    } else {
        Print(message + " FAILED");
        Print("  observed ( "+observed.Length()+"chars): '"+observed+"'");
        Print("  expected:( "+expected.Length()+"chars) '"+expected+"'");
    }
}