use derive_builder::Builder;

#[derive(Clone, Builder)]
#[builder(try_setter, setter(into))]
pub struct AddOwnReaciton {
    pub channel_id: String,
    pub message_id: String,
    pub emoji: String,
}

#[derive(Clone, Builder)]
#[builder(try_setter, setter(into))]
pub struct RemoveAllReaction {
    pub channel_id: String,
    pub message_id: String,
    pub emoji: String,
}
