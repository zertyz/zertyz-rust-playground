// This should be executed once when the DLL is loaded
// (usually, when handle_id 0 is given out by Rust)
// In case any enum variant fails to register, the MQL Program must quit to avoid undefined behavior
// -- and, in this case, attempts to start other programs (with other handle_ids) must also fail, as
//    the Rust binary vs the MQL Program versions are out of sync

set_enum_variant_value("EnumDayOfWeek", "Sunday", SUNDAY);
set_enum_variant_value("EnumDayOfWeek", "Monday", MONDAY);
set_enum_variant_value("EnumDayOfWeek", "Tuesday", TUESDAY);
set_enum_variant_value("EnumDayOfWeek", "Wednesday", WEDNESDAY);
set_enum_variant_value("EnumDayOfWeek", "Thursday", THURSDAY);
set_enum_variant_value("EnumDayOfWeek", "Friday", FRIDAY);
set_enum_variant_value("EnumDayOfWeek", "Saturday", SATURDAY);
set_enum_variant_value("EnumSymbolCalcMode", "SymbolCalcModeForex", SYMBOL_CALC_MODE_FOREX);
set_enum_variant_value("EnumSymbolCalcMode", "SymbolCalcModeForexNoLeverage", SYMBOL_CALC_MODE_FOREX_NO_LEVERAGE);
set_enum_variant_value("EnumSymbolCalcMode", "SymbolCalcModeFutures", SYMBOL_CALC_MODE_FUTURES);
set_enum_variant_value("EnumSymbolCalcMode", "SymbolCalcModeCfd", SYMBOL_CALC_MODE_CFD);
set_enum_variant_value("EnumSymbolCalcMode", "SymbolCalcModeCfdindex", SYMBOL_CALC_MODE_CFDINDEX);
set_enum_variant_value("EnumSymbolCalcMode", "SymbolCalcModeCfdleverage", SYMBOL_CALC_MODE_CFDLEVERAGE);
set_enum_variant_value("EnumSymbolCalcMode", "SymbolCalcModeExchStocks", SYMBOL_CALC_MODE_EXCH_STOCKS);
set_enum_variant_value("EnumSymbolCalcMode", "SymbolCalcModeExchFutures", SYMBOL_CALC_MODE_EXCH_FUTURES);
set_enum_variant_value("EnumSymbolCalcMode", "SymbolCalcModeExchFuturesForts", SYMBOL_CALC_MODE_EXCH_FUTURES_FORTS);
set_enum_variant_value("EnumSymbolCalcMode", "SymbolCalcModeExchBonds", SYMBOL_CALC_MODE_EXCH_BONDS);
set_enum_variant_value("EnumSymbolCalcMode", "SymbolCalcModeExchStocksMoex", SYMBOL_CALC_MODE_EXCH_STOCKS_MOEX);
set_enum_variant_value("EnumSymbolCalcMode", "SymbolCalcModeExchBondsMoex", SYMBOL_CALC_MODE_EXCH_BONDS_MOEX);
set_enum_variant_value("EnumSymbolCalcMode", "SymbolCalcModeServCollateral", SYMBOL_CALC_MODE_SERV_COLLATERAL);
set_enum_variant_value("EnumSymbolChartMode", "SymbolChartModeBid", SYMBOL_CHART_MODE_BID);
set_enum_variant_value("EnumSymbolChartMode", "SymbolChartModeLast", SYMBOL_CHART_MODE_LAST);
set_enum_variant_value("EnumSymbolIndustry", "IndustryUndefined", INDUSTRY_UNDEFINED);
set_enum_variant_value("EnumSymbolIndustry", "IndustryAgriculturalInputs", INDUSTRY_AGRICULTURAL_INPUTS);
set_enum_variant_value("EnumSymbolIndustry", "IndustryAluminium", INDUSTRY_ALUMINIUM);
set_enum_variant_value("EnumSymbolIndustry", "IndustryBuildingMaterials", INDUSTRY_BUILDING_MATERIALS);
set_enum_variant_value("EnumSymbolIndustry", "IndustryChemicals", INDUSTRY_CHEMICALS);
set_enum_variant_value("EnumSymbolIndustry", "IndustryCokingCoal", INDUSTRY_COKING_COAL);
set_enum_variant_value("EnumSymbolIndustry", "IndustryCopper", INDUSTRY_COPPER);
set_enum_variant_value("EnumSymbolIndustry", "IndustryGold", INDUSTRY_GOLD);
set_enum_variant_value("EnumSymbolIndustry", "IndustryLumberWood", INDUSTRY_LUMBER_WOOD);
set_enum_variant_value("EnumSymbolIndustry", "IndustryIndustrialMetals", INDUSTRY_INDUSTRIAL_METALS);
set_enum_variant_value("EnumSymbolIndustry", "IndustryPreciousMetals", INDUSTRY_PRECIOUS_METALS);
set_enum_variant_value("EnumSymbolIndustry", "IndustryPaper", INDUSTRY_PAPER);
set_enum_variant_value("EnumSymbolIndustry", "IndustrySilver", INDUSTRY_SILVER);
set_enum_variant_value("EnumSymbolIndustry", "IndustrySpecialtyChemicals", INDUSTRY_SPECIALTY_CHEMICALS);
set_enum_variant_value("EnumSymbolIndustry", "IndustrySteel", INDUSTRY_STEEL);
set_enum_variant_value("EnumSymbolIndustry", "IndustryAdvertising", INDUSTRY_ADVERTISING);
set_enum_variant_value("EnumSymbolIndustry", "IndustryBroadcasting", INDUSTRY_BROADCASTING);
set_enum_variant_value("EnumSymbolIndustry", "IndustryGamingMultimedia", INDUSTRY_GAMING_MULTIMEDIA);
set_enum_variant_value("EnumSymbolIndustry", "IndustryEntertainment", INDUSTRY_ENTERTAINMENT);
set_enum_variant_value("EnumSymbolIndustry", "IndustryInternetContent", INDUSTRY_INTERNET_CONTENT);
set_enum_variant_value("EnumSymbolIndustry", "IndustryPublishing", INDUSTRY_PUBLISHING);
set_enum_variant_value("EnumSymbolIndustry", "IndustryTelecom", INDUSTRY_TELECOM);
set_enum_variant_value("EnumSymbolIndustry", "IndustryApparelManufacturing", INDUSTRY_APPAREL_MANUFACTURING);
set_enum_variant_value("EnumSymbolIndustry", "IndustryApparelRetail", INDUSTRY_APPAREL_RETAIL);
set_enum_variant_value("EnumSymbolIndustry", "IndustryAutoManufacturers", INDUSTRY_AUTO_MANUFACTURERS);
set_enum_variant_value("EnumSymbolIndustry", "IndustryAutoParts", INDUSTRY_AUTO_PARTS);
set_enum_variant_value("EnumSymbolIndustry", "IndustryAutoDealership", INDUSTRY_AUTO_DEALERSHIP);
set_enum_variant_value("EnumSymbolIndustry", "IndustryDepartmentStores", INDUSTRY_DEPARTMENT_STORES);
set_enum_variant_value("EnumSymbolIndustry", "IndustryFootwearAccessories", INDUSTRY_FOOTWEAR_ACCESSORIES);
set_enum_variant_value("EnumSymbolIndustry", "IndustryFurnishings", INDUSTRY_FURNISHINGS);
set_enum_variant_value("EnumSymbolIndustry", "IndustryGambling", INDUSTRY_GAMBLING);
set_enum_variant_value("EnumSymbolIndustry", "IndustryHomeImprovRetail", INDUSTRY_HOME_IMPROV_RETAIL);
set_enum_variant_value("EnumSymbolIndustry", "IndustryInternetRetail", INDUSTRY_INTERNET_RETAIL);
set_enum_variant_value("EnumSymbolIndustry", "IndustryLeisure", INDUSTRY_LEISURE);
set_enum_variant_value("EnumSymbolIndustry", "IndustryLodging", INDUSTRY_LODGING);
set_enum_variant_value("EnumSymbolIndustry", "IndustryLuxuryGoods", INDUSTRY_LUXURY_GOODS);
set_enum_variant_value("EnumSymbolIndustry", "IndustryPackagingContainers", INDUSTRY_PACKAGING_CONTAINERS);
set_enum_variant_value("EnumSymbolIndustry", "IndustryPersonalServices", INDUSTRY_PERSONAL_SERVICES);
set_enum_variant_value("EnumSymbolIndustry", "IndustryRecreationalVehicles", INDUSTRY_RECREATIONAL_VEHICLES);
set_enum_variant_value("EnumSymbolIndustry", "IndustryResidentConstruction", INDUSTRY_RESIDENT_CONSTRUCTION);
set_enum_variant_value("EnumSymbolIndustry", "IndustryResortsCasinos", INDUSTRY_RESORTS_CASINOS);
set_enum_variant_value("EnumSymbolIndustry", "IndustryRestaurants", INDUSTRY_RESTAURANTS);
set_enum_variant_value("EnumSymbolIndustry", "IndustrySpecialtyRetail", INDUSTRY_SPECIALTY_RETAIL);
set_enum_variant_value("EnumSymbolIndustry", "IndustryTextileManufacturing", INDUSTRY_TEXTILE_MANUFACTURING);
set_enum_variant_value("EnumSymbolIndustry", "IndustryTravelServices", INDUSTRY_TRAVEL_SERVICES);
set_enum_variant_value("EnumSymbolIndustry", "IndustryBeveragesBrewers", INDUSTRY_BEVERAGES_BREWERS);
set_enum_variant_value("EnumSymbolIndustry", "IndustryBeveragesNonAlco", INDUSTRY_BEVERAGES_NON_ALCO);
set_enum_variant_value("EnumSymbolIndustry", "IndustryBeveragesWineries", INDUSTRY_BEVERAGES_WINERIES);
set_enum_variant_value("EnumSymbolIndustry", "IndustryConfectioners", INDUSTRY_CONFECTIONERS);
set_enum_variant_value("EnumSymbolIndustry", "IndustryDiscountStores", INDUSTRY_DISCOUNT_STORES);
set_enum_variant_value("EnumSymbolIndustry", "IndustryEducationTrainig", INDUSTRY_EDUCATION_TRAINIG);
set_enum_variant_value("EnumSymbolIndustry", "IndustryFarmProducts", INDUSTRY_FARM_PRODUCTS);
set_enum_variant_value("EnumSymbolIndustry", "IndustryFoodDistribution", INDUSTRY_FOOD_DISTRIBUTION);
set_enum_variant_value("EnumSymbolIndustry", "IndustryGroceryStores", INDUSTRY_GROCERY_STORES);
set_enum_variant_value("EnumSymbolIndustry", "IndustryHouseholdProducts", INDUSTRY_HOUSEHOLD_PRODUCTS);
set_enum_variant_value("EnumSymbolIndustry", "IndustryPackagedFoods", INDUSTRY_PACKAGED_FOODS);
set_enum_variant_value("EnumSymbolIndustry", "IndustryTobacco", INDUSTRY_TOBACCO);
set_enum_variant_value("EnumSymbolIndustry", "IndustryOilGasDrilling", INDUSTRY_OIL_GAS_DRILLING);
set_enum_variant_value("EnumSymbolIndustry", "IndustryOilGasEp", INDUSTRY_OIL_GAS_EP);
set_enum_variant_value("EnumSymbolIndustry", "IndustryOilGasEquipment", INDUSTRY_OIL_GAS_EQUIPMENT);
set_enum_variant_value("EnumSymbolIndustry", "IndustryOilGasIntegrated", INDUSTRY_OIL_GAS_INTEGRATED);
set_enum_variant_value("EnumSymbolIndustry", "IndustryOilGasMidstream", INDUSTRY_OIL_GAS_MIDSTREAM);
set_enum_variant_value("EnumSymbolIndustry", "IndustryOilGasRefining", INDUSTRY_OIL_GAS_REFINING);
set_enum_variant_value("EnumSymbolIndustry", "IndustryThermalCoal", INDUSTRY_THERMAL_COAL);
set_enum_variant_value("EnumSymbolIndustry", "IndustryUranium", INDUSTRY_URANIUM);
set_enum_variant_value("EnumSymbolIndustry", "IndustryExchangeTradedFund", INDUSTRY_EXCHANGE_TRADED_FUND);
set_enum_variant_value("EnumSymbolIndustry", "IndustryAssetsManagement", INDUSTRY_ASSETS_MANAGEMENT);
set_enum_variant_value("EnumSymbolIndustry", "IndustryBanksDiversified", INDUSTRY_BANKS_DIVERSIFIED);
set_enum_variant_value("EnumSymbolIndustry", "IndustryBanksRegional", INDUSTRY_BANKS_REGIONAL);
set_enum_variant_value("EnumSymbolIndustry", "IndustryCapitalMarkets", INDUSTRY_CAPITAL_MARKETS);
set_enum_variant_value("EnumSymbolIndustry", "IndustryCloseEndFundDebt", INDUSTRY_CLOSE_END_FUND_DEBT);
set_enum_variant_value("EnumSymbolIndustry", "IndustryCloseEndFundEquity", INDUSTRY_CLOSE_END_FUND_EQUITY);
set_enum_variant_value("EnumSymbolIndustry", "IndustryCloseEndFundForeign", INDUSTRY_CLOSE_END_FUND_FOREIGN);
set_enum_variant_value("EnumSymbolIndustry", "IndustryCreditServices", INDUSTRY_CREDIT_SERVICES);
set_enum_variant_value("EnumSymbolIndustry", "IndustryFinancialConglomerate", INDUSTRY_FINANCIAL_CONGLOMERATE);
set_enum_variant_value("EnumSymbolIndustry", "IndustryFinancialDataExchange", INDUSTRY_FINANCIAL_DATA_EXCHANGE);
set_enum_variant_value("EnumSymbolIndustry", "IndustryInsuranceBrokers", INDUSTRY_INSURANCE_BROKERS);
set_enum_variant_value("EnumSymbolIndustry", "IndustryInsuranceDiversified", INDUSTRY_INSURANCE_DIVERSIFIED);
set_enum_variant_value("EnumSymbolIndustry", "IndustryInsuranceLife", INDUSTRY_INSURANCE_LIFE);
set_enum_variant_value("EnumSymbolIndustry", "IndustryInsuranceProperty", INDUSTRY_INSURANCE_PROPERTY);
set_enum_variant_value("EnumSymbolIndustry", "IndustryInsuranceReinsurance", INDUSTRY_INSURANCE_REINSURANCE);
set_enum_variant_value("EnumSymbolIndustry", "IndustryInsuranceSpecialty", INDUSTRY_INSURANCE_SPECIALTY);
set_enum_variant_value("EnumSymbolIndustry", "IndustryMortgageFinance", INDUSTRY_MORTGAGE_FINANCE);
set_enum_variant_value("EnumSymbolIndustry", "IndustryShellCompanies", INDUSTRY_SHELL_COMPANIES);
set_enum_variant_value("EnumSymbolIndustry", "IndustryBiotechnology", INDUSTRY_BIOTECHNOLOGY);
set_enum_variant_value("EnumSymbolIndustry", "IndustryDiagnosticsResearch", INDUSTRY_DIAGNOSTICS_RESEARCH);
set_enum_variant_value("EnumSymbolIndustry", "IndustryDrugsManufacturers", INDUSTRY_DRUGS_MANUFACTURERS);
set_enum_variant_value("EnumSymbolIndustry", "IndustryDrugsManufacturersSpec", INDUSTRY_DRUGS_MANUFACTURERS_SPEC);
set_enum_variant_value("EnumSymbolIndustry", "IndustryHealthcarePlans", INDUSTRY_HEALTHCARE_PLANS);
set_enum_variant_value("EnumSymbolIndustry", "IndustryHealthInformation", INDUSTRY_HEALTH_INFORMATION);
set_enum_variant_value("EnumSymbolIndustry", "IndustryMedicalFacilities", INDUSTRY_MEDICAL_FACILITIES);
set_enum_variant_value("EnumSymbolIndustry", "IndustryMedicalDevices", INDUSTRY_MEDICAL_DEVICES);
set_enum_variant_value("EnumSymbolIndustry", "IndustryMedicalDistribution", INDUSTRY_MEDICAL_DISTRIBUTION);
set_enum_variant_value("EnumSymbolIndustry", "IndustryMedicalInstruments", INDUSTRY_MEDICAL_INSTRUMENTS);
set_enum_variant_value("EnumSymbolIndustry", "IndustryPharmRetailers", INDUSTRY_PHARM_RETAILERS);
set_enum_variant_value("EnumSymbolIndustry", "IndustryAerospaceDefense", INDUSTRY_AEROSPACE_DEFENSE);
set_enum_variant_value("EnumSymbolIndustry", "IndustryAirlines", INDUSTRY_AIRLINES);
set_enum_variant_value("EnumSymbolIndustry", "IndustryAirportsServices", INDUSTRY_AIRPORTS_SERVICES);
set_enum_variant_value("EnumSymbolIndustry", "IndustryBuildingProducts", INDUSTRY_BUILDING_PRODUCTS);
set_enum_variant_value("EnumSymbolIndustry", "IndustryBusinessEquipment", INDUSTRY_BUSINESS_EQUIPMENT);
set_enum_variant_value("EnumSymbolIndustry", "IndustryConglomerates", INDUSTRY_CONGLOMERATES);
set_enum_variant_value("EnumSymbolIndustry", "IndustryConsultingServices", INDUSTRY_CONSULTING_SERVICES);
set_enum_variant_value("EnumSymbolIndustry", "IndustryElectricalEquipment", INDUSTRY_ELECTRICAL_EQUIPMENT);
set_enum_variant_value("EnumSymbolIndustry", "IndustryEngineeringConstruction", INDUSTRY_ENGINEERING_CONSTRUCTION);
set_enum_variant_value("EnumSymbolIndustry", "IndustryFarmHeavyMachinery", INDUSTRY_FARM_HEAVY_MACHINERY);
set_enum_variant_value("EnumSymbolIndustry", "IndustryIndustrialDistribution", INDUSTRY_INDUSTRIAL_DISTRIBUTION);
set_enum_variant_value("EnumSymbolIndustry", "IndustryInfrastructureOperations", INDUSTRY_INFRASTRUCTURE_OPERATIONS);
set_enum_variant_value("EnumSymbolIndustry", "IndustryFreightLogistics", INDUSTRY_FREIGHT_LOGISTICS);
set_enum_variant_value("EnumSymbolIndustry", "IndustryMarineShipping", INDUSTRY_MARINE_SHIPPING);
set_enum_variant_value("EnumSymbolIndustry", "IndustryMetalFabrication", INDUSTRY_METAL_FABRICATION);
set_enum_variant_value("EnumSymbolIndustry", "IndustryPollutionControl", INDUSTRY_POLLUTION_CONTROL);
set_enum_variant_value("EnumSymbolIndustry", "IndustryRailroads", INDUSTRY_RAILROADS);
set_enum_variant_value("EnumSymbolIndustry", "IndustryRentalLeasing", INDUSTRY_RENTAL_LEASING);
set_enum_variant_value("EnumSymbolIndustry", "IndustrySecurityProtection", INDUSTRY_SECURITY_PROTECTION);
set_enum_variant_value("EnumSymbolIndustry", "IndustrySpealityBusinessServices", INDUSTRY_SPEALITY_BUSINESS_SERVICES);
set_enum_variant_value("EnumSymbolIndustry", "IndustrySpealityMachinery", INDUSTRY_SPEALITY_MACHINERY);
set_enum_variant_value("EnumSymbolIndustry", "IndustryStuffingEmployment", INDUSTRY_STUFFING_EMPLOYMENT);
set_enum_variant_value("EnumSymbolIndustry", "IndustryToolsAccessories", INDUSTRY_TOOLS_ACCESSORIES);
set_enum_variant_value("EnumSymbolIndustry", "IndustryTrucking", INDUSTRY_TRUCKING);
set_enum_variant_value("EnumSymbolIndustry", "IndustryWasteManagement", INDUSTRY_WASTE_MANAGEMENT);
set_enum_variant_value("EnumSymbolIndustry", "IndustryRealEstateDevelopment", INDUSTRY_REAL_ESTATE_DEVELOPMENT);
set_enum_variant_value("EnumSymbolIndustry", "IndustryRealEstateDiversified", INDUSTRY_REAL_ESTATE_DIVERSIFIED);
set_enum_variant_value("EnumSymbolIndustry", "IndustryRealEstateServices", INDUSTRY_REAL_ESTATE_SERVICES);
set_enum_variant_value("EnumSymbolIndustry", "IndustryReitDiversified", INDUSTRY_REIT_DIVERSIFIED);
set_enum_variant_value("EnumSymbolIndustry", "IndustryReitHealtcare", INDUSTRY_REIT_HEALTCARE);
set_enum_variant_value("EnumSymbolIndustry", "IndustryReitHotelMotel", INDUSTRY_REIT_HOTEL_MOTEL);
set_enum_variant_value("EnumSymbolIndustry", "IndustryReitIndustrial", INDUSTRY_REIT_INDUSTRIAL);
set_enum_variant_value("EnumSymbolIndustry", "IndustryReitMortage", INDUSTRY_REIT_MORTAGE);
set_enum_variant_value("EnumSymbolIndustry", "IndustryReitOffice", INDUSTRY_REIT_OFFICE);
set_enum_variant_value("EnumSymbolIndustry", "IndustryReitResidental", INDUSTRY_REIT_RESIDENTAL);
set_enum_variant_value("EnumSymbolIndustry", "IndustryReitRetail", INDUSTRY_REIT_RETAIL);
set_enum_variant_value("EnumSymbolIndustry", "IndustryReitSpeciality", INDUSTRY_REIT_SPECIALITY);
set_enum_variant_value("EnumSymbolIndustry", "IndustryCommunicationEquipment", INDUSTRY_COMMUNICATION_EQUIPMENT);
set_enum_variant_value("EnumSymbolIndustry", "IndustryComputerHardware", INDUSTRY_COMPUTER_HARDWARE);
set_enum_variant_value("EnumSymbolIndustry", "IndustryConsumerElectronics", INDUSTRY_CONSUMER_ELECTRONICS);
set_enum_variant_value("EnumSymbolIndustry", "IndustryElectronicComponents", INDUSTRY_ELECTRONIC_COMPONENTS);
set_enum_variant_value("EnumSymbolIndustry", "IndustryElectronicDistribution", INDUSTRY_ELECTRONIC_DISTRIBUTION);
set_enum_variant_value("EnumSymbolIndustry", "IndustryItServices", INDUSTRY_IT_SERVICES);
set_enum_variant_value("EnumSymbolIndustry", "IndustryScientificInstruments", INDUSTRY_SCIENTIFIC_INSTRUMENTS);
set_enum_variant_value("EnumSymbolIndustry", "IndustrySemiconductorEquipment", INDUSTRY_SEMICONDUCTOR_EQUIPMENT);
set_enum_variant_value("EnumSymbolIndustry", "IndustrySemiconductors", INDUSTRY_SEMICONDUCTORS);
set_enum_variant_value("EnumSymbolIndustry", "IndustrySoftwareApplication", INDUSTRY_SOFTWARE_APPLICATION);
set_enum_variant_value("EnumSymbolIndustry", "IndustrySoftwareInfrastructure", INDUSTRY_SOFTWARE_INFRASTRUCTURE);
set_enum_variant_value("EnumSymbolIndustry", "IndustrySolar", INDUSTRY_SOLAR);
set_enum_variant_value("EnumSymbolIndustry", "IndustryUtilitiesDiversified", INDUSTRY_UTILITIES_DIVERSIFIED);
set_enum_variant_value("EnumSymbolIndustry", "IndustryUtilitiesPowerproducers", INDUSTRY_UTILITIES_POWERPRODUCERS);
set_enum_variant_value("EnumSymbolIndustry", "IndustryUtilitiesRenewable", INDUSTRY_UTILITIES_RENEWABLE);
set_enum_variant_value("EnumSymbolIndustry", "IndustryUtilitiesRegulatedElectric", INDUSTRY_UTILITIES_REGULATED_ELECTRIC);
set_enum_variant_value("EnumSymbolIndustry", "IndustryUtilitiesRegulatedGas", INDUSTRY_UTILITIES_REGULATED_GAS);
set_enum_variant_value("EnumSymbolIndustry", "IndustryUtilitiesRegulatedWater", INDUSTRY_UTILITIES_REGULATED_WATER);
//set_enum_variant_value("EnumSymbolIndustry", "IndustryUtilitiesFirst", INDUSTRY_UTILITIES_FIRST);
//set_enum_variant_value("EnumSymbolIndustry", "IndustryUtilitiesLast", INDUSTRY_UTILITIES_LAST);
set_enum_variant_value("EnumSymbolOptionMode", "SymbolOptionModeEuropean", SYMBOL_OPTION_MODE_EUROPEAN);
set_enum_variant_value("EnumSymbolOptionMode", "SymbolOptionModeAmerican", SYMBOL_OPTION_MODE_AMERICAN );
set_enum_variant_value("EnumSymbolOptionRight", "SymbolOptionRightCall", SYMBOL_OPTION_RIGHT_CALL);
set_enum_variant_value("EnumSymbolOptionRight", "SymbolOptionRightPut", SYMBOL_OPTION_RIGHT_PUT);
set_enum_variant_value("EnumSymbolOrderGtcMode", "SymbolOrdersGtc", SYMBOL_ORDERS_GTC);
set_enum_variant_value("EnumSymbolOrderGtcMode", "SymbolOrdersDaily", SYMBOL_ORDERS_DAILY);
set_enum_variant_value("EnumSymbolOrderGtcMode", "SymbolOrdersDailyExcludingStops", SYMBOL_ORDERS_DAILY_EXCLUDING_STOPS);
set_enum_variant_value("EnumSymbolSector", "SectorUndefined", SECTOR_UNDEFINED);
set_enum_variant_value("EnumSymbolSector", "SectorBasicMaterials", SECTOR_BASIC_MATERIALS);
set_enum_variant_value("EnumSymbolSector", "SectorCommunicationServices", SECTOR_COMMUNICATION_SERVICES);
set_enum_variant_value("EnumSymbolSector", "SectorConsumerCyclical", SECTOR_CONSUMER_CYCLICAL);
set_enum_variant_value("EnumSymbolSector", "SectorConsumerDefensive", SECTOR_CONSUMER_DEFENSIVE);
set_enum_variant_value("EnumSymbolSector", "SectorCurrency", SECTOR_CURRENCY);
set_enum_variant_value("EnumSymbolSector", "SectorCurrencyCrypto", SECTOR_CURRENCY_CRYPTO);
set_enum_variant_value("EnumSymbolSector", "SectorEnergy", SECTOR_ENERGY);
set_enum_variant_value("EnumSymbolSector", "SectorFinancial", SECTOR_FINANCIAL);
set_enum_variant_value("EnumSymbolSector", "SectorHealthcare", SECTOR_HEALTHCARE);
set_enum_variant_value("EnumSymbolSector", "SectorIndustrials", SECTOR_INDUSTRIALS);
set_enum_variant_value("EnumSymbolSector", "SectorRealEstate", SECTOR_REAL_ESTATE);
set_enum_variant_value("EnumSymbolSector", "SectorTechnology", SECTOR_TECHNOLOGY);
set_enum_variant_value("EnumSymbolSector", "SectorUtilities", SECTOR_UTILITIES);
set_enum_variant_value("EnumSymbolSwapMode", "SymbolSwapModeDisabled", SYMBOL_SWAP_MODE_DISABLED);
set_enum_variant_value("EnumSymbolSwapMode", "SymbolSwapModePoints", SYMBOL_SWAP_MODE_POINTS);
set_enum_variant_value("EnumSymbolSwapMode", "SymbolSwapModeCurrencySymbol", SYMBOL_SWAP_MODE_CURRENCY_SYMBOL);
set_enum_variant_value("EnumSymbolSwapMode", "SymbolSwapModeCurrencyMargin", SYMBOL_SWAP_MODE_CURRENCY_MARGIN);
set_enum_variant_value("EnumSymbolSwapMode", "SymbolSwapModeCurrencyDeposit", SYMBOL_SWAP_MODE_CURRENCY_DEPOSIT);
set_enum_variant_value("EnumSymbolSwapMode", "SymbolSwapModeInterestCurrent", SYMBOL_SWAP_MODE_INTEREST_CURRENT);
set_enum_variant_value("EnumSymbolSwapMode", "SymbolSwapModeInterestOpen", SYMBOL_SWAP_MODE_INTEREST_OPEN);
set_enum_variant_value("EnumSymbolSwapMode", "SymbolSwapModeReopenCurrent", SYMBOL_SWAP_MODE_REOPEN_CURRENT);
set_enum_variant_value("EnumSymbolSwapMode", "SymbolSwapModeReopenBid", SYMBOL_SWAP_MODE_REOPEN_BID);
set_enum_variant_value("EnumSymbolTradeExecution", "SymbolTradeExecutionRequest", SYMBOL_TRADE_EXECUTION_REQUEST);
set_enum_variant_value("EnumSymbolTradeExecution", "SymbolTradeExecutionInstant", SYMBOL_TRADE_EXECUTION_INSTANT);
set_enum_variant_value("EnumSymbolTradeExecution", "SymbolTradeExecutionMarket", SYMBOL_TRADE_EXECUTION_MARKET);
set_enum_variant_value("EnumSymbolTradeExecution", "SymbolTradeExecutionExchange", SYMBOL_TRADE_EXECUTION_EXCHANGE);
set_enum_variant_value("EnumSymbolTradeMode", "SymbolTradeModeDisabled", SYMBOL_TRADE_MODE_DISABLED);
set_enum_variant_value("EnumSymbolTradeMode", "SymbolTradeModeLongonly", SYMBOL_TRADE_MODE_LONGONLY);
set_enum_variant_value("EnumSymbolTradeMode", "SymbolTradeModeShortonly", SYMBOL_TRADE_MODE_SHORTONLY);
set_enum_variant_value("EnumSymbolTradeMode", "SymbolTradeModeCloseonly", SYMBOL_TRADE_MODE_CLOSEONLY);
set_enum_variant_value("EnumSymbolTradeMode", "SymbolTradeModeFull", SYMBOL_TRADE_MODE_FULL);
set_enum_variant_value("EnumAccountMarginMode", "AccountMarginModeRetailNetting", ACCOUNT_MARGIN_MODE_RETAIL_NETTING);
set_enum_variant_value("EnumAccountMarginMode", "AccountMarginModeExchange", ACCOUNT_MARGIN_MODE_EXCHANGE);
set_enum_variant_value("EnumAccountMarginMode", "AccountMarginModeRetailHedging", ACCOUNT_MARGIN_MODE_RETAIL_HEDGING);
set_enum_variant_value("EnumAccountStopoutMode", "AccountStopoutModePercent", ACCOUNT_STOPOUT_MODE_PERCENT);
set_enum_variant_value("EnumAccountStopoutMode", "AccountStopoutModeMoney", ACCOUNT_STOPOUT_MODE_MONEY);
set_enum_variant_value("EnumAccountTradeMode", "AccountTradeModeDemo", ACCOUNT_TRADE_MODE_DEMO);
set_enum_variant_value("EnumAccountTradeMode", "AccountTradeModeContest", ACCOUNT_TRADE_MODE_CONTEST);
set_enum_variant_value("EnumAccountTradeMode", "AccountTradeModeReal", ACCOUNT_TRADE_MODE_REAL);
set_enum_variant_value("EnumDealEntry", "DealEntryIn", DEAL_ENTRY_IN);
set_enum_variant_value("EnumDealEntry", "DealEntryOut", DEAL_ENTRY_OUT);
set_enum_variant_value("EnumDealEntry", "DealEntryInout", DEAL_ENTRY_INOUT);
set_enum_variant_value("EnumDealEntry", "DealEntryOutBy", DEAL_ENTRY_OUT_BY);
set_enum_variant_value("EnumDealReason", "DealReasonClient", DEAL_REASON_CLIENT);
set_enum_variant_value("EnumDealReason", "DealReasonMobile", DEAL_REASON_MOBILE);
set_enum_variant_value("EnumDealReason", "DealReasonWeb", DEAL_REASON_WEB);
set_enum_variant_value("EnumDealReason", "DealReasonExpert", DEAL_REASON_EXPERT);
set_enum_variant_value("EnumDealReason", "DealReasonSl", DEAL_REASON_SL);
set_enum_variant_value("EnumDealReason", "DealReasonTp", DEAL_REASON_TP);
set_enum_variant_value("EnumDealReason", "DealReasonSo", DEAL_REASON_SO);
set_enum_variant_value("EnumDealReason", "DealReasonRollover", DEAL_REASON_ROLLOVER);
set_enum_variant_value("EnumDealReason", "DealReasonVmargin", DEAL_REASON_VMARGIN);
set_enum_variant_value("EnumDealReason", "DealReasonSplit", DEAL_REASON_SPLIT);
set_enum_variant_value("EnumDealType", "DealTypeBuy", DEAL_TYPE_BUY);
set_enum_variant_value("EnumDealType", "DealTypeSell", DEAL_TYPE_SELL);
set_enum_variant_value("EnumDealType", "DealTypeBalance", DEAL_TYPE_BALANCE);
set_enum_variant_value("EnumDealType", "DealTypeCredit", DEAL_TYPE_CREDIT);
set_enum_variant_value("EnumDealType", "DealTypeCharge", DEAL_TYPE_CHARGE);
set_enum_variant_value("EnumDealType", "DealTypeCorrection", DEAL_TYPE_CORRECTION);
set_enum_variant_value("EnumDealType", "DealTypeBonus", DEAL_TYPE_BONUS);
set_enum_variant_value("EnumDealType", "DealTypeCommission", DEAL_TYPE_COMMISSION);
set_enum_variant_value("EnumDealType", "DealTypeCommissionDaily", DEAL_TYPE_COMMISSION_DAILY);
set_enum_variant_value("EnumDealType", "DealTypeCommissionMonthly", DEAL_TYPE_COMMISSION_MONTHLY);
set_enum_variant_value("EnumDealType", "DealTypeCommissionAgentDaily", DEAL_TYPE_COMMISSION_AGENT_DAILY);
set_enum_variant_value("EnumDealType", "DealTypeCommissionAgentMonthly", DEAL_TYPE_COMMISSION_AGENT_MONTHLY);
set_enum_variant_value("EnumDealType", "DealTypeInterest", DEAL_TYPE_INTEREST);
set_enum_variant_value("EnumDealType", "DealTypeBuyCanceled", DEAL_TYPE_BUY_CANCELED);
set_enum_variant_value("EnumDealType", "DealTypeSellCanceled", DEAL_TYPE_SELL_CANCELED);
set_enum_variant_value("EnumDealType", "DealDividend", DEAL_DIVIDEND);
set_enum_variant_value("EnumDealType", "DealDividendFranked", DEAL_DIVIDEND_FRANKED);
set_enum_variant_value("EnumDealType", "DealTax", DEAL_TAX);
set_enum_variant_value("EnumBookType", "BookTypeSell", BOOK_TYPE_SELL);
set_enum_variant_value("EnumBookType", "BookTypeBuy", BOOK_TYPE_BUY);
set_enum_variant_value("EnumBookType", "BookTypeSellMarket", BOOK_TYPE_SELL_MARKET);
set_enum_variant_value("EnumBookType", "BookTypeBuyMarket", BOOK_TYPE_BUY_MARKET);
set_enum_variant_value("EnumTradeTransactionType", "TradeTransactionOrderAdd", TRADE_TRANSACTION_ORDER_ADD);
set_enum_variant_value("EnumTradeTransactionType", "TradeTransactionOrderUpdate", TRADE_TRANSACTION_ORDER_UPDATE);
set_enum_variant_value("EnumTradeTransactionType", "TradeTransactionOrderDelete", TRADE_TRANSACTION_ORDER_DELETE);
set_enum_variant_value("EnumTradeTransactionType", "TradeTransactionDealAdd", TRADE_TRANSACTION_DEAL_ADD);
set_enum_variant_value("EnumTradeTransactionType", "TradeTransactionDealUpdate", TRADE_TRANSACTION_DEAL_UPDATE);
set_enum_variant_value("EnumTradeTransactionType", "TradeTransactionDealDelete", TRADE_TRANSACTION_DEAL_DELETE);
set_enum_variant_value("EnumTradeTransactionType", "TradeTransactionHistoryAdd", TRADE_TRANSACTION_HISTORY_ADD);
set_enum_variant_value("EnumTradeTransactionType", "TradeTransactionHistoryUpdate", TRADE_TRANSACTION_HISTORY_UPDATE);
set_enum_variant_value("EnumTradeTransactionType", "TradeTransactionHistoryDelete", TRADE_TRANSACTION_HISTORY_DELETE);
set_enum_variant_value("EnumTradeTransactionType", "TradeTransactionPosition", TRADE_TRANSACTION_POSITION);
set_enum_variant_value("EnumTradeTransactionType", "TradeTransactionRequest", TRADE_TRANSACTION_REQUEST);
set_enum_variant_value("EnumOrderType", "OrderTypeBuy", ORDER_TYPE_BUY);
set_enum_variant_value("EnumOrderType", "OrderTypeSell", ORDER_TYPE_SELL);
set_enum_variant_value("EnumOrderType", "OrderTypeBuyLimit", ORDER_TYPE_BUY_LIMIT);
set_enum_variant_value("EnumOrderType", "OrderTypeSellLimit", ORDER_TYPE_SELL_LIMIT);
set_enum_variant_value("EnumOrderType", "OrderTypeBuyStop", ORDER_TYPE_BUY_STOP);
set_enum_variant_value("EnumOrderType", "OrderTypeSellStop", ORDER_TYPE_SELL_STOP);
set_enum_variant_value("EnumOrderType", "OrderTypeBuyStopLimit", ORDER_TYPE_BUY_STOP_LIMIT);
set_enum_variant_value("EnumOrderType", "OrderTypeSellStopLimit", ORDER_TYPE_SELL_STOP_LIMIT);
set_enum_variant_value("EnumOrderType", "OrderTypeCloseBy", ORDER_TYPE_CLOSE_BY);
set_enum_variant_value("EnumOrderState", "OrderStateStarted", ORDER_STATE_STARTED);
set_enum_variant_value("EnumOrderState", "OrderStatePlaced", ORDER_STATE_PLACED);
set_enum_variant_value("EnumOrderState", "OrderStateCanceled", ORDER_STATE_CANCELED);
set_enum_variant_value("EnumOrderState", "OrderStatePartial", ORDER_STATE_PARTIAL);
set_enum_variant_value("EnumOrderState", "OrderStateFilled", ORDER_STATE_FILLED);
set_enum_variant_value("EnumOrderState", "OrderStateRejected", ORDER_STATE_REJECTED);
set_enum_variant_value("EnumOrderState", "OrderStateExpired", ORDER_STATE_EXPIRED);
set_enum_variant_value("EnumOrderState", "OrderStateRequestAdd", ORDER_STATE_REQUEST_ADD);
set_enum_variant_value("EnumOrderState", "OrderStateRequestModify", ORDER_STATE_REQUEST_MODIFY);
set_enum_variant_value("EnumOrderState", "OrderStateRequestCancel", ORDER_STATE_REQUEST_CANCEL);
set_enum_variant_value("EnumDealType", "DealTypeBuy", DEAL_TYPE_BUY);
set_enum_variant_value("EnumDealType", "DealTypeSell", DEAL_TYPE_SELL);
set_enum_variant_value("EnumDealType", "DealTypeBalance", DEAL_TYPE_BALANCE);
set_enum_variant_value("EnumDealType", "DealTypeCredit", DEAL_TYPE_CREDIT);
set_enum_variant_value("EnumDealType", "DealTypeCharge", DEAL_TYPE_CHARGE);
set_enum_variant_value("EnumDealType", "DealTypeCorrection", DEAL_TYPE_CORRECTION);
set_enum_variant_value("EnumDealType", "DealTypeBonus", DEAL_TYPE_BONUS);
set_enum_variant_value("EnumDealType", "DealTypeCommission", DEAL_TYPE_COMMISSION);
set_enum_variant_value("EnumDealType", "DealTypeCommissionDaily", DEAL_TYPE_COMMISSION_DAILY);
set_enum_variant_value("EnumDealType", "DealTypeCommissionMonthly", DEAL_TYPE_COMMISSION_MONTHLY);
set_enum_variant_value("EnumDealType", "DealTypeCommissionAgentDaily", DEAL_TYPE_COMMISSION_AGENT_DAILY);
set_enum_variant_value("EnumDealType", "DealTypeCommissionAgentMonthly", DEAL_TYPE_COMMISSION_AGENT_MONTHLY);
set_enum_variant_value("EnumDealType", "DealTypeInterest", DEAL_TYPE_INTEREST);
set_enum_variant_value("EnumDealType", "DealTypeBuyCanceled", DEAL_TYPE_BUY_CANCELED);
set_enum_variant_value("EnumDealType", "DealTypeSellCanceled", DEAL_TYPE_SELL_CANCELED);
set_enum_variant_value("EnumDealType", "DealDividend", DEAL_DIVIDEND);
set_enum_variant_value("EnumDealType", "DealDividendFranked", DEAL_DIVIDEND_FRANKED);
set_enum_variant_value("EnumDealType", "DealTax", DEAL_TAX);
set_enum_variant_value("EnumOrderTypeTime", "OrderTimeGtc", ORDER_TIME_GTC);
set_enum_variant_value("EnumOrderTypeTime", "OrderTimeDay", ORDER_TIME_DAY);
set_enum_variant_value("EnumOrderTypeTime", "OrderTimeSpecified", ORDER_TIME_SPECIFIED);
set_enum_variant_value("EnumOrderTypeTime", "OrderTimeSpecifiedDay", ORDER_TIME_SPECIFIED_DAY);
set_enum_variant_value("EnumTradeRequestActions", "TradeActionDeal", TRADE_ACTION_DEAL);
set_enum_variant_value("EnumTradeRequestActions", "TradeActionPending", TRADE_ACTION_PENDING);
set_enum_variant_value("EnumTradeRequestActions", "TradeActionSltp", TRADE_ACTION_SLTP);
set_enum_variant_value("EnumTradeRequestActions", "TradeActionModify", TRADE_ACTION_MODIFY);
set_enum_variant_value("EnumTradeRequestActions", "TradeActionRemove", TRADE_ACTION_REMOVE);
set_enum_variant_value("EnumTradeRequestActions", "TradeActionCloseBy", TRADE_ACTION_CLOSE_BY);
set_enum_variant_value("EnumOrderType", "OrderTypeBuy", ORDER_TYPE_BUY);
set_enum_variant_value("EnumOrderType", "OrderTypeSell", ORDER_TYPE_SELL);
set_enum_variant_value("EnumOrderType", "OrderTypeBuyLimit", ORDER_TYPE_BUY_LIMIT);
set_enum_variant_value("EnumOrderType", "OrderTypeSellLimit", ORDER_TYPE_SELL_LIMIT);
set_enum_variant_value("EnumOrderType", "OrderTypeBuyStop", ORDER_TYPE_BUY_STOP);
set_enum_variant_value("EnumOrderType", "OrderTypeSellStop", ORDER_TYPE_SELL_STOP);
set_enum_variant_value("EnumOrderType", "OrderTypeBuyStopLimit", ORDER_TYPE_BUY_STOP_LIMIT);
set_enum_variant_value("EnumOrderType", "OrderTypeSellStopLimit", ORDER_TYPE_SELL_STOP_LIMIT);
set_enum_variant_value("EnumOrderType", "OrderTypeCloseBy", ORDER_TYPE_CLOSE_BY);
set_enum_variant_value("EnumOrderTypeFilling", "OrderFillingFok", ORDER_FILLING_FOK);
set_enum_variant_value("EnumOrderTypeFilling", "OrderFillingIoc", ORDER_FILLING_IOC);
set_enum_variant_value("EnumOrderTypeFilling", "OrderFillingReturn", ORDER_FILLING_RETURN);
