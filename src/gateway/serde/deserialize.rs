#[cfg(not(debug_assertions))]
use log::{error, info};
use serde::{Deserialize, Serialize, de};
use serde_json::Value;

use crate::gateway::data_structure::common::{ChannelData, GuildPropertiesData, MemberData};
use crate::gateway::data_structure::guild::{
    GuildEmojisUpdateData, GuildRoleCreateData, GuildRoleDeleteData, GuildRoleUpdateBulkData,
};
use crate::gateway::data_structure::message::MessageReactData;
use crate::gateway::data_structure::user::ReadyData;
use crate::gateway::data_structure::user::SessionReplaceData;
use crate::gateway::{
    data_structure::{
        guild::{GuildCreateData, GuildDeleteData},
        message::{MessageEventData, TypingEventData},
    },
    op_handlers::dispatch::DispatchEvent,
};

pub struct ReceiveData {
    pub d: ReceiveDataType,
    pub op: u8,
}

pub enum ReceiveDataType {
    OP0(Box<DispatchEvent>),
    OP1(Option<u32>),
    OP9(bool),
    OP10(OP10D),
    OP11,
}

#[derive(Serialize, Deserialize)]
pub struct OP10D {
    pub heartbeat_interval: u32,
}

impl<'de> Deserialize<'de> for ReceiveData {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(deserializer)?;
        let op = value["op"]
            .as_u64()
            .ok_or_else(|| de::Error::missing_field("op"))? as u8;

        let d = match op {
            0 => {
                let dispatch_event = match value["t"].as_str().unwrap() {
                    "READY" => DispatchEvent::Ready(Box::new(
                        ReadyData::deserialize(&value["d"]).map_err(de::Error::custom)?,
                    )),
                    "GUILD_DELETE" => DispatchEvent::GuildDelete(
                        GuildDeleteData::deserialize(&value["d"]).map_err(de::Error::custom)?,
                    ),
                    "GUILD_CREATE" => DispatchEvent::GuildCreate(Box::new(
                        GuildCreateData::deserialize(&value["d"]).map_err(de::Error::custom)?,
                    )),
                    "GUILD_UPDATE" => DispatchEvent::GuildUpdate(
                        GuildPropertiesData::deserialize(&value["d"]).map_err(de::Error::custom)?,
                    ),
                    "GUILD_EMOJIS_UPDATE" => DispatchEvent::GuildEmojisUpdate(
                        GuildEmojisUpdateData::deserialize(&value["d"])
                            .map_err(de::Error::custom)?,
                    ),
                    "GUILD_ROLE_CREATE" => DispatchEvent::GuildRoleCreate(
                        GuildRoleCreateData::deserialize(&value["d"]).map_err(de::Error::custom)?,
                    ),
                    "GUILD_ROLE_UPDATE" => DispatchEvent::GuildRoleUpdate(
                        GuildRoleCreateData::deserialize(&value["d"]).map_err(de::Error::custom)?,
                    ),
                    "GUILD_ROLE_UPDATE_BULK" => DispatchEvent::GuildRoleUpdateBulk(
                        GuildRoleUpdateBulkData::deserialize(&value["d"])
                            .map_err(de::Error::custom)?,
                    ),
                    "GUILD_ROLE_DELETE" => DispatchEvent::GuildRoleDelete(
                        GuildRoleDeleteData::deserialize(&value["d"]).map_err(de::Error::custom)?,
                    ),
                    "GUILD_MEMBER_UPDATE" => DispatchEvent::GuildMemberUpdate(
                        MemberData::deserialize(&value["d"]).map_err(de::Error::custom)?,
                    ),
                    "MESSAGE_CREATE" => DispatchEvent::MessageCreate(
                        MessageEventData::deserialize(&value["d"]).map_err(de::Error::custom)?,
                    ),
                    "MESSAGE_DELETE" => DispatchEvent::MessageDelete(
                        MessageEventData::deserialize(&value["d"]).map_err(de::Error::custom)?,
                    ),
                    "MESSAGE_UPDATE" => DispatchEvent::MessageUpdate(
                        MessageEventData::deserialize(&value["d"]).map_err(de::Error::custom)?,
                    ),
                    "TYPING_START" => DispatchEvent::TypingStart(
                        TypingEventData::deserialize(&value["d"]).map_err(de::Error::custom)?,
                    ),
                    "TYPING_STOP" => DispatchEvent::TypingStop(
                        TypingEventData::deserialize(&value["d"]).map_err(de::Error::custom)?,
                    ),
                    "MESSAGE_REACTION_ADD" => DispatchEvent::MessageReactionAdd(
                        MessageReactData::deserialize(&value["d"]).map_err(de::Error::custom)?,
                    ),
                    "MESSAGE_REACTION_REMOVE" => DispatchEvent::MessageReactionRemove(
                        MessageReactData::deserialize(&value["d"]).map_err(de::Error::custom)?,
                    ),
                    "MESSAGE_REACTION_REMOVE_EMOJI" => DispatchEvent::MessageReactionRemoveEmoji(
                        MessageReactData::deserialize(&value["d"]).map_err(de::Error::custom)?,
                    ),
                    "SESSIONS_REPLACE" => DispatchEvent::SessionReplace(
                        Vec::<SessionReplaceData>::deserialize(&value["d"])
                            .map_err(de::Error::custom)?,
                    ),
                    "CHANNEL_UPDATE" => DispatchEvent::ChannelUpdate(
                        ChannelData::deserialize(&value["d"]).map_err(de::Error::custom)?,
                    ),
                    #[cfg(not(debug_assertions))]
                    _ => error!("Unimplemented dispatch event: {}", value),

                    #[cfg(debug_assertions)]
                    _ => panic!("Unimplemented dispatch event: {}", value),
                };
                ReceiveDataType::OP0(Box::new(dispatch_event))
            }
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
