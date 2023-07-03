//! Maps trading server constants, enums and structures -- as exposed by Metatrader

use strum::{EnumString,FromRepr};


#[repr(u32)]
#[derive(Debug,PartialEq,EnumString,FromRepr,Clone,Copy)]
pub enum Mt5ErrorCodes {
    /// The operation completed successfully
    ErrSuccess = 0,
    /// Unexpected internal Error
    ErrInternalError = 4001,
    /// Wrong parameter in the inner call of the client terminal function
    ErrWrongInternalParameter = 4002,
    /// Wrong parameter when calling the system function
    ErrInvalidParameter = 4003,
    /// Not enough memory to perform the system function
    ErrNotEnoughMemory = 4004,
    /// The structure contains objects of strings and/or dynamic arrays and/or structure of such objects and/or classes
    ErrStructWithobjectsOrclass = 4005,
    /// Array of a wrong type, wrong size, or a damaged object of a dynamic array
    ErrInvalidArray = 4006,
    /// Not enough memory for the relocation of an array, or an attempt to change the size of a static array
    ErrArrayResizeError = 4007,
    /// Not enough memory for the relocation of string
    ErrStringResizeError = 4008,
    /// Not initialized string
    ErrNotinitializedString = 4009,
    /// Invalid date and/or time
    ErrInvalidDatetime = 4010,
    /// Total amount of elements in the array cannot exceed 2147483647
    ErrArrayBadSize = 4011,
    /// Wrong pointer
    ErrInvalidPointer = 4012,
    /// Wrong type of pointer
    ErrInvalidPointerType = 4013,
    /// Function is not allowed for call
    ErrFunctionNotAllowed = 4014,
    /// The names of the dynamic and the static resource match
    ErrResourceNameDuplicated = 4015,
    /// Resource with this name has not been found in ex5
    ErrResourceNotFound = 4016,
    /// Unsupported resource type or its size exceeds 16 Mb
    ErrResourceUnsuppotedType = 4017,
    /// The resource name exceeds 63 characters
    ErrResourceNameIsTooLong = 4018,
    /// Overflow occurred when calculating math function
    ErrMathOverflow = 4019,
    /// Out of test end date after calling Sleep()
    ErrSleepError = 4020,
    /// Test forcibly stopped from the outside. For example, optimization interrupted, visual testing window closed or testing agent stopped
    ErrProgramStopped = 4022,
    /// Wrong chart id
    ErrChartWrongId = 4101,
    /// Chart does not respond
    ErrChartNoReply = 4102,
    /// Chart not found
    ErrChartNotFound = 4103,
    /// No Expert Advisor in the chart that could handle the event
    ErrChartNoExpert = 4104,
    /// Chart opening Error
    ErrChartCannotOpen = 4105,
    /// Failed to change chart symbol and period
    ErrChartCannotChange = 4106,
    /// Error value of the parameter for the function of working with charts
    ErrChartWrongParameter = 4107,
    /// Failed to create timer
    ErrChartCannotCreateTimer = 4108,
    /// Wrong chart property id
    ErrChartWrongProperty = 4109,
    /// Error creating screenshots
    ErrChartScreenshotFailed = 4110,
    /// Error navigating through chart
    ErrChartNavigateFailed = 4111,
    /// Error applying template
    ErrChartTemplateFailed = 4112,
    /// Subwindow containing the indicator was not found
    ErrChartWindowNotFound = 4113,
    /// Error adding an indicator to chart
    ErrChartIndicatorCannotAdd = 4114,
    /// Error deleting an indicator from the chart
    ErrChartIndicatorCannotDel = 4115,
    /// Indicator not found on the specified chart
    ErrChartIndicatorNotFound = 4116,
    /// Error working with a graphical object
    ErrObjectError = 4201,
    /// Graphical object was not found
    ErrObjectNotFound = 4202,
    /// Wrong id of a graphical object property
    ErrObjectWrongProperty = 4203,
    /// Unable to get date corresponding to the value
    ErrObjectGetdateFailed = 4204,
    /// Unable to get value corresponding to the date
    ErrObjectGetvalueFailed = 4205,
    /// Unknown symbol
    ErrMarketUnknownSymbol = 4301,
    /// Symbol is not selected in MarketWatch
    ErrMarketNotSelected = 4302,
    /// Wrong identifier of a symbol property
    ErrMarketWrongProperty = 4303,
    /// Time of the last tick is not known (no ticks)
    ErrMarketLasttimeUnknown = 4304,
    /// Error adding or deleting a symbol in MarketWatch
    ErrMarketSelectError = 4305,
    /// Requested history not found
    ErrHistoryNotFound = 4401,
    /// Wrong id of the history property
    ErrHistoryWrongProperty = 4402,
    /// Exceeded history request timeout
    ErrHistoryTimeout = 4403,
    /// Number of requested bars limited by terminal settings
    ErrHistoryBarsLimit = 4404,
    /// Multiple Errors when loading history
    ErrHistoryLoadErrors = 4405,
    /// Receiving array is too small to store all requested data
    ErrHistorySmallBuffer = 4407,
    /// Global variable of the client terminal is not found
    ErrGlobalvariableNotFound = 4501,
    /// Global variable of the client terminal with the same name already exists
    ErrGlobalvariableExists = 4502,
    /// Global variables were not modified
    ErrGlobalvariableNotModified = 4503,
    /// Cannot read file with global variable values
    ErrGlobalvariableCannotread = 4504,
    /// Cannot write file with global variable values
    ErrGlobalvariableCannotwrite = 4505,
    /// Email sending failed
    ErrMailSendFailed = 4510,
    /// Sound playing failed
    ErrPlaySoundFailed = 4511,
    /// Wrong identifier of the program property
    ErrMql5WrongProperty = 4512,
    /// Wrong identifier of the terminal property
    ErrTerminalWrongProperty = 4513,
    /// File sending via ftp failed
    ErrFtpSendFailed = 4514,
    /// Failed to send a notification
    ErrNotificationSendFailed = 4515,
    /// Invalid parameter for sending a notification – an empty string or null has been passed to the SendNotification() function
    ErrNotificationWrongParameter = 4516,
    /// Wrong settings of notifications in the terminal (id is not specified or permission is not set)
    ErrNotificationWrongSettings = 4517,
    /// Too frequent sending of notifications
    ErrNotificationTooFrequent = 4518,
    /// ftp server is not specified
    ErrFtpNoserver = 4519,
    /// ftp login is not specified
    ErrFtpNologin = 4520,
    /// File not found in the mql5\Files directory to send on ftp server
    ErrFtpFileError = 4521,
    /// ftp connection failed
    ErrFtpConnectFailed = 4522,
    /// ftp path not found on server
    ErrFtpChangedir = 4523,
    /// ftp connection closed
    ErrFtpClosed = 4524,
    /// Not enough memory for the distribution of indicator buffers
    ErrBuffersNoMemory = 4601,
    /// Wrong indicator buffer index
    ErrBuffersWrongIndex = 4602,
    /// Wrong id of the custom indicator property
    ErrCustomWrongProperty = 4603,
    /// Wrong account property id
    ErrAccountWrongProperty = 4701,
    /// Wrong trade property id
    ErrTradeWrongProperty = 4751,
    /// Trading by Expert Advisors prohibited
    ErrTradeDisabled = 4752,
    /// Position not found
    ErrTradePositionNotFound = 4753,
    /// Order not found
    ErrTradeOrderNotFound = 4754,
    /// Deal not found
    ErrTradeDealNotFound = 4755,
    /// Trade request sending failed
    ErrTradeSendFailed = 4756,
    /// Failed to calculate profit or margin
    ErrTradeCalcFailed = 4758,
    /// Unknown symbol
    ErrIndicatorUnknownSymbol = 4801,
    /// Indicator cannot be created
    ErrIndicatorCannotCreate = 4802,
    /// Not enough memory to add the indicator
    ErrIndicatorNoMemory = 4803,
    /// The indicator cannot be applied to another indicator
    ErrIndicatorCannotApply = 4804,
    /// Error applying an indicator to chart
    ErrIndicatorCannotAdd = 4805,
    /// Requested data not found
    ErrIndicatorDataNotFound = 4806,
    /// Wrong indicator handle
    ErrIndicatorWrongHandle = 4807,
    /// Wrong number of parameters when creating an indicator
    ErrIndicatorWrongParameters = 4808,
    /// No parameters when creating an indicator
    ErrIndicatorParametersMissing = 4809,
    /// The first parameter in the array must be the name of the custom indicator
    ErrIndicatorCustomName = 4810,
    /// Invalid parameter type in the array when creating an indicator
    ErrIndicatorParameterType = 4811,
    /// Wrong index of the requested indicator buffer
    ErrIndicatorWrongIndex = 4812,
    /// Depth Of Market can not be added
    ErrBooksCannotAdd = 4901,
    /// Depth Of Market can not be removed
    ErrBooksCannotDelete = 4902,
    /// The data from Depth Of Market can not be obtained
    ErrBooksCannotGet = 4903,
    /// Error in subscribing to receive new data from Depth Of Market
    ErrBooksCannotSubscribe = 4904,
    /// More than 64 files cannot be opened at the same time
    ErrTooManyFiles = 5001,
    /// Invalid file name
    ErrWrongFilename = 5002,
    /// Too long file name
    ErrTooLongFilename = 5003,
    /// File opening Error
    ErrCannotOpenFile = 5004,
    /// Not enough memory for cache to read
    ErrFileCachebufferError = 5005,
    /// File deleting Error
    ErrCannotDeleteFile = 5006,
    /// A file with this handle was closed, or was not opening at all
    ErrInvalidFilehandle = 5007,
    /// Wrong file handle
    ErrWrongFilehandle = 5008,
    /// The file must be opened for writing
    ErrFileNottowrite = 5009,
    /// The file must be opened for reading
    ErrFileNottoread = 5010,
    /// The file must be opened as a binary one
    ErrFileNotbin = 5011,
    /// The file must be opened as a text
    ErrFileNottxt = 5012,
    /// The file must be opened as a text or csv
    ErrFileNottxtorcsv = 5013,
    /// The file must be opened as csv
    ErrFileNotcsv = 5014,
    /// File reading Error
    ErrFileReaderror = 5015,
    /// String size must be specified, because the file is opened as binary
    ErrFileBinstringsize = 5016,
    /// A text file must be for string arrays, for other arrays - binary
    ErrIncompatibleFile = 5017,
    /// This is not a file, this is a directory
    ErrFileIsDirectory = 5018,
    /// File does not exist
    ErrFileNotExist = 5019,
    /// File can not be rewritten
    ErrFileCannotRewrite = 5020,
    /// Wrong directory name
    ErrWrongDirectoryname = 5021,
    /// Directory does not exist
    ErrDirectoryNotExist = 5022,
    /// This is a file, not a directory
    ErrFileIsnotDirectory = 5023,
    /// The directory cannot be removed
    ErrCannotDeleteDirectory = 5024,
    /// Failed to clear the directory (probably one or more files are blocked and removal operation failed)
    ErrCannotCleanDirectory = 5025,
    /// Failed to write a resource to a file
    ErrFileWriteerror = 5026,
    /// Unable to read the next piece of data from a csv file (FileReadString, FileReadNumber, FileReadDatetime, FileReadBool), since the end of file is reached
    ErrFileEndoffile = 5027,
    /// No date in the string
    ErrNoStringDate = 5030,
    /// Wrong date in the string
    ErrWrongStringDate = 5031,
    /// Wrong time in the string
    ErrWrongStringTime = 5032,
    /// Error converting string to date
    ErrStringTimeError = 5033,
    /// Not enough memory for the string
    ErrStringOutOfMemory = 5034,
    /// The string length is less than expected
    ErrStringSmallLen = 5035,
    /// Too large number, more than ulongMax
    ErrStringTooBignumber = 5036,
    /// Invalid format string
    ErrWrongFormatstring = 5037,
    /// Amount of format specifiers more than the parameters
    ErrTooManyFormatters = 5038,
    /// Amount of parameters more than the format specifiers
    ErrTooManyParameters = 5039,
    /// Damaged parameter of string type
    ErrWrongStringParameter = 5040,
    /// Position outside the string
    ErrStringposOutofrange = 5041,
    /// 0 added to the string end, a useless operation
    ErrStringZeroadded = 5042,
    /// Unknown data type when converting to a string
    ErrStringUnknowntype = 5043,
    /// Damaged string object
    ErrWrongStringObject = 5044,
    /// Copying incompatible arrays. String array can be copied only to a string array, and a numeric array - in numeric array only
    ErrIncompatibleArrays = 5050,
    /// The receiving array is declared as asSeries, and it is of insufficient size
    ErrSmallAsseriesArray = 5051,
    /// Too small array, the starting position is outside the array
    ErrSmallArray = 5052,
    /// An array of zero length
    ErrZerosizeArray = 5053,
    /// Must be a numeric array
    ErrNumberArraysOnly = 5054,
    /// Must be a one-dimensional array
    ErrOnedimArraysOnly = 5055,
    /// Timeseries cannot be used
    ErrSeriesArray = 5056,
    /// Must be an array of type double
    ErrDoubleArrayOnly = 5057,
    /// Must be an array of type float
    ErrFloatArrayOnly = 5058,
    /// Must be an array of type long
    ErrLongArrayOnly = 5059,
    /// Must be an array of type int
    ErrIntArrayOnly = 5060,
    /// Must be an array of type short
    ErrShortArrayOnly = 5061,
    /// Must be an array of type char
    ErrCharArrayOnly = 5062,
    /// String array only
    ErrStringArrayOnly = 5063,
    /// Opencl functions are not supported on this computer
    ErrOpenclNotSupported = 5100,
    /// Internal Error occurred when running Opencl
    ErrOpenclInternal = 5101,
    /// Invalid Opencl handle
    ErrOpenclInvalidHandle = 5102,
    /// Error creating the Opencl context
    ErrOpenclContextCreate = 5103,
    /// Failed to create a run queue in Opencl
    ErrOpenclQueueCreate = 5104,
    /// Error occurred when compiling an Opencl program
    ErrOpenclProgramCreate = 5105,
    /// Too long kernel name (Opencl kernel)
    ErrOpenclTooLongKernelName = 5106,
    /// Error creating an Opencl kernel
    ErrOpenclKernelCreate = 5107,
    /// Error occurred when setting parameters for the Opencl kernel
    ErrOpenclSetKernelParameter = 5108,
    /// Opencl program runtime Error
    ErrOpenclExecute = 5109,
    /// Invalid size of the Opencl buffer
    ErrOpenclWrongBufferSize = 5110,
    /// Invalid offset in the Opencl buffer
    ErrOpenclWrongBufferOffset = 5111,
    /// Failed to create an Opencl buffer
    ErrOpenclBufferCreate = 5112,
    /// Too many Opencl objects
    ErrOpenclTooManyObjects = 5113,
    /// Opencl device selection Error
    ErrOpenclSelectdevice = 5114,
    /// Internal database Error
    ErrDatabaseInternal = 5120,
    /// Invalid database handle
    ErrDatabaseInvalidHandle = 5121,
    /// Exceeded the maximum acceptable number of Database objects
    ErrDatabaseTooManyObjects = 5122,
    /// Database connection Error
    ErrDatabaseConnect = 5123,
    /// Request execution Error
    ErrDatabaseExecute = 5124,
    /// Request generation Error
    ErrDatabasePrepare = 5125,
    /// No more data to read
    ErrDatabaseNoMoreData = 5126,
    /// Failed to move to the next request entry
    ErrDatabaseStep = 5127,
    /// Data for reading request results are not ready yet
    ErrDatabaseNotReady = 5128,
    /// Failed to auto substitute parameters to an sql request
    ErrDatabaseBindParameters = 5129,
    /// Invalid url
    ErrWebrequestInvalidAddress = 5200,
    /// Failed to connect to specified url
    ErrWebrequestConnectFailed = 5201,
    /// Timeout exceeded
    ErrWebrequestTimeout = 5202,
    /// http request failed
    ErrWebrequestRequestFailed = 5203,
    /// Invalid socket handle passed to function
    ErrNetsocketInvalidhandle = 5270,
    /// Too many open sockets (max 128)
    ErrNetsocketTooManyOpened = 5271,
    /// Failed to connect to remote host
    ErrNetsocketCannotConnect = 5272,
    /// Failed to send/receive data from socket
    ErrNetsocketIoError = 5273,
    /// Failed to establish secure connection (tls Handshake)
    ErrNetsocketHandshakeFailed = 5274,
    /// No data on certificate protecting the connection
    ErrNetsocketNoCertificate = 5275,
    /// A custom symbol must be specified
    ErrNotCustomSymbol = 5300,
    /// The name of the custom symbol is invalid. The symbol name can only contain Latin letters without punctuation, spaces or special characters (may only contain ".", "_", "&" and "#").\n    It is not recommended to use characters <, >, :, ", /,\, |, ?, *.
    ErrCustomSymbolWrongName = 5301,
    /// The name of the custom symbol is too long. The length of the symbol name must not exceed 32 characters including the ending 0 character
    ErrCustomSymbolNameLong = 5302,
    /// The path of the custom symbol is too long. The path length should not exceed 128 characters including "Custom\\", the symbol name, group separators and the ending 0
    ErrCustomSymbolPathLong = 5303,
    /// A custom symbol with the same name already exists
    ErrCustomSymbolExist = 5304,
    /// Error occurred while creating, deleting or changing the custom symbol
    ErrCustomSymbolError = 5305,
    /// You are trying to delete a custom symbol selected in Market Watch
    ErrCustomSymbolSelected = 5306,
    /// An invalid custom symbol property
    ErrCustomSymbolPropertyWrong = 5307,
    /// A wrong parameter while setting the property of a custom symbol
    ErrCustomSymbolParameterError = 5308,
    /// A too long string parameter while setting the property of a custom symbol
    ErrCustomSymbolParameterLong = 5309,
    /// Ticks in the array are not arranged in the order of time
    ErrCustomTicksWrongOrder = 5310,
    /// Array size is insufficient for receiving descriptions of all values
    ErrCalendarMoreData = 5400,
    /// Request time limit exceeded
    ErrCalendarTimeout = 5401,
    /// Country is not found
    ErrCalendarNoData = 5402,
    /// Generic Error
    ErrDatabaseError   = 5601,
    /// sqlite internal logic Error
    ErrSqLiteDatabaseInternal = 5602,
    /// Access denied
    ErrDatabasePerm = 5603,
    /// Callback routine requested abort
    ErrDatabaseAbort = 5604,
    /// Database file locked
    ErrDatabaseBusy = 5605,
    /// Database table locked
    ErrDatabaseLocked = 5606,
    /// Insufficient memory for completing operation
    ErrDatabaseNomem = 5607,
    /// Attempt to write to readonly database
    ErrDatabaseReadonly = 5608,
    /// Operation terminated by sqlite3_interrupt()
    ErrDatabaseInterrupt = 5609,
    /// Disk I/O Error
    ErrDatabaseIoerr = 5610,
    /// Database disk image corrupted
    ErrDatabaseCorrupt = 5611,
    /// Unknown operation code in sqlite3_file_control()
    ErrDatabaseNotfound = 5612,
    /// Insertion failed because database is full
    ErrDatabaseFull = 5613,
    /// Unable to open the database file
    ErrDatabaseCantopen = 5614,
    /// Database lock protocol Error
    ErrDatabaseProtocol = 5615,
    /// Internal use only
    ErrDatabaseEmpty = 5616,
    /// Database schema changed
    ErrDatabaseSchema = 5617,
    /// String or blob exceeds size limit
    ErrDatabaseToobig = 5618,
    /// Abort due to constraint violation
    ErrDatabaseConstraint = 5619,
    /// Data type mismatch
    ErrDatabaseMismatch = 5620,
    /// Library used incorrectly
    ErrDatabaseMisuse = 5621,
    /// Uses os features not supported on host
    ErrDatabaseNolfs = 5622,
    /// Authorization denied
    ErrDatabaseAuth = 5623,
    /// Not used
    ErrDatabaseFormat = 5624,
    /// Bind parameter Error, incorrect index
    ErrDatabaseRange = 5625,
    /// File opened that is not database file
    ErrDatabaseNotadb = 5626,
    /// User defined Errors start with this code
    ErrUserErrorFirst = 65536,

    /// [super::MqlTradeResult]'s `retcode` had an unknown number. See the MQL5 site for an update
    UnknownErrorCode = u32::MAX,
}
impl Into<u32> for Mt5ErrorCodes {
    fn into(self) -> u32 {
        self as u32
    }
}
impl From<u32> for Mt5ErrorCodes {
    fn from(variant_value: u32) -> Self {
        if let Some(variant) = Self::from_repr(variant_value) {
            return variant;
        }
        Self::UnknownErrorCode
    }
}

/// From the site: https://www.mql5.com/en/docs/constants/errorswarnings/enum_trade_return_codes :\
/// All requests to execute trade operations are sent as a structure of a trade request [super::MqlTradeRequest] using function `OrderSend()`.
/// The function execution result is placed to structure [super::MqlTradeResult], whose `retcode` field contains the trade server return code.
#[repr(u32)]
#[derive(Debug,PartialEq,EnumString,FromRepr,Clone,Copy)]
pub enum Mt5TradeServerReturnCodes {
    /// Requote
    TradeRetcodeRequote = 10004,
    /// Request rejected
    TradeRetcodeReject = 10006,
    /// Request canceled by trader
    TradeRetcodeCancel = 10007,
    /// Order placed
    TradeRetcodePlaced = 10008,
    /// Request completed
    TradeRetcodeDone = 10009,
    /// Only part of the request was completed
    TradeRetcodeDonePartial = 10010,
    /// Request processing Error
    TradeRetcodeError = 10011,
    /// Request canceled by timeout
    TradeRetcodeTimeout = 10012,
    /// Invalid request
    TradeRetcodeInvalid = 10013,
    /// Invalid volume in the request
    TradeRetcodeInvalidVolume = 10014,
    /// Invalid price in the request
    TradeRetcodeInvalidPrice = 10015,
    /// Invalid stops in the request
    TradeRetcodeInvalidStops = 10016,
    /// Trade is disabled
    TradeRetcodeTradeDisabled = 10017,
    /// Market is closed
    TradeRetcodeMarketClosed = 10018,
    /// There is not enough money to complete the request
    TradeRetcodeNoMoney = 10019,
    /// Prices changed
    TradeRetcodePriceChanged = 10020,
    /// There are no quotes to process the request
    TradeRetcodePriceOff = 10021,
    /// Invalid order expiration date in the request
    TradeRetcodeInvalidExpiration = 10022,
    /// Order state changed
    TradeRetcodeOrderChanged = 10023,
    /// Too frequent requests
    TradeRetcodeTooManyRequests = 10024,
    /// No changes in request
    TradeRetcodeNoChanges = 10025,
    /// Autotrading disabled by server
    TradeRetcodeServerDisablesAt = 10026,
    /// Autotrading disabled by client terminal
    TradeRetcodeClientDisablesAt = 10027,
    /// Request locked for processing
    TradeRetcodeLocked = 10028,
    /// Order or position frozen
    TradeRetcodeFrozen = 10029,
    /// Invalid order filling type
    TradeRetcodeInvalidFill = 10030,
    /// No connection with the trade server
    TradeRetcodeConnection = 10031,
    /// Operation is allowed only for live accounts
    TradeRetcodeOnlyReal = 10032,
    /// The number of pending orders has reached the limit
    TradeRetcodeLimitOrders = 10033,
    /// The volume of orders and positions for the symbol has reached the limit
    TradeRetcodeLimitVolume = 10034,
    /// Incorrect or prohibited order type
    TradeRetcodeInvalidOrder = 10035,
    /// Position with the specified POSITION_IDENTIFIER has already been closed
    TradeRetcodePositionClosed = 10036,
    /// A close volume exceeds the current position volume
    TradeRetcodeInvalidCloseVolume = 10038,
    /// A close order already exists for a specified position. This may happen when working in the hedging system:
    ///   - when attempting to close a position with an opposite one, while close orders for the position already exist
    ///   - when attempting to fully or partially close a position if the total volume of the already present close orders and the newly placed one exceeds the current position volume
    TradeRetcodeCloseOrderExist = 10039,
    /// The number of open positions simultaneously present on an account can be limited by the server settings. After a limit is reached, the server returns the TradeRetcodeLimitPositions Error when attempting to place an order.
    /// The limitation operates differently depending on the position accounting type:
    ///   - Netting — number of open positions is considered. When a limit is reached, the platform does not let placing new orders whose execution may increase the number of open positions. In fact, the platform allows placing
    ///     orders only for the symbolsthat already have open positions. The current pending orders are not considered since their execution may lead to changes in the current positions but it cannot increase their number.
    ///   - Hedging — pending orders are considered together with open positions, since a pending order activation always leads to opening a new position. When a limit is reached, the platform does not allow placing both new
    ///     market orders for opening positions and pending orders.
    TradeRetcodeLimitPositions = 10040,
    /// The pending order activation request is rejected, the order is canceled
    TradeRetcodeRejectCancel = 10041,
    /// The request is rejected, because the "Only long positions are allowed" rule is set for the symbol (POSITION_TYPE_BUY)
    TradeRetcodeLongOnly = 10042,
    /// The request is rejected, because the "Only short positions are allowed" rule is set for the symbol (POSITION_TYPE_SELL)
    TradeRetcodeShortOnly = 10043,
    /// The request is rejected, because the "Only position closing is allowed" rule is set for the symbol
    TradeRetcodeCloseOnly = 10044,
    /// The request is rejected, because "Position closing is allowed only by FIFO rule" flag is set for the trading account (ACCOUNT_FIFO_CLOSE=true)
    TradeRetcodeFifoClose = 10045,
    /// The request is rejected, because the "Opposite positions on a single symbol are disabled" rule is set for the trading account.
    /// For example, if the account has a Buy position, then a user cannot open a Sell position or place a pending sell order.
    /// The rule is only applied to accounts with hedging accounting system (ACCOUNT_MARGIN_MODE=ACCOUNT_MARGIN_MODE_RETAIL_HEDGING).
    TradeRetcodeHedgeProhibited = 10046,

    /// [super::MqlTradeResult]'s `retcode` had an unknown number. See the MQL5 site for an update
    UnknownRetcode = 0,
}
impl Into<u32> for Mt5TradeServerReturnCodes {
    fn into(self) -> u32 {
        self as u32
    }
}
impl From<u32> for Mt5TradeServerReturnCodes {
    fn from(variant_value: u32) -> Self {
        if let Some(variant) = Self::from_repr(variant_value) {
            return variant;
        }
        Self::UnknownRetcode
    }
}