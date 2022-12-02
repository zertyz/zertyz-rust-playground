// Defines a structure and function to retrieve account information, which is, then, shared as-is with Rust

struct AccountInfoBridge {
	                        double                   account_balance;      // Account balance in the deposit currency
	                        double                    account_credit;      // Account credit in the deposit currency
	                        double                    account_profit;      // Current profit of an account in the deposit currency
	                        double                    account_equity;      // Account equity in the deposit currency
	                        double                    account_margin;      // Account margin used in the deposit currency
	                        double               account_margin_free;      // Free margin of an account in the deposit currency
	                        double              account_margin_level;      // Account margin level in percents
	                        double            account_margin_so_call;      // Margin call level. Depending on the set ACCOUNT_MARGIN_SO_MODE is expressed in percents or in the deposit currency
	                        double              account_margin_so_so;      // Margin stop out level. Depending on the set ACCOUNT_MARGIN_SO_MODE is expressed in percents or in the deposit currency
	                        double            account_margin_initial;      // Initial margin. The amount reserved on an account to cover the margin of all pending orders 
	                        double        account_margin_maintenance;      // Maintenance margin. The minimum equity reserved on an account to cover the minimum amount of all open positions
	                        double                    account_assets;      // The current assets of an account
	                        double               account_liabilities;      // The current liabilities on an account
	                        double        account_commission_blocked;      // The current blocked commission amount on an account
	                          long                     account_login;      // Account number
	                          long                  account_leverage;      // Account leverage
	                        string                      account_name;      // Client name
	                        string                    account_server;      // Trade server name
	                        string                  account_currency;      // Account currency
	                        string                   account_company;      // Name of a company that serves the account
	       ENUM_ACCOUNT_TRADE_MODE                account_trade_mode;      // Account trade mode
	                           int              account_limit_orders;      // Maximum allowed number of active pending orders
	     ENUM_ACCOUNT_STOPOUT_MODE            account_margin_so_mode;      // Mode for setting the minimal allowed margin
	      ENUM_ACCOUNT_MARGIN_MODE               account_margin_mode;      // Margin calculation mode
	                           int           account_currency_digits;      // The number of decimal places in the account currency, which are required for an accurate display of trading results
	                          bool             account_trade_allowed;      // Allowed trade for the current account
	                          bool              account_trade_expert;      // Allowed trade for an Expert Advisor
	                          bool                account_fifo_close;      // An indication showing that positions can only be closed by FIFO rule. If the property value is set to
	                          bool             account_hedge_allowed;      // Allowed opposite positions on a single symbol
};

AccountInfoBridge InstantiateAccountInfoBridge() {
	AccountInfoBridge instance;
	instance.account_balance                   = AccountInfoDouble(ACCOUNT_BALANCE);
	instance.account_credit                    = AccountInfoDouble(ACCOUNT_CREDIT);
	instance.account_profit                    = AccountInfoDouble(ACCOUNT_PROFIT);
	instance.account_equity                    = AccountInfoDouble(ACCOUNT_EQUITY);
	instance.account_margin                    = AccountInfoDouble(ACCOUNT_MARGIN);
	instance.account_margin_free               = AccountInfoDouble(ACCOUNT_MARGIN_FREE);
	instance.account_margin_level              = AccountInfoDouble(ACCOUNT_MARGIN_LEVEL);
	instance.account_margin_so_call            = AccountInfoDouble(ACCOUNT_MARGIN_SO_CALL);
	instance.account_margin_so_so              = AccountInfoDouble(ACCOUNT_MARGIN_SO_SO);
	instance.account_margin_initial            = AccountInfoDouble(ACCOUNT_MARGIN_INITIAL);
	instance.account_margin_maintenance        = AccountInfoDouble(ACCOUNT_MARGIN_MAINTENANCE);
	instance.account_assets                    = AccountInfoDouble(ACCOUNT_ASSETS);
	instance.account_liabilities               = AccountInfoDouble(ACCOUNT_LIABILITIES);
	instance.account_commission_blocked        = AccountInfoDouble(ACCOUNT_COMMISSION_BLOCKED);
	instance.account_name                      = AccountInfoString(ACCOUNT_NAME);
	instance.account_server                    = AccountInfoString(ACCOUNT_SERVER);
	instance.account_currency                  = AccountInfoString(ACCOUNT_CURRENCY);
	instance.account_company                   = AccountInfoString(ACCOUNT_COMPANY);
	instance.account_login                     = AccountInfoInteger(ACCOUNT_LOGIN);
	instance.account_trade_mode                = AccountInfoInteger(ACCOUNT_TRADE_MODE);
	instance.account_leverage                  = AccountInfoInteger(ACCOUNT_LEVERAGE);
	instance.account_limit_orders              = AccountInfoInteger(ACCOUNT_LIMIT_ORDERS);
	instance.account_margin_so_mode            = AccountInfoInteger(ACCOUNT_MARGIN_SO_MODE);
	instance.account_trade_allowed             = AccountInfoInteger(ACCOUNT_TRADE_ALLOWED);
	instance.account_trade_expert              = AccountInfoInteger(ACCOUNT_TRADE_EXPERT);
	instance.account_margin_mode               = AccountInfoInteger(ACCOUNT_MARGIN_MODE);
	instance.account_currency_digits           = AccountInfoInteger(ACCOUNT_CURRENCY_DIGITS);
	instance.account_fifo_close                = AccountInfoInteger(ACCOUNT_FIFO_CLOSE);
	instance.account_hedge_allowed             = AccountInfoInteger(ACCOUNT_HEDGE_ALLOWED);

/*
	Print("account_balance            = " + instance.account_balance);
    Print("account_credit             = " + instance.account_credit);
    Print("account_profit             = " + instance.account_profit);
    Print("account_equity             = " + instance.account_equity);
    Print("account_margin             = " + instance.account_margin);
    Print("account_margin_free        = " + instance.account_margin_free);
    Print("account_margin_level       = " + instance.account_margin_level);
    Print("account_margin_so_call     = " + instance.account_margin_so_call);
    Print("account_margin_so_so       = " + instance.account_margin_so_so);
    Print("account_margin_initial     = " + instance.account_margin_initial);
    Print("account_margin_maintenance = " + instance.account_margin_maintenance);
    Print("account_assets             = " + instance.account_assets);
    Print("account_liabilities        = " + instance.account_liabilities);
    Print("account_commission_blocked = " + instance.account_commission_blocked);
    Print("account_name               = " + instance.account_name);
    Print("account_server             = " + instance.account_server);
    Print("account_currency           = " + instance.account_currency);
    Print("account_company            = " + instance.account_company);
    Print("account_login              = " + instance.account_login);
    Print("account_trade_mode         = " + instance.account_trade_mode);
    Print("account_leverage           = " + instance.account_leverage);
    Print("account_limit_orders       = " + instance.account_limit_orders);
    Print("account_margin_so_mode     = " + instance.account_margin_so_mode);
    Print("account_trade_allowed      = " + instance.account_trade_allowed);
    Print("account_trade_expert       = " + instance.account_trade_expert);
    Print("account_margin_mode        = " + instance.account_margin_mode);
    Print("account_currency_digits    = " + instance.account_currency_digits);
    Print("account_fifo_close         = " + instance.account_fifo_close);
    Print("account_hedge_allowed      = " + instance.account_hedge_allowed);
*/

	return instance;
}

