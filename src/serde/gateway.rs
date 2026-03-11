#[cfg(not(debug_assertions))]
use log::error;

use anyhow::Result;
use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize, de};
use serde_json::Value;

use crate::gateway::dispatch::dispatch_deserialize;
use crate::serde::types::gateway::{OP10D, ReceiveData, ReceiveDataType, SendData, SendDataType};

// TODO: make impl Serialize implementation consistent across all places
impl Serialize for SendData {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut serializer_map = serializer.serialize_map(None)?;

        serializer_map.serialize_entry("op", &self.op)?;

        match &self.d {
            SendDataType::OP1(op1_d) => serializer_map.serialize_entry("d", &op1_d)?,
            SendDataType::OP2(op2_d) => serializer_map.serialize_entry("d", &op2_d)?,
        }

        serializer_map.end()
    }
}

impl<'de> Deserialize<'de> for ReceiveData {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(deserializer)?;
        let op = value["op"]
            .as_u64()
            .ok_or_else(|| de::Error::missing_field("op"))? as u8;

        let d = match op {
            0 => ReceiveDataType::OP0(Box::new(
                dispatch_deserialize(&value).map_err(de::Error::custom)?,
            )),
            1 => {
                let inner: Option<u32> =
                    Option::deserialize(&value["d"]).map_err(de::Error::custom)?;
                ReceiveDataType::OP1(inner)
            }
            9 => {
                let inner = bool::deserialize(&value["d"]).map_err(de::Error::custom)?;
                ReceiveDataType::OP9(inner)
            }
            10 => {
                let inner = OP10D::deserialize(&value["d"]).map_err(de::Error::custom)?;
                ReceiveDataType::OP10(inner)
            }
            11 => ReceiveDataType::OP11,
            _ => return Err(de::Error::custom(format!("unknown op: {}", op))),
        };

        Ok(ReceiveData { d, op })
    }
}
