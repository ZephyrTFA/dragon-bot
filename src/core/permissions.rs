use crate::{
    core::module::get_module,
    module::{errors::ModuleError, permissions::PermissionsManager},
};
use log::warn;
use serenity::all::{
    CacheHttp, CommandInteraction, Context, CreateInteractionResponseFollowup, Member,
};

pub struct ModulePermission(&'static str, &'static str, &'static str);
impl ModulePermission {
    pub const fn new(module: &'static str, id: &'static str, desc: &'static str) -> Self {
        Self(module, id, desc)
    }
    pub fn module(&self) -> &str {
        self.0
    }
    pub fn id(&self) -> &str {
        self.1
    }
    pub fn desc(&self) -> &str {
        self.2
    }
}

impl PartialEq for ModulePermission {
    fn eq(&self, other: &Self) -> bool {
        (self.id() == other.id()) && (self.module() == other.module())
    }
}

pub trait DragonModulePermission {
    fn all_permissions(&self) -> impl Future<Output = Vec<ModulePermission>> {
        async { vec![] }
    }
}

pub async fn assert_permission(
    ctx: &Context,
    command: &CommandInteraction,
    member: &Member,
    permission: ModulePermission,
) -> Result<bool, ModuleError> {
    if member.permissions.is_some_and(|perm| perm.administrator()) {
        return Ok(true);
    }

    let permissions = get_module::<PermissionsManager>()?;
    if !permissions
        .module::<PermissionsManager>()
        .has_permission(member, permission)
        .await?
    {
        if let Err(error) = command
            .create_followup(
                ctx.http(),
                CreateInteractionResponseFollowup::new()
                    .ephemeral(true)
                    .content("You do not have permission to use this command."),
            )
            .await
        {
            warn!("failed to send permission assertion error response: {error}");
        }
        return Ok(false);
    }
    Ok(true)
}
