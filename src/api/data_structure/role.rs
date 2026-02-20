use derive_builder::Builder;

#[derive(Clone, Builder)]
#[builder(try_setter, setter(into))]
pub struct AddRoleToMember {
    pub guild_id: String,
    pub user_id: String,
    pub role_id: String,
}

#[derive(Clone, Builder)]
#[builder(try_setter, setter(into))]
pub struct RemoveRoleFromMember {
    pub guild_id: String,
    pub user_id: String,
    pub role_id: String,
}

#[derive(Clone, Builder)]
#[builder(try_setter, setter(into))]
pub struct CreateRole {
    pub guild_id: String,

    pub name: String,
    pub color: u32,
    pub permission: String,
}

#[derive(Clone, Builder)]
#[builder(try_setter, setter(into))]
pub struct DeleteRole {
    pub guild_id: String,
    pub role_id: String,
}
