use serde::Deserialize;

#[derive(Deserialize)]
pub struct SessionReplaceData{
    pub afk: bool,
    pub mobile: bool,
    pub session_id: String,
    pub status: String,
}