use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt;

// ============================================================
// Data types
// ============================================================

/// 数据类型 (DataType.cs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum DataType {
    Bit = 0,
    UInt16 = 1,
    Int16 = 2,
    Bcd16 = 3,
    Hex16 = 4,
    Binary16 = 5,
    UInt32 = 11,
    Int32 = 12,
    Bcd32 = 13,
    Hex32 = 14,
    Binary32 = 15,
    Single = 16,
    UInt64 = 21,
    Int64 = 22,
    Bcd64 = 23,
    Hex64 = 24,
    Binary64 = 25,
    Double = 26,
    String = 30,
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bit => write!(f, "Bit"),
            Self::UInt16 => write!(f, "UInt16"),
            Self::Int16 => write!(f, "Int16"),
            Self::Bcd16 => write!(f, "BCD16"),
            Self::Hex16 => write!(f, "Hex16"),
            Self::Binary16 => write!(f, "Binary16"),
            Self::UInt32 => write!(f, "UInt32"),
            Self::Int32 => write!(f, "Int32"),
            Self::Bcd32 => write!(f, "BCD32"),
            Self::Hex32 => write!(f, "Hex32"),
            Self::Binary32 => write!(f, "Binary32"),
            Self::Single => write!(f, "Float"),
            Self::UInt64 => write!(f, "UInt64"),
            Self::Int64 => write!(f, "Int64"),
            Self::Bcd64 => write!(f, "BCD64"),
            Self::Hex64 => write!(f, "Hex64"),
            Self::Binary64 => write!(f, "Binary64"),
            Self::Double => write!(f, "Double"),
            Self::String => write!(f, "String"),
        }
    }
}

/// 数据宽度类型 (DataWidth.cs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum DataWidth {
    Bit = 0,
    Byte = 1,
    Word = 2,
    DWord = 4,
    QWord = 8,
}

// ============================================================
// Box (FBox device)
// ============================================================

/// 盒子连接状态 (BoxConnectionState.cs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum BoxConnectionState {
    Undetermined = 0,
    Connected = 1,
    TimedOut = 2,
    Disconnected = 3,
    Unavailable = 4,
    #[serde(other)]
    Unknown = -1,
}

impl fmt::Display for BoxConnectionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Undetermined => write!(f, "Unknown"),
            Self::Connected => write!(f, "Online"),
            Self::TimedOut => write!(f, "TimedOut"),
            Self::Disconnected => write!(f, "Offline"),
            Self::Unavailable => write!(f, "Unavailable"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

/// 盒子类型 (Box.cs - BoxTypes)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum BoxType {
    Standard = 0,
    Mini = 1,
    Lite = 2,
    Vpn = 3,
    LiteT = 4,
    FLink = 5,
    FL3 = 6,
    LiteNew = 7,
    HMI = 8,
    LiteV5 = 9,
    Q0 = 10,
    Desktop = 11,
    #[serde(other)]
    Unknown = -1,
}

impl fmt::Display for BoxType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Standard => write!(f, "Standard"),
            Self::Mini => write!(f, "Mini"),
            Self::Lite => write!(f, "Lite"),
            Self::Vpn => write!(f, "VPN"),
            Self::LiteT => write!(f, "Lite-T"),
            Self::FLink => write!(f, "FLink"),
            Self::FL3 => write!(f, "FL3"),
            Self::LiteNew => write!(f, "Lite-New"),
            Self::HMI => write!(f, "HMI"),
            Self::LiteV5 => write!(f, "Lite-V5"),
            Self::Q0 => write!(f, "Q0"),
            Self::Desktop => write!(f, "Desktop"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

/// 网络类型 (BoxConnectionStateItem.cs - NetworkTypes)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum NetworkType {
    None = 0,
    Ethernet = 1,
    SecondGen = 2,
    ThirdGen = 3,
    Wifi = 4,
    FourthGen = 5,
    #[serde(other)]
    Unknown = -1,
}

impl fmt::Display for NetworkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "-"),
            Self::Ethernet => write!(f, "Ethernet"),
            Self::SecondGen => write!(f, "2G"),
            Self::ThirdGen => write!(f, "3G"),
            Self::Wifi => write!(f, "WiFi"),
            Self::FourthGen => write!(f, "4G"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

// ============================================================
// Monitoring point (DMon)
// ============================================================

/// 监控点数据状态 (DMonStatus.cs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum DMonStatus {
    Normal = 0,
    NoValue = 1,
    Timeout = 2,
    Error = 3,
    SocketError = 4,
    FdsError = 5,
    Unfinished = 16,
}

impl fmt::Display for DMonStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Normal => write!(f, "Normal"),
            Self::NoValue => write!(f, "NoValue"),
            Self::Timeout => write!(f, "Timeout"),
            Self::Error => write!(f, "Error"),
            Self::SocketError => write!(f, "SocketError"),
            Self::FdsError => write!(f, "FdsError"),
            Self::Unfinished => write!(f, "Unfinished"),
        }
    }
}

/// 读写权限 (PrivilegeType.cs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum PrivilegeType {
    Write = 2,
    ReadOnly = 4,
    ReadWrite = 6,
}

impl fmt::Display for PrivilegeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Write => write!(f, "WriteOnly"),
            Self::ReadOnly => write!(f, "ReadOnly"),
            Self::ReadWrite => write!(f, "ReadWrite"),
        }
    }
}

/// 写入值类型 (WriteValueType.cs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum WriteValueType {
    AutoParse = 0,
    Decimal = 1,
    String = 2,
}

// ============================================================
// Alarm
// ============================================================

/// 报警条件类型 (AlarmConditionType.cs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum AlarmConditionType {
    Neq = 0,
    Eq = 1,
    Gt = 2,
    Gte = 3,
    Lt = 4,
    Lte = 5,
}

impl fmt::Display for AlarmConditionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Neq => write!(f, "!="),
            Self::Eq => write!(f, "=="),
            Self::Gt => write!(f, ">"),
            Self::Gte => write!(f, ">="),
            Self::Lt => write!(f, "<"),
            Self::Lte => write!(f, "<="),
        }
    }
}

/// 报警条件组合方式 (AlarmConditionCombineMethod.cs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum AlarmConditionCombineMethod {
    None = 0,
    And = 1,
    Or = 2,
}

/// 报警状态 (AlarmState.cs) - Flags
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum AlarmState {
    Unknown = 0,
    Triggered = 1,
    Recovered = 2,
    Confirmed = 4,
}

impl fmt::Display for AlarmState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "Unknown"),
            Self::Triggered => write!(f, "Triggered"),
            Self::Recovered => write!(f, "Recovered"),
            Self::Confirmed => write!(f, "Confirmed"),
        }
    }
}

/// 报警动作 (AlarmAction.cs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum AlarmAction {
    None = 0,
    Trigger = 1,
    Confirm = 2,
    Recover = 3,
}

impl fmt::Display for AlarmAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Trigger => write!(f, "Trigger"),
            Self::Confirm => write!(f, "Confirm"),
            Self::Recover => write!(f, "Recover"),
        }
    }
}

// ============================================================
// Contact
// ============================================================

/// 通知类型 (NoticeType.cs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum NoticeType {
    None = 0,
    Sms = 1,
    Voice = 2,
    SmsVoice = 3,
}

impl fmt::Display for NoticeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Sms => write!(f, "SMS"),
            Self::Voice => write!(f, "Voice"),
            Self::SmsVoice => write!(f, "SMS+Voice"),
        }
    }
}

// ============================================================
// History
// ============================================================

/// 历史数据区间类型 (TimeRangeTypes.cs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum TimeRangeType {
    BeginOpenEndOpen = 0,
    BeginOpenEndClose = 1,
    BeginCloseEndOpen = 2,
    BeginCloseEndClose = 3,
}

/// 历史记录采样方式 (HdataItemDto.cs - HistorySampleType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum HistorySampleType {
    Cycle = 0,
    Trigger = 1,
}

// ============================================================
// Device
// ============================================================

/// 设备连接类型 (ServerType.cs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum DeviceConnectionType {
    Serial = 0,
    Ethernet = 2,
}

impl fmt::Display for DeviceConnectionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Serial => write!(f, "Serial"),
            Self::Ethernet => write!(f, "Ethernet"),
        }
    }
}

/// 串口校验模式 (ParityType.cs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum ParityType {
    None = 0,
    Odd = 1,
    Even = 2,
}

/// 串口工作模式 (WorkModel.cs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum WorkModel {
    RS232 = 0,
    RS485_4 = 1,
    RS485_2 = 2,
}

/// 主从设备 (PlcClass.cs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum PlcClass {
    Master = 0,
    Slave = 1,
    MasterSlave = 2,
}

// ============================================================
// Write status
// ============================================================

/// 写值状态 (WriteStatus.cs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum WriteStatus {
    None = 0,
    Writing = 1,
    Success = 2,
    Failed = 3,
    DeviceHasRemoved = 31,
    BoxOffLine = 32,
    NoPermission = 33,
    DmonOnlyRead = 34,
    FirmwareNotSupport = 35,
    DataTypeNotMatch = 36,
    OutOfRange = 37,
    WritingValueTimeOut = 38,
}

impl fmt::Display for WriteStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Writing => write!(f, "Writing"),
            Self::Success => write!(f, "Success"),
            Self::Failed => write!(f, "Failed"),
            Self::DeviceHasRemoved => write!(f, "DeviceRemoved"),
            Self::BoxOffLine => write!(f, "BoxOffline"),
            Self::NoPermission => write!(f, "NoPermission"),
            Self::DmonOnlyRead => write!(f, "ReadOnly"),
            Self::FirmwareNotSupport => write!(f, "FirmwareNotSupport"),
            Self::DataTypeNotMatch => write!(f, "TypeMismatch"),
            Self::OutOfRange => write!(f, "OutOfRange"),
            Self::WritingValueTimeOut => write!(f, "Timeout"),
        }
    }
}

// ============================================================
// Encoding
// ============================================================

/// 编码类型 (EncodeType.cs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum EncodeType {
    None = 0,
    Unicode = 1,
    Ascii = 2,
    GB2312 = 3,
}

/// 地址进制类型 (AddressRadixType.cs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum AddressRadixType {
    None = 0,
    Oct = 8,
    Dec = 10,
    Hex = 16,
}
