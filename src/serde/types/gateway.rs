use serde::{Deserialize, Serialize};

use crate::gateway::dispatch::DispatchEvent;

pub struct ReceiveData {
    pub d: ReceiveDataType,
    pub op: u8,
}

pub struct SendData {
    pub d: SendDataType,
    pub op: u8,
}

pub enum ReceiveDataType {
    OP0(Box<DispatchEvent>),
    OP1(Option<u32>),
    OP9(bool),
    OP10(OP10D),
    OP11,
}

pub enum SendDataType {
    OP1(Option<u32>),
    OP2(OP2D),
}

#[derive(Serialize)]
pub struct OP2D {
    pub token: String,
    pub properties: OP2DProps,
}

#[derive(Serialize)]
pub struct OP2DProps {
    pub os: String,
    pub browser: String,
    pub device: String,
}

#[derive(Serialize, Deserialize)]
pub struct OP10D {
    pub heartbeat_interval: u32,
}
