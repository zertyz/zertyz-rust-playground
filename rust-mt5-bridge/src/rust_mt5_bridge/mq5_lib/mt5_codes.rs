//! Maps trading server constants, enums and structures -- as exposed by Metatrader

use strum::{EnumString,FromRepr};


#[repr(u32)]
#[derive(Debug,PartialEq,EnumString,FromRepr,Clone,Copy)]
pub enum Mt5ErrorCodes {
    /// The operation completed successfully
    errSuccess = 0,
    /// Unexpected internal error
    errInternalError = 4001,
    /// Wrong parameter in the inner call of the client terminal function
    errWrongInternalParameter = 4002,
    /// Wrong parameter when calling the system function
    errInvalidParameter = 4003,
    /// Not enough memory to perform the system function
    errNotEnoughMemory = 4004,
    /// The structure contains objects of strings and/or dynamic arrays and/or structure of such objects and/or classes
    errStructWithobjectsOrclass = 4005,
    /// Array of a wrong type, wrong size, or a damaged object of a dynamic array
    errInvalidArray = 4006,
    /// Not enough memory for the relocation of an array, or an attempt to change the size of a static array
    errArrayResizeError = 4007,
    /// Not enough memory for the relocation of string
    errStringResizeError = 4008,
    /// Not initialized string
    errNotinitializedString = 4009,
    /// Invalid date and/or time
    errInvalidDatetime = 4010,
    /// Total amount of elements in the array cannot exceed 2147483647
    errArrayBadSize = 4011,
    /// Wrong pointer
    errInvalidPointer = 4012,
    /// Wrong type of pointer
    errInvalidPointerType = 4013,
    /// Function is not allowed for call
    errFunctionNotAllowed = 4014,
    /// The names of the dynamic and the static resource match
    errResourceNameDuplicated = 4015,
    /// Resource with this name has not been found in ex5
    errResourceNotFound = 4016,
    /// Unsupported resource type or its size exceeds 16 Mb
    errResourceUnsuppotedType = 4017,
    /// The resource name exceeds 63 characters
    errResourceNameIsTooLong = 4018,
    /// Overflow occurred when calculating math function
    errMathOverflow = 4019,
    /// Out of test end date after calling Sleep()
    errSleepError = 4020,
    /// Test forcibly stopped from the outside. For example, optimization interrupted, visual testing window closed or testing agent stopped
    errProgramStopped = 4022,
    /// Wrong chart id
    errChartWrongId = 4101,
    /// Chart does not respond
    errChartNoReply = 4102,
    /// Chart not found
    errChartNotFound = 4103,
    /// No Expert Advisor in the chart that could handle the event
    errChartNoExpert = 4104,
    /// Chart opening error
    errChartCannotOpen = 4105,
    /// Failed to change chart symbol and period
    errChartCannotChange = 4106,
    /// Error value of the parameter for the function of working with charts
    errChartWrongParameter = 4107,
    /// Failed to create timer
    errChartCannotCreateTimer = 4108,
    /// Wrong chart property id
    errChartWrongProperty = 4109,
    /// Error creating screenshots
    errChartScreenshotFailed = 4110,
    /// Error navigating through chart
    errChartNavigateFailed = 4111,
    /// Error applying template
    errChartTemplateFailed = 4112,
    /// Subwindow containing the indicator was not found
    errChartWindowNotFound = 4113,
    /// Error adding an indicator to chart
    errChartIndicatorCannotAdd = 4114,
    /// Error deleting an indicator from the chart
    errChartIndicatorCannotDel = 4115,
    /// Indicator not found on the specified chart
    errChartIndicatorNotFound = 4116,
    /// Error working with a graphical object
    errObjectError = 4201,
    /// Graphical object was not found
    errObjectNotFound = 4202,
    /// Wrong id of a graphical object property
    errObjectWrongProperty = 4203,
    /// Unable to get date corresponding to the value
    errObjectGetdateFailed = 4204,
    /// Unable to get value corresponding to the date
    errObjectGetvalueFailed = 4205,
    /// Unknown symbol
    errMarketUnknownSymbol = 4301,
    /// Symbol is not selected in MarketWatch
    errMarketNotSelected = 4302,
    /// Wrong identifier of a symbol property
    errMarketWrongProperty = 4303,
    /// Time of the last tick is not known (no ticks)
    errMarketLasttimeUnknown = 4304,
    /// Error adding or deleting a symbol in MarketWatch
    errMarketSelectError = 4305,
    /// Requested history not found
    errHistoryNotFound = 4401,
    /// Wrong id of the history property
    errHistoryWrongProperty = 4402,
    /// Exceeded history request timeout
    errHistoryTimeout = 4403,
    /// Number of requested bars limited by terminal settings
    errHistoryBarsLimit = 4404,
    /// Multiple errors when loading history
    errHistoryLoadErrors = 4405,
    /// Receiving array is too small to store all requested data
    errHistorySmallBuffer = 4407,
    /// Global variable of the client terminal is not found
    errGlobalvariableNotFound = 4501,
    /// Global variable of the client terminal with the same name already exists
    errGlobalvariableExists = 4502,
    /// Global variables were not modified
    errGlobalvariableNotModified = 4503,
    /// Cannot read file with global variable values
    errGlobalvariableCannotread = 4504,
    /// Cannot write file with global variable values
    errGlobalvariableCannotwrite = 4505,
    /// Email sending failed
    errMailSendFailed = 4510,
    /// Sound playing failed
    errPlaySoundFailed = 4511,
    /// Wrong identifier of the program property
    errMql5WrongProperty = 4512,
    /// Wrong identifier of the terminal property
    errTerminalWrongProperty = 4513,
    /// File sending via ftp failed
    errFtpSendFailed = 4514,
    /// Failed to send a notification
    errNotificationSendFailed = 4515,
    /// Invalid parameter for sending a notification – an empty string or null has been passed to the SendNotification() function
    errNotificationWrongParameter = 4516,
    /// Wrong settings of notifications in the terminal (id is not specified or permission is not set)
    errNotificationWrongSettings = 4517,
    /// Too frequent sending of notifications
    errNotificationTooFrequent = 4518,
    /// ftp server is not specified
    errFtpNoserver = 4519,
    /// ftp login is not specified
    errFtpNologin = 4520,
    /// File not found in the mql5\Files directory to send on ftp server
    errFtpFileError = 4521,
    /// ftp connection failed
    errFtpConnectFailed = 4522,
    /// ftp path not found on server
    errFtpChangedir = 4523,
    /// ftp connection closed
    errFtpClosed = 4524,
    /// Not enough memory for the distribution of indicator buffers
    errBuffersNoMemory = 4601,
    /// Wrong indicator buffer index
    errBuffersWrongIndex = 4602,
    /// Wrong id of the custom indicator property
    errCustomWrongProperty = 4603,
    /// Wrong account property id
    errAccountWrongProperty = 4701,
    /// Wrong trade property id
    errTradeWrongProperty = 4751,
    /// Trading by Expert Advisors prohibited
    errTradeDisabled = 4752,
    /// Position not found
    errTradePositionNotFound = 4753,
    /// Order not found
    errTradeOrderNotFound = 4754,
    /// Deal not found
    errTradeDealNotFound = 4755,
    /// Trade request sending failed
    errTradeSendFailed = 4756,
    /// Failed to calculate profit or margin
    errTradeCalcFailed = 4758,
    /// Unknown symbol
    errIndicatorUnknownSymbol = 4801,
    /// Indicator cannot be created
    errIndicatorCannotCreate = 4802,
    /// Not enough memory to add the indicator
    errIndicatorNoMemory = 4803,
    /// The indicator cannot be applied to another indicator
    errIndicatorCannotApply = 4804,
    /// Error applying an indicator to chart
    errIndicatorCannotAdd = 4805,
    /// Requested data not found
    errIndicatorDataNotFound = 4806,
    /// Wrong indicator handle
    errIndicatorWrongHandle = 4807,
    /// Wrong number of parameters when creating an indicator
    errIndicatorWrongParameters = 4808,
    /// No parameters when creating an indicator
    errIndicatorParametersMissing = 4809,
    /// The first parameter in the array must be the name of the custom indicator
    errIndicatorCustomName = 4810,
    /// Invalid parameter type in the array when creating an indicator
    errIndicatorParameterType = 4811,
    /// Wrong index of the requested indicator buffer
    errIndicatorWrongIndex = 4812,
    /// Depth Of Market can not be added
    errBooksCannotAdd = 4901,
    /// Depth Of Market can not be removed
    errBooksCannotDelete = 4902,
    /// The data from Depth Of Market can not be obtained
    errBooksCannotGet = 4903,
    /// Error in subscribing to receive new data from Depth Of Market
    errBooksCannotSubscribe = 4904,
    /// More than 64 files cannot be opened at the same time
    errTooManyFiles = 5001,
    /// Invalid file name
    errWrongFilename = 5002,
    /// Too long file name
    errTooLongFilename = 5003,
    /// File opening error
    errCannotOpenFile = 5004,
    /// Not enough memory for cache to read
    errFileCachebufferError = 5005,
    /// File deleting error
    errCannotDeleteFile = 5006,
    /// A file with this handle was closed, or was not opening at all
    errInvalidFilehandle = 5007,
    /// Wrong file handle
    errWrongFilehandle = 5008,
    /// The file must be opened for writing
    errFileNottowrite = 5009,
    /// The file must be opened for reading
    errFileNottoread = 5010,
    /// The file must be opened as a binary one
    errFileNotbin = 5011,
    /// The file must be opened as a text
    errFileNottxt = 5012,
    /// The file must be opened as a text or csv
    errFileNottxtorcsv = 5013,
    /// The file must be opened as csv
    errFileNotcsv = 5014,
    /// File reading error
    errFileReaderror = 5015,
    /// String size must be specified, because the file is opened as binary
    errFileBinstringsize = 5016,
    /// A text file must be for string arrays, for other arrays - binary
    errIncompatibleFile = 5017,
    /// This is not a file, this is a directory
    errFileIsDirectory = 5018,
    /// File does not exist
    errFileNotExist = 5019,
    /// File can not be rewritten
    errFileCannotRewrite = 5020,
    /// Wrong directory name
    errWrongDirectoryname = 5021,
    /// Directory does not exist
    errDirectoryNotExist = 5022,
    /// This is a file, not a directory
    errFileIsnotDirectory = 5023,
    /// The directory cannot be removed
    errCannotDeleteDirectory = 5024,
    /// Failed to clear the directory (probably one or more files are blocked and removal operation failed)
    errCannotCleanDirectory = 5025,
    /// Failed to write a resource to a file
    errFileWriteerror = 5026,
    /// Unable to read the next piece of data from a csv file (FileReadString, FileReadNumber, FileReadDatetime, FileReadBool), since the end of file is reached
    errFileEndoffile = 5027,
    /// No date in the string
    errNoStringDate = 5030,
    /// Wrong date in the string
    errWrongStringDate = 5031,
    /// Wrong time in the string
    errWrongStringTime = 5032,
    /// Error converting string to date
    errStringTimeError = 5033,
    /// Not enough memory for the string
    errStringOutOfMemory = 5034,
    /// The string length is less than expected
    errStringSmallLen = 5035,
    /// Too large number, more than ulongMax
    errStringTooBignumber = 5036,
    /// Invalid format string
    errWrongFormatstring = 5037,
    /// Amount of format specifiers more than the parameters
    errTooManyFormatters = 5038,
    /// Amount of parameters more than the format specifiers
    errTooManyParameters = 5039,
    /// Damaged parameter of string type
    errWrongStringParameter = 5040,
    /// Position outside the string
    errStringposOutofrange = 5041,
    /// 0 added to the string end, a useless operation
    errStringZeroadded = 5042,
    /// Unknown data type when converting to a string
    errStringUnknowntype = 5043,
    /// Damaged string object
    errWrongStringObject = 5044,
    /// Copying incompatible arrays. String array can be copied only to a string array, and a numeric array - in numeric array only
    errIncompatibleArrays = 5050,
    /// The receiving array is declared as asSeries, and it is of insufficient size
    errSmallAsseriesArray = 5051,
    /// Too small array, the starting position is outside the array
    errSmallArray = 5052,
    /// An array of zero length
    errZerosizeArray = 5053,
    /// Must be a numeric array
    errNumberArraysOnly = 5054,
    /// Must be a one-dimensional array
    errOnedimArraysOnly = 5055,
    /// Timeseries cannot be used
    errSeriesArray = 5056,
    /// Must be an array of type double
    errDoubleArrayOnly = 5057,
    /// Must be an array of type float
    errFloatArrayOnly = 5058,
    /// Must be an array of type long
    errLongArrayOnly = 5059,
    /// Must be an array of type int
    errIntArrayOnly = 5060,
    /// Must be an array of type short
    errShortArrayOnly = 5061,
    /// Must be an array of type char
    errCharArrayOnly = 5062,
    /// String array only
    errStringArrayOnly = 5063,
    /// Opencl functions are not supported on this computer
    errOpenclNotSupported = 5100,
    /// Internal error occurred when running Opencl
    errOpenclInternal = 5101,
    /// Invalid Opencl handle
    errOpenclInvalidHandle = 5102,
    /// Error creating the Opencl context
    errOpenclContextCreate = 5103,
    /// Failed to create a run queue in Opencl
    errOpenclQueueCreate = 5104,
    /// Error occurred when compiling an Opencl program
    errOpenclProgramCreate = 5105,
    /// Too long kernel name (Opencl kernel)
    errOpenclTooLongKernelName = 5106,
    /// Error creating an Opencl kernel
    errOpenclKernelCreate = 5107,
    /// Error occurred when setting parameters for the Opencl kernel
    errOpenclSetKernelParameter = 5108,
    /// Opencl program runtime error
    errOpenclExecute = 5109,
    /// Invalid size of the Opencl buffer
    errOpenclWrongBufferSize = 5110,
    /// Invalid offset in the Opencl buffer
    errOpenclWrongBufferOffset = 5111,
    /// Failed to create an Opencl buffer
    errOpenclBufferCreate = 5112,
    /// Too many Opencl objects
    errOpenclTooManyObjects = 5113,
    /// Opencl device selection error
    errOpenclSelectdevice = 5114,
    /// Internal database error
    errDatabaseInternal = 5120,
    /// Invalid database handle
    errDatabaseInvalidHandle = 5121,
    /// Exceeded the maximum acceptable number of Database objects
    errDatabaseTooManyObjects = 5122,
    /// Database connection error
    errDatabaseConnect = 5123,
    /// Request execution error
    errDatabaseExecute = 5124,
    /// Request generation error
    errDatabasePrepare = 5125,
    /// No more data to read
    errDatabaseNoMoreData = 5126,
    /// Failed to move to the next request entry
    errDatabaseStep = 5127,
    /// Data for reading request results are not ready yet
    errDatabaseNotReady = 5128,
    /// Failed to auto substitute parameters to an sql request
    errDatabaseBindParameters = 5129,
    /// Invalid url
    errWebrequestInvalidAddress = 5200,
    /// Failed to connect to specified url
    errWebrequestConnectFailed = 5201,
    /// Timeout exceeded
    errWebrequestTimeout = 5202,
    /// http request failed
    errWebrequestRequestFailed = 5203,
    /// Invalid socket handle passed to function
    errNetsocketInvalidhandle = 5270,
    /// Too many open sockets (max 128)
    errNetsocketTooManyOpened = 5271,
    /// Failed to connect to remote host
    errNetsocketCannotConnect = 5272,
    /// Failed to send/receive data from socket
    errNetsocketIoError = 5273,
    /// Failed to establish secure connection (tls Handshake)
    errNetsocketHandshakeFailed = 5274,
    /// No data on certificate protecting the connection
    errNetsocketNoCertificate = 5275,
    /// A custom symbol must be specified
    errNotCustomSymbol = 5300,
    /// The name of the custom symbol is invalid. The symbol name can only contain Latin letters without punctuation, spaces or special characters (may only contain ".", "_", "&" and "#").\n    It is not recommended to use characters <, >, :, ", /,\, |, ?, *.
    errCustomSymbolWrongName = 5301,
    /// The name of the custom symbol is too long. The length of the symbol name must not exceed 32 characters including the ending 0 character
    errCustomSymbolNameLong = 5302,
    /// The path of the custom symbol is too long. The path length should not exceed 128 characters including "Custom\\", the symbol name, group separators and the ending 0
    errCustomSymbolPathLong = 5303,
    /// A custom symbol with the same name already exists
    errCustomSymbolExist = 5304,
    /// Error occurred while creating, deleting or changing the custom symbol
    errCustomSymbolError = 5305,
    /// You are trying to delete a custom symbol selected in Market Watch
    errCustomSymbolSelected = 5306,
    /// An invalid custom symbol property
    errCustomSymbolPropertyWrong = 5307,
    /// A wrong parameter while setting the property of a custom symbol
    errCustomSymbolParameterError = 5308,
    /// A too long string parameter while setting the property of a custom symbol
    errCustomSymbolParameterLong = 5309,
    /// Ticks in the array are not arranged in the order of time
    errCustomTicksWrongOrder = 5310,
    /// Array size is insufficient for receiving descriptions of all values
    errCalendarMoreData = 5400,
    /// Request time limit exceeded
    errCalendarTimeout = 5401,
    /// Country is not found
    errCalendarNoData = 5402,
    /// Generic error
    errDatabaseError   = 5601,
    /// sqlite internal logic error
    errSqLiteDatabaseInternal = 5602,
    /// Access denied
    errDatabasePerm = 5603,
    /// Callback routine requested abort
    errDatabaseAbort = 5604,
    /// Database file locked
    errDatabaseBusy = 5605,
    /// Database table locked
    errDatabaseLocked = 5606,
    /// Insufficient memory for completing operation
    errDatabaseNomem = 5607,
    /// Attempt to write to readonly database
    errDatabaseReadonly = 5608,
    /// Operation terminated by sqlite3_interrupt()
    errDatabaseInterrupt = 5609,
    /// Disk I/O error
    errDatabaseIoerr = 5610,
    /// Database disk image corrupted
    errDatabaseCorrupt = 5611,
    /// Unknown operation code in sqlite3_file_control()
    errDatabaseNotfound = 5612,
    /// Insertion failed because database is full
    errDatabaseFull = 5613,
    /// Unable to open the database file
    errDatabaseCantopen = 5614,
    /// Database lock protocol error
    errDatabaseProtocol = 5615,
    /// Internal use only
    errDatabaseEmpty = 5616,
    /// Database schema changed
    errDatabaseSchema = 5617,
    /// String or blob exceeds size limit
    errDatabaseToobig = 5618,
    /// Abort due to constraint violation
    errDatabaseConstraint = 5619,
    /// Data type mismatch
    errDatabaseMismatch = 5620,
    /// Library used incorrectly
    errDatabaseMisuse = 5621,
    /// Uses os features not supported on host
    errDatabaseNolfs = 5622,
    /// Authorization denied
    errDatabaseAuth = 5623,
    /// Not used
    errDatabaseFormat = 5624,
    /// Bind parameter error, incorrect index
    errDatabaseRange = 5625,
    /// File opened that is not database file
    errDatabaseNotadb = 5626,
    /// User defined errors start with this code
    errUserErrorFirst = 65536,

    /// [super::MqlTradeResult]'s `retcode` had an unknown number. See the MQL5 site for an update
    UnknownErrorcode = u32::MAX,
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
        Self::UnknownErrorcode
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
    /// Request processing error
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
    /// The number of open positions simultaneously present on an account can be limited by the server settings. After a limit is reached, the server returns the TradeRetcodeLimitPositions error when attempting to place an order.
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