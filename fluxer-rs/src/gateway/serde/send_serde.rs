use serde::{Serialize, ser::SerializeMap};

pub enum SendDataType {
    OP1(Option<u32>),
    OP2(OP2D),
}

pub struct SendData {
    pub d: SendDataType,
    pub op: u8,
}

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
