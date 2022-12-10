// Defines a structure and function to retrieve past deals, which is, then, shared as-is with Rust

struct DealPropertiesBridge {
	                        double                       deal_volume;      // Deal volume
	                        double                        deal_price;      // Deal price
	                        double                   deal_commission;      // Deal commission
	                        double                         deal_swap;      // Cumulative swap on close
	                        double                       deal_profit;      // Deal profit
	                        double                          deal_fee;      // Fee for making a deal charged immediately after performing a deal
	                        double                           deal_sl;      // Stop Loss level
	                        double                           deal_tp;      // Take Profit level
	                          long                       deal_ticket;      // Deal ticket. Unique number assigned to each deal
	                          long                        deal_order;      // Deal order number
	                          long                     deal_time_msc;      // The time of a deal execution in milliseconds since 01.01.1970
	                          long                        deal_magic;      // Deal magic number (see ORDER_MAGIC)
	                          long                  deal_position_id;      // Identifier of a position, in the opening, modification or closing of which this deal took part. Each position has a unique identifier that is assigned to all deals executed for the symbol during the entire lifetime of the position.
	                      datetime                         deal_time;      // Deal time
	                        string                       deal_symbol;      // Deal symbol
	                        string                      deal_comment;      // Deal comment
	                        string                  deal_external_id;      // Deal identifier in an external trading system (on the Exchange) 
	                ENUM_DEAL_TYPE                         deal_type;      // Deal type
	               ENUM_DEAL_ENTRY                        deal_entry;      // Deal entry - entry in, entry out, reverse
	              ENUM_DEAL_REASON                       deal_reason;      // The reason or source for deal execution
};

DealPropertiesBridge instantiate_deal_properties_bridge(ulong ticket_number) {
	DealPropertiesBridge instance;
	instance.deal_volume                       = HistoryDealGetDouble(ticket_number, DEAL_VOLUME);
	instance.deal_price                        = HistoryDealGetDouble(ticket_number, DEAL_PRICE);
	instance.deal_commission                   = HistoryDealGetDouble(ticket_number, DEAL_COMMISSION);
	instance.deal_swap                         = HistoryDealGetDouble(ticket_number, DEAL_SWAP);
	instance.deal_profit                       = HistoryDealGetDouble(ticket_number, DEAL_PROFIT);
	instance.deal_fee                          = HistoryDealGetDouble(ticket_number, DEAL_FEE);
	instance.deal_sl                           = HistoryDealGetDouble(ticket_number, DEAL_SL);
	instance.deal_tp                           = HistoryDealGetDouble(ticket_number, DEAL_TP);
	instance.deal_symbol                       = HistoryDealGetString(ticket_number, DEAL_SYMBOL);
	instance.deal_comment                      = HistoryDealGetString(ticket_number, DEAL_COMMENT);
	instance.deal_external_id                  = HistoryDealGetString(ticket_number, DEAL_EXTERNAL_ID);
	instance.deal_ticket                       = HistoryDealGetInteger(ticket_number, DEAL_TICKET);
	instance.deal_order                        = HistoryDealGetInteger(ticket_number, DEAL_ORDER);
	instance.deal_time                         = HistoryDealGetInteger(ticket_number, DEAL_TIME);
	instance.deal_time_msc                     = HistoryDealGetInteger(ticket_number, DEAL_TIME_MSC);
	instance.deal_type                         = HistoryDealGetInteger(ticket_number, DEAL_TYPE);
	instance.deal_entry                        = HistoryDealGetInteger(ticket_number, DEAL_ENTRY);
	instance.deal_magic                        = HistoryDealGetInteger(ticket_number, DEAL_MAGIC);
	instance.deal_reason                       = HistoryDealGetInteger(ticket_number, DEAL_REASON);
	instance.deal_position_id                  = HistoryDealGetInteger(ticket_number, DEAL_POSITION_ID);

/*
	Print("deal_volume      = " + instance.deal_volume);
    Print("deal_price       = " + instance.deal_price);
    Print("deal_commission  = " + instance.deal_commission);
    Print("deal_swap        = " + instance.deal_swap);
    Print("deal_profit      = " + instance.deal_profit);
    Print("deal_fee         = " + instance.deal_fee);
    Print("deal_sl          = " + instance.deal_sl);
    Print("deal_tp          = " + instance.deal_tp);
    Print("deal_symbol      = " + instance.deal_symbol);
    Print("deal_comment     = " + instance.deal_comment);
    Print("deal_external_id = " + instance.deal_external_id);
    Print("deal_ticket      = " + instance.deal_ticket);
    Print("deal_order       = " + instance.deal_order);
    Print("deal_time        = " + instance.deal_time);
    Print("deal_time_msc    = " + instance.deal_time_msc);
    Print("deal_type        = " + instance.deal_type);
    Print("deal_entry       = " + instance.deal_entry);
    Print("deal_magic       = " + instance.deal_magic);
    Print("deal_reason      = " + instance.deal_reason);
    Print("deal_position_id = " + instance.deal_position_id);
*/

	return instance;
}


// reports all deals since time immemorial
void collect_and_report_all_deals_properties(int rust_handle) {
   HistorySelect(0, TimeCurrent());
   for (uint i=0; i<HistoryDealsTotal(); i++) {
       long ticket_number = HistoryDealGetTicket(i);
       if (ticket_number > 0) {
           DealPropertiesBridge deal_properties = instantiate_deal_properties_bridge(ticket_number);
           //Print(StringFormat("RustMtBridge(%d): '%s': Reporting deal properties...", rust_handle, _Symbol));
           report_deal_properties(rust_handle, deal_properties);
       }
   }

}