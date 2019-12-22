// Buttplug Rust Source Code File - See https://buttplug.io for more info.
//
// Copyright 2016-2019 Nonpolynomial Labs LLC. All rights reserved.
//
// Licensed under the BSD 3-Clause license. See LICENSE file in the project root
// for full license information.

//! Structs representing low level [Buttplug
//! Protocol](https://buttplug-spec.docs.buttplug.io) messages

use super::errors::*;
use crate::devices::Endpoint;
#[cfg(feature = "serialize_json")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serialize_json")]
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::HashMap;

/// Base trait for all Buttplug Protocol Message Structs. Handles management of
/// message ids, as well as implementing conveinence functions for converting
/// between message structs and [ButtplugMessageUnion] enums, serialization, etc...
pub trait ButtplugMessage: Send + Sync + Clone {
    /// Returns the id number of the message
    fn get_id(&self) -> u32;
    /// Sets the id number of the message
    fn set_id(&mut self, id: u32);
    /// Returns the message as a [ButtplugMessageUnion] enum.
    fn as_union(self) -> ButtplugMessageUnion;
    /// Returns the message as a string in Buttplug JSON Protocol format.
    #[cfg(feature = "serialize_json")]
    fn as_protocol_json(self) -> String
    where
        Self: ButtplugMessage + Serialize + Deserialize<'static>,
    {
        "[".to_owned() + &serde_json::to_string(&self).unwrap() + "]"
    }
}

/// Represents the Buttplug Protocol Ok message, as documented in the [Buttplug
/// Protocol Spec](https://buttplug-spec.docs.buttplug.io/status.html#ok).
#[derive(Debug, PartialEq, Default, ButtplugMessage, Clone)]
#[cfg_attr(feature = "serialize_json", derive(Serialize, Deserialize))]
pub struct Ok {
    /// Message Id, used for matching message pairs in remote connection instances.
    #[cfg_attr(feature = "serialize_json", serde(rename = "Id"))]
    id: u32,
}

impl Ok {
    /// Creates a new Ok message with the given Id.
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}

/// Error codes pertaining to error classes that can be represented in the
/// Buttplug [Error] message.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_json", derive(Serialize_repr, Deserialize_repr))]
#[repr(u8)]
pub enum ErrorCode {
    ErrorUnknown = 0,
    ErrorHandshake,
    ErrorPing,
    ErrorMessage,
    ErrorDevice,
}

/// Represents the Buttplug Protocol Error message, as documented in the [Buttplug
/// Protocol Spec](https://buttplug-spec.docs.buttplug.io/status.html#error).
#[derive(Debug, ButtplugMessage, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_json", derive(Serialize, Deserialize))]
pub struct Error {
    /// Message Id, used for matching message pairs in remote connection instances.
    #[cfg_attr(feature = "serialize_json", serde(rename = "Id"))]
    id: u32,
    /// Specifies the class of the error.
    #[cfg_attr(feature = "serialize_json", serde(rename = "ErrorCode"))]
    pub error_code: ErrorCode,
    /// Description of the error.
    #[cfg_attr(feature = "serialize_json", serde(rename = "ErrorMessage"))]
    pub error_message: String,
}

impl Error {
    /// Creates a new error object.
    pub fn new(error_code: ErrorCode, error_message: &str) -> Self {
        Self {
            id: 0,
            error_code,
            error_message: error_message.to_string(),
        }
    }
}

impl From<ButtplugError> for Error {
    /// Converts a [super::errors::ButtplugError] object into a Buttplug Protocol
    /// [Error] message.
    fn from(error: ButtplugError) -> Self {
        let code = match error {
            ButtplugError::ButtplugDeviceError(_) => ErrorCode::ErrorDevice,
            ButtplugError::ButtplugMessageError(_) => ErrorCode::ErrorMessage,
            ButtplugError::ButtplugPingError(_) => ErrorCode::ErrorPing,
            ButtplugError::ButtplugHandshakeError(_) => ErrorCode::ErrorHandshake,
            ButtplugError::ButtplugUnknownError(_) => ErrorCode::ErrorUnknown,
        };
        // Gross but was having problems with naming collisions on the error trait
        let msg = match error {
            ButtplugError::ButtplugDeviceError(_s) => _s.message,
            ButtplugError::ButtplugMessageError(_s) => _s.message,
            ButtplugError::ButtplugPingError(_s) => _s.message,
            ButtplugError::ButtplugHandshakeError(_s) => _s.message,
            ButtplugError::ButtplugUnknownError(_s) => _s.message,
        };
        Error::new(code, &msg)
    }
}

#[derive(Debug, ButtplugMessage, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_json", derive(Serialize, Deserialize))]
pub struct Ping {
    /// Message Id, used for matching message pairs in remote connection instances.
    #[cfg_attr(feature = "serialize_json", serde(rename = "Id"))]
    id: u32,
}

impl Default for Ping {
    /// Creates a new Ping message with the given Id.
    fn default() -> Self {
        Self { id: 1 }
    }
}

#[derive(Debug, Default, ButtplugMessage, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_json", derive(Serialize, Deserialize))]
pub struct Test {
    /// Message Id, used for matching message pairs in remote connection instances.
    #[cfg_attr(feature = "serialize_json", serde(rename = "Id"))]
    id: u32,
    /// Test string, which will be echo'd back to client when sent to server.
    #[cfg_attr(feature = "serialize_json", serde(rename = "TestString"))]
    test_string: String,
}

impl Test {
    /// Creates a new Ping message with the given Id.
    pub fn new(test: &str) -> Self {
        Self {
            id: 1,
            test_string: test.to_owned(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize_json", derive(Serialize, Deserialize))]
pub struct MessageAttributes {
    #[cfg_attr(feature = "serialize_json", serde(rename = "FeatureCount"))]
    pub feature_count: Option<u32>,
    #[cfg_attr(feature = "serialize_json", serde(rename = "StepCount"))]
    pub step_count: Option<Vec<u32>>,
    #[cfg_attr(feature = "serialize_json", serde(rename = "Endpoints"))]
    pub endpoints: Option<Vec<Endpoint>>,
    #[cfg_attr(feature = "serialize_json", serde(rename = "MaxDuration"))]
    pub max_duration: Option<Vec<u32>>,
    #[cfg_attr(feature = "serialize_json", serde(rename = "Patterns"))]
    pub patterns: Option<Vec<Vec<String>>>,
    #[cfg_attr(feature = "serialize_json", serde(rename = "ActuatorType"))]
    pub actuator_type: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize_json", derive(Serialize, Deserialize))]
pub struct DeviceMessageInfo {
    #[cfg_attr(feature = "serialize_json", serde(rename = "DeviceIndex"))]
    pub device_index: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "DeviceName"))]
    pub device_name: String,
    #[cfg_attr(feature = "serialize_json", serde(rename = "DeviceMessages"))]
    pub device_messages: HashMap<String, MessageAttributes>,
}

impl From<&DeviceAdded> for DeviceMessageInfo {
    fn from(device_added: &DeviceAdded) -> Self {
        Self {
            device_index: device_added.device_index,
            device_name: device_added.device_name.clone(),
            device_messages: device_added.device_messages.clone(),
        }
    }
}

#[derive(Default, ButtplugMessage, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize_json", derive(Serialize, Deserialize))]
pub struct DeviceList {
    #[cfg_attr(feature = "serialize_json", serde(rename = "Id"))]
    id: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "Devices"))]
    pub devices: Vec<DeviceMessageInfo>,
}

#[derive(Default, ButtplugMessage, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serialize_json", derive(Serialize, Deserialize))]
pub struct DeviceAdded {
    #[cfg_attr(feature = "serialize_json", serde(rename = "Id"))]
    id: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "DeviceIndex"))]
    pub device_index: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "DeviceName"))]
    pub device_name: String,
    #[cfg_attr(feature = "serialize_json", serde(rename = "DeviceMessages"))]
    pub device_messages: HashMap<String, MessageAttributes>,
}

#[derive(Debug, Default, ButtplugMessage, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_json", derive(Serialize, Deserialize))]
pub struct DeviceRemoved {
    #[cfg_attr(feature = "serialize_json", serde(rename = "Id"))]
    id: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "DeviceIndex"))]
    pub device_index: u32,
}

#[derive(Debug, ButtplugMessage, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_json", derive(Serialize, Deserialize))]
pub struct StartScanning {
    #[cfg_attr(feature = "serialize_json", serde(rename = "Id"))]
    id: u32,
}

impl Default for StartScanning {
    fn default() -> Self {
        Self { id: 1 }
    }
}

#[derive(Debug, ButtplugMessage, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_json", derive(Serialize, Deserialize))]
pub struct StopScanning {
    #[cfg_attr(feature = "serialize_json", serde(rename = "Id"))]
    id: u32,
}

impl Default for StopScanning {
    fn default() -> Self {
        Self { id: 1 }
    }
}

#[derive(Debug, Default, ButtplugMessage, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_json", derive(Serialize, Deserialize))]
pub struct ScanningFinished {
    #[cfg_attr(feature = "serialize_json", serde(rename = "Id"))]
    id: u32,
}

#[derive(Debug, ButtplugMessage, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_json", derive(Serialize, Deserialize))]
pub struct RequestDeviceList {
    #[cfg_attr(feature = "serialize_json", serde(rename = "Id"))]
    id: u32,
}

impl Default for RequestDeviceList {
    fn default() -> Self {
        Self { id: 1 }
    }
}

#[derive(Debug, Default, ButtplugMessage, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_json", derive(Serialize, Deserialize))]
pub struct RequestServerInfo {
    #[cfg_attr(feature = "serialize_json", serde(rename = "Id"))]
    id: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "ClientName"))]
    pub client_name: String,
    #[cfg_attr(feature = "serialize_json", serde(rename = "MessageVersion"))]
    pub message_version: u32,
}

impl RequestServerInfo {
    pub fn new(client_name: &str, message_version: u32) -> Self {
        Self {
            id: 1,
            client_name: client_name.to_string(),
            message_version,
        }
    }
}

#[derive(Debug, Default, ButtplugMessage, PartialEq, Clone)]
#[cfg_attr(feature = "serialize_json", derive(Serialize, Deserialize))]
pub struct ServerInfo {
    #[cfg_attr(feature = "serialize_json", serde(rename = "Id"))]
    id: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "MajorVersion"))]
    pub major_version: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "MinorVersion"))]
    pub minor_version: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "BuildVersion"))]
    pub build_version: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "MessageVersion"))]
    pub message_version: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "MaxPingTime"))]
    pub max_ping_time: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "ServerName"))]
    pub server_name: String,
}

impl ServerInfo {
    pub fn new(server_name: &str, message_version: u32, max_ping_time: u32) -> Self {
        Self {
            id: 0,
            major_version: 0,
            minor_version: 0,
            build_version: 0,
            message_version,
            max_ping_time,
            server_name: server_name.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serialize_json", derive(Serialize, Deserialize))]
pub enum LogLevel {
    Off = 0,
    Fatal,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[derive(Debug, ButtplugMessage, PartialEq, Clone)]
#[cfg_attr(feature = "serialize_json", derive(Serialize, Deserialize))]
pub struct RequestLog {
    #[cfg_attr(feature = "serialize_json", serde(rename = "Id"))]
    id: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "LogLevel"))]
    pub log_level: LogLevel,
}

impl RequestLog {
    pub fn new(log_level: LogLevel) -> Self {
        Self { id: 1, log_level }
    }
}

#[derive(Debug, ButtplugMessage, PartialEq, Clone)]
#[cfg_attr(feature = "serialize_json", derive(Serialize, Deserialize))]
pub struct Log {
    #[cfg_attr(feature = "serialize_json", serde(rename = "Id"))]
    id: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "LogLevel"))]
    pub log_level: LogLevel,
    #[cfg_attr(feature = "serialize_json", serde(rename = "LogMessage"))]
    pub log_message: String,
}

impl Log {
    pub fn new(log_level: LogLevel, log_message: String) -> Self {
        Self {
            id: 0,
            log_level,
            log_message,
        }
    }
}

#[derive(Debug, Default, ButtplugMessage, PartialEq, Clone)]
#[cfg_attr(feature = "serialize_json", derive(Serialize, Deserialize))]
pub struct StopDeviceCmd {
    #[cfg_attr(feature = "serialize_json", serde(rename = "Id"))]
    pub id: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "DeviceIndex"))]
    pub device_index: u32,
}

impl StopDeviceCmd {
    pub fn new(device_index: u32) -> Self {
        Self {
            id: 1,
            device_index,
        }
    }
}

#[derive(Debug, Default, ButtplugMessage, PartialEq, Clone)]
#[cfg_attr(feature = "serialize_json", derive(Serialize, Deserialize))]
pub struct StopAllDevices {
    #[cfg_attr(feature = "serialize_json", serde(rename = "Id"))]
    pub id: u32,
}

#[derive(Debug, Default, PartialEq, Clone)]
#[cfg_attr(feature = "serialize_json", derive(Serialize, Deserialize))]
pub struct VibrateSubcommand {
    #[cfg_attr(feature = "serialize_json", serde(rename = "Index"))]
    pub index: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "Speed"))]
    pub speed: f64,
}

impl VibrateSubcommand {
    pub fn new(index: u32, speed: f64) -> Self {
        Self { index, speed }
    }
}

#[derive(Debug, Default, ButtplugMessage, PartialEq, Clone)]
#[cfg_attr(feature = "serialize_json", derive(Serialize, Deserialize))]
pub struct VibrateCmd {
    #[cfg_attr(feature = "serialize_json", serde(rename = "Id"))]
    pub id: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "DeviceIndex"))]
    pub device_index: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "Speeds"))]
    pub speeds: Vec<VibrateSubcommand>,
}

impl VibrateCmd {
    pub fn new(device_index: u32, speeds: Vec<VibrateSubcommand>) -> Self {
        Self {
            id: 1,
            device_index,
            speeds,
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
#[cfg_attr(feature = "serialize_json", derive(Serialize, Deserialize))]
pub struct VectorSubcommand {
    #[cfg_attr(feature = "serialize_json", serde(rename = "Index"))]
    pub index: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "Duration"))]
    pub duration: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "Position"))]
    pub position: f64,
}

impl VectorSubcommand {
    pub fn new(index: u32, duration: u32, position: f64) -> Self {
        Self {
            index,
            duration,
            position,
        }
    }
}

#[derive(Debug, Default, ButtplugMessage, PartialEq, Clone)]
#[cfg_attr(feature = "serialize_json", derive(Serialize, Deserialize))]
pub struct LinearCmd {
    #[cfg_attr(feature = "serialize_json", serde(rename = "Id"))]
    pub id: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "DeviceIndex"))]
    pub device_index: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "Vectors"))]
    pub vectors: Vec<VectorSubcommand>,
}

impl LinearCmd {
    pub fn new(device_index: u32, vectors: Vec<VectorSubcommand>) -> Self {
        Self {
            id: 1,
            device_index,
            vectors,
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
#[cfg_attr(feature = "serialize_json", derive(Serialize, Deserialize))]
pub struct RotationSubcommand {
    #[cfg_attr(feature = "serialize_json", serde(rename = "Index"))]
    pub index: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "Speed"))]
    pub speed: f64,
    #[cfg_attr(feature = "serialize_json", serde(rename = "Clockwise"))]
    pub clockwise: bool,
}

impl RotationSubcommand {
    pub fn new(index: u32, speed: f64, clockwise: bool) -> Self {
        Self {
            index,
            speed,
            clockwise,
        }
    }
}

#[derive(Debug, Default, ButtplugMessage, PartialEq, Clone)]
#[cfg_attr(feature = "serialize_json", derive(Serialize, Deserialize))]
pub struct RotateCmd {
    #[cfg_attr(feature = "serialize_json", serde(rename = "Id"))]
    pub id: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "DeviceIndex"))]
    pub device_index: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "Rotations"))]
    pub rotations: Vec<RotationSubcommand>,
}

impl RotateCmd {
    pub fn new(device_index: u32, rotations: Vec<RotationSubcommand>) -> Self {
        Self {
            id: 1,
            device_index,
            rotations,
        }
    }
}

#[derive(Debug, Default, ButtplugMessage, PartialEq, Clone)]
#[cfg_attr(feature = "serialize_json", derive(Serialize, Deserialize))]
pub struct FleshlightLaunchFW12Cmd {
    #[cfg_attr(feature = "serialize_json", serde(rename = "Id"))]
    pub id: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "DeviceIndex"))]
    pub device_index: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "Position"))]
    pub position: u8,
    #[cfg_attr(feature = "serialize_json", serde(rename = "Speed"))]
    pub speed: u8,
}

impl FleshlightLaunchFW12Cmd {
    pub fn new(device_index: u32, position: u8, speed: u8) -> Self {
        Self {
            id: 1,
            device_index,
            position,
            speed,
        }
    }
}

#[derive(Debug, ButtplugMessage, PartialEq, Clone)]
#[cfg_attr(feature = "serialize_json", derive(Serialize, Deserialize))]
pub struct LovenseCmd {
    #[cfg_attr(feature = "serialize_json", serde(rename = "Id"))]
    pub id: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "DeviceIndex"))]
    pub device_index: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "Command"))]
    pub command: String,
}

impl LovenseCmd {
    pub fn new(device_index: u32, command: &str) -> Self {
        Self {
            id: 1,
            device_index,
            command: command.to_owned(),
        }
    }
}

// Dear god this needs to be deprecated
#[derive(Debug, ButtplugMessage, PartialEq, Clone)]
#[cfg_attr(feature = "serialize_json", derive(Serialize, Deserialize))]
pub struct KiirooCmd {
    #[cfg_attr(feature = "serialize_json", serde(rename = "Id"))]
    pub id: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "DeviceIndex"))]
    pub device_index: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "Command"))]
    pub command: String,
}

impl KiirooCmd {
    pub fn new(device_index: u32, command: &str) -> Self {
        Self {
            id: 1,
            device_index,
            command: command.to_owned(),
        }
    }
}

#[derive(Debug, ButtplugMessage, Default, PartialEq, Clone)]
#[cfg_attr(feature = "serialize_json", derive(Serialize, Deserialize))]
pub struct VorzeA10CycloneCmd {
    #[cfg_attr(feature = "serialize_json", serde(rename = "Id"))]
    pub id: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "DeviceIndex"))]
    pub device_index: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "Speed"))]
    pub speed: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "Clockwise"))]
    pub clockwise: bool,
}

impl VorzeA10CycloneCmd {
    pub fn new(device_index: u32, speed: u32, clockwise: bool) -> Self {
        Self {
            id: 1,
            device_index,
            speed,
            clockwise,
        }
    }
}

#[derive(Debug, ButtplugMessage, Default, PartialEq, Clone)]
#[cfg_attr(feature = "serialize_json", derive(Serialize, Deserialize))]
pub struct SingleMotorVibrateCmd {
    #[cfg_attr(feature = "serialize_json", serde(rename = "Id"))]
    pub id: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "DeviceIndex"))]
    pub device_index: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "Speed"))]
    pub speed: f64,
}

impl SingleMotorVibrateCmd {
    pub fn new(device_index: u32, speed: f64) -> Self {
        Self {
            id: 1,
            device_index,
            speed,
        }
    }
}

#[derive(Debug, ButtplugMessage, PartialEq, Clone)]
#[cfg_attr(feature = "serialize_json", derive(Serialize, Deserialize))]
pub struct RawWriteCmd {
    #[cfg_attr(feature = "serialize_json", serde(rename = "Id"))]
    pub id: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "DeviceIndex"))]
    pub device_index: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "Endpoint"))]
    pub endpoint: Endpoint,
    #[cfg_attr(feature = "serialize_json", serde(rename = "Data"))]
    pub data: Vec<u8>,
    #[cfg_attr(feature = "serialize_json", serde(rename = "WriteWithResponse"))]
    pub write_with_response: bool,

}

impl RawWriteCmd {
    pub fn new(device_index: u32, endpoint: Endpoint, data: Vec<u8>, write_with_response: bool) -> Self {
        Self {
            id: 1,
            device_index,
            endpoint,
            data,
            write_with_response
        }
    }
}

#[derive(Debug, ButtplugMessage, PartialEq, Clone)]
#[cfg_attr(feature = "serialize_json", derive(Serialize, Deserialize))]
pub struct RawReadCmd {
    #[cfg_attr(feature = "serialize_json", serde(rename = "Id"))]
    pub id: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "DeviceIndex"))]
    pub device_index: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "Endpoint"))]
    pub endpoint: Endpoint,
    #[cfg_attr(feature = "serialize_json", serde(rename = "ExpectedLength"))]
    pub expected_length: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "WaitForData"))]
    pub wait_for_data: bool,

}

impl RawReadCmd {
    pub fn new(device_index: u32, endpoint: Endpoint, expected_length: u32, wait_for_data: bool) -> Self {
        Self {
            id: 1,
            device_index,
            endpoint,
            expected_length,
            wait_for_data,
        }
    }
}

#[derive(Debug, ButtplugMessage, PartialEq, Clone)]
#[cfg_attr(feature = "serialize_json", derive(Serialize, Deserialize))]
pub struct RawReading {
    #[cfg_attr(feature = "serialize_json", serde(rename = "Id"))]
    pub id: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "DeviceIndex"))]
    pub device_index: u32,
    #[cfg_attr(feature = "serialize_json", serde(rename = "Endpoint"))]
    pub endpoint: Endpoint,
    #[cfg_attr(feature = "serialize_json", serde(rename = "Data"))]
    pub data: Vec<u8>,
}

impl RawReading {
    pub fn new(device_index: u32, endpoint: Endpoint, data: Vec<u8>) -> Self {
        Self {
            id: 1,
            device_index,
            endpoint,
            data
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize_json", derive(Serialize, Deserialize))]
pub enum ButtplugMessageUnion {
    Ok(Ok),
    Error(Error),
    Ping(Ping),
    Test(Test),
    RequestLog(RequestLog),
    Log(Log),
    RequestServerInfo(RequestServerInfo),
    ServerInfo(ServerInfo),
    DeviceList(DeviceList),
    DeviceAdded(DeviceAdded),
    DeviceRemoved(DeviceRemoved),
    StartScanning(StartScanning),
    StopScanning(StopScanning),
    ScanningFinished(ScanningFinished),
    RequestDeviceList(RequestDeviceList),
    VibrateCmd(VibrateCmd),
    LinearCmd(LinearCmd),
    RotateCmd(RotateCmd),
    FleshlightLaunchFW12Cmd(FleshlightLaunchFW12Cmd),
    LovenseCmd(LovenseCmd),
    KiirooCmd(KiirooCmd),
    VorzeA10CycloneCmd(VorzeA10CycloneCmd),
    SingleMotorVibrateCmd(SingleMotorVibrateCmd),
    RawWriteCmd(RawWriteCmd),
    RawReadCmd(RawReadCmd),
    RawReading(RawReading),
    StopDeviceCmd(StopDeviceCmd),
    StopAllDevices(StopAllDevices),
}

impl ButtplugMessage for ButtplugMessageUnion {
    fn get_id(&self) -> u32 {
        match self {
            ButtplugMessageUnion::Ok(ref msg) => msg.id,
            ButtplugMessageUnion::Error(ref msg) => msg.id,
            ButtplugMessageUnion::Log(ref msg) => msg.id,
            ButtplugMessageUnion::RequestLog(ref msg) => msg.id,
            ButtplugMessageUnion::Ping(ref msg) => msg.id,
            ButtplugMessageUnion::Test(ref msg) => msg.id,
            ButtplugMessageUnion::RequestServerInfo(ref msg) => msg.id,
            ButtplugMessageUnion::ServerInfo(ref msg) => msg.id,
            ButtplugMessageUnion::DeviceList(ref msg) => msg.id,
            ButtplugMessageUnion::DeviceAdded(ref msg) => msg.id,
            ButtplugMessageUnion::DeviceRemoved(ref msg) => msg.id,
            ButtplugMessageUnion::StartScanning(ref msg) => msg.id,
            ButtplugMessageUnion::StopScanning(ref msg) => msg.id,
            ButtplugMessageUnion::ScanningFinished(ref msg) => msg.id,
            ButtplugMessageUnion::RequestDeviceList(ref msg) => msg.id,
            ButtplugMessageUnion::VibrateCmd(ref msg) => msg.id,
            ButtplugMessageUnion::LinearCmd(ref msg) => msg.id,
            ButtplugMessageUnion::RotateCmd(ref msg) => msg.id,
            ButtplugMessageUnion::FleshlightLaunchFW12Cmd(ref msg) => msg.id,
            ButtplugMessageUnion::LovenseCmd(ref msg) => msg.id,
            ButtplugMessageUnion::KiirooCmd(ref msg) => msg.id,
            ButtplugMessageUnion::VorzeA10CycloneCmd(ref msg) => msg.id,
            ButtplugMessageUnion::SingleMotorVibrateCmd(ref msg) => msg.id,
            ButtplugMessageUnion::RawWriteCmd(ref msg) => msg.id,
            ButtplugMessageUnion::RawReadCmd(ref msg) => msg.id,
            ButtplugMessageUnion::RawReading(ref msg) => msg.id,
            ButtplugMessageUnion::StopDeviceCmd(ref msg) => msg.id,
            ButtplugMessageUnion::StopAllDevices(ref msg) => msg.id,
        }
    }

    fn set_id(&mut self, id: u32) {
        match self {
            ButtplugMessageUnion::Ok(ref mut msg) => msg.set_id(id),
            ButtplugMessageUnion::Error(ref mut msg) => msg.set_id(id),
            ButtplugMessageUnion::Log(ref mut msg) => msg.set_id(id),
            ButtplugMessageUnion::RequestLog(ref mut msg) => msg.set_id(id),
            ButtplugMessageUnion::Ping(ref mut msg) => msg.set_id(id),
            ButtplugMessageUnion::Test(ref mut msg) => msg.set_id(id),
            ButtplugMessageUnion::RequestServerInfo(ref mut msg) => msg.set_id(id),
            ButtplugMessageUnion::ServerInfo(ref mut msg) => msg.set_id(id),
            ButtplugMessageUnion::DeviceList(ref mut msg) => msg.set_id(id),
            ButtplugMessageUnion::DeviceAdded(ref mut msg) => msg.set_id(id),
            ButtplugMessageUnion::DeviceRemoved(ref mut msg) => msg.set_id(id),
            ButtplugMessageUnion::StartScanning(ref mut msg) => msg.set_id(id),
            ButtplugMessageUnion::StopScanning(ref mut msg) => msg.set_id(id),
            ButtplugMessageUnion::ScanningFinished(ref mut msg) => msg.set_id(id),
            ButtplugMessageUnion::RequestDeviceList(ref mut msg) => msg.set_id(id),
            ButtplugMessageUnion::VibrateCmd(ref mut msg) => msg.set_id(id),
            ButtplugMessageUnion::LinearCmd(ref mut msg) => msg.set_id(id),
            ButtplugMessageUnion::RotateCmd(ref mut msg) => msg.set_id(id),
            ButtplugMessageUnion::FleshlightLaunchFW12Cmd(ref mut msg) => msg.set_id(id),
            ButtplugMessageUnion::LovenseCmd(ref mut msg) => msg.set_id(id),
            ButtplugMessageUnion::KiirooCmd(ref mut msg) => msg.set_id(id),
            ButtplugMessageUnion::VorzeA10CycloneCmd(ref mut msg) => msg.set_id(id),
            ButtplugMessageUnion::SingleMotorVibrateCmd(ref mut msg) => msg.set_id(id),
            ButtplugMessageUnion::RawWriteCmd(ref mut msg) => msg.set_id(id),
            ButtplugMessageUnion::RawReadCmd(ref mut msg) => msg.set_id(id),
            ButtplugMessageUnion::RawReading(ref mut msg) => msg.set_id(id),
            ButtplugMessageUnion::StopDeviceCmd(ref mut msg) => msg.set_id(id),
            ButtplugMessageUnion::StopAllDevices(ref mut msg) => msg.set_id(id),
        }
    }

    fn as_union(self) -> ButtplugMessageUnion {
        panic!("as_union shouldn't be called on union.");
    }
}

#[cfg(feature = "serialize_json")]
#[cfg(test)]
mod test {
    use super::{ButtplugMessageUnion, Error, ErrorCode, Ok, RawReading};
    use crate::devices::Endpoint;

    const OK_STR: &str = "{\"Ok\":{\"Id\":0}}";
    const ERROR_STR: &str =
        "{\"Error\":{\"Id\":0,\"ErrorCode\":1,\"ErrorMessage\":\"Test Error\"}}";

    #[test]
    fn test_ok_serialize() {
        let ok = ButtplugMessageUnion::Ok(Ok::new(0));
        let js = serde_json::to_string(&ok).unwrap();
        assert_eq!(OK_STR, js);
    }

    #[test]
    fn test_ok_deserialize() {
        let union: ButtplugMessageUnion = serde_json::from_str(&OK_STR).unwrap();
        assert_eq!(ButtplugMessageUnion::Ok(Ok::new(0)), union);
    }

    #[test]
    fn test_error_serialize() {
        let error =
            ButtplugMessageUnion::Error(Error::new(ErrorCode::ErrorHandshake, "Test Error"));
        let js = serde_json::to_string(&error).unwrap();
        assert_eq!(ERROR_STR, js);
    }

    #[test]
    fn test_error_deserialize() {
        let union: ButtplugMessageUnion = serde_json::from_str(&ERROR_STR).unwrap();
        assert_eq!(
            ButtplugMessageUnion::Error(Error::new(ErrorCode::ErrorHandshake, "Test Error")),
            union
        );
    }

    #[test]
    fn test_endpoint_deserialize() {
        let endpoint_str = "{\"RawReading\":{\"Id\":1,\"DeviceIndex\":0,\"Endpoint\":\"tx\",\"Data\":[0]}}";
        let union: ButtplugMessageUnion = serde_json::from_str(&endpoint_str).unwrap();
        assert_eq!(ButtplugMessageUnion::RawReading(RawReading::new(0, Endpoint::Tx, vec!(0))), union);
    }

    #[test]
    fn test_endpoint_serialize() {
        let union = ButtplugMessageUnion::RawReading(RawReading::new(0, Endpoint::Tx, vec!(0)));
        let js = serde_json::to_string(&union).unwrap();
        println!("{}", js);
        let endpoint_str = "{\"RawReading\":{\"Id\":1,\"DeviceIndex\":0,\"Endpoint\":\"tx\",\"Data\":[0]}}";
        assert_eq!(js, endpoint_str);
    }
}
