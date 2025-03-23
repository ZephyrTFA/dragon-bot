use super::module::{DragonBotModule, GetModule};
use crate::{
    core::module::get_module,
    module::{errors::ModuleError, permissions::PermissionsManager},
};
use log::warn;
use serenity::all::{
    CacheHttp, CommandInteraction, Context, CreateInteractionResponseFollowup, Member,
};

pub trait DragonModulePermissions {
    fn all_permissions(&self) -> &'static [&'static str] {
        &[]
    }
}

pub async fn assert_permission(
    ctx: &Context,
    command: &CommandInteraction,
    module: &impl DragonBotModule,
    member: &Member,
    permission: &str,
) -> Result<bool, ModuleError> {
    let permissions = get_module::<PermissionsManager>().await;
    let permissions: &PermissionsManager = permissions.module();

    if !permissions
        .has_permission(module, member, permission)
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
