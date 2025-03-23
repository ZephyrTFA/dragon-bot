use super::{config::Configurable, errors::ModuleError};
use crate::core::{
    module::DragonBotModule,
    permissions::{DragonModulePermissions, ModulePermission},
};
use serenity::all::{Guild, Member, RoleId};

mod command;
mod config;
mod permission;

#[derive(Default)]
pub struct PermissionsManager;

impl DragonBotModule for PermissionsManager {
    fn module_id() -> &'static str
    where
        Self: Sized,
    {
        "permissions-manager"
    }
}

#[derive(Debug)]
pub enum PermissionsError {
    PermissionNotFound,
    PermissionAlreadyGiven,
    PermissionNotGiven,
}

impl PermissionsManager {
    pub async fn has_permission(
        &self,
        module: &impl DragonModulePermissions,
        member: &Member,
        permission: ModulePermission,
    ) -> Result<bool, ModuleError> {
        if !module.all_permissions().await.contains(&permission) {
            Err(PermissionsError::PermissionNotFound)?;
            unreachable!();
        }

        let guild_config = self.get_config(member.guild_id).await?;

        let user_permissions = &guild_config.user;
        if user_permissions
            .get(&member.user.id)
            .is_some_and(|permissions| permissions.iter().any(|perm| perm == permission.id()))
        {
            return Ok(true);
        }

        let role_permissions = &guild_config.role;
        for role in &member.roles {
            if role_permissions
                .get(role)
                .is_some_and(|permissions| permissions.iter().any(|perm| perm == permission.id()))
            {
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub async fn give_permission_user(
        &self,
        member: &Member,
        permission: &str,
    ) -> Result<(), ModuleError> {
        let mut guild_config = self.get_config(member.guild_id).await?;

        let permissions = &mut guild_config.user;
        permissions
            .entry(member.user.id)
            .or_insert_with(std::vec::Vec::new);

        let user_permissions = permissions.get_mut(&member.user.id).unwrap();
        if user_permissions.contains(&permission.to_string()) {
            return Err(PermissionsError::PermissionAlreadyGiven.into());
        }
        user_permissions.push(permission.to_string());

        self.set_config(member.guild_id, guild_config).await
    }

    pub async fn take_permission_user(
        &self,
        member: &Member,
        permission: &str,
    ) -> Result<(), ModuleError> {
        let mut guild_config = self.get_config(member.guild_id).await?;
        let permissions = &mut guild_config.user;
        if !permissions.contains_key(&member.user.id) {
            return Err(PermissionsError::PermissionNotGiven.into());
        }
        let user_permissions = permissions.get_mut(&member.user.id).unwrap();
        user_permissions.retain(|user_permission| user_permission != permission);

        self.set_config(member.guild_id, guild_config).await
    }

    pub async fn give_permission_role(
        &self,
        guild: &Guild,
        role: RoleId,
        permission: &str,
    ) -> Result<(), ModuleError> {
        let mut guild_config = self.get_config(guild.id).await?;
        let permissions = &mut guild_config.role;
        permissions.entry(role).or_insert_with(std::vec::Vec::new);
        let permissions = permissions.get_mut(&role).unwrap();
        permissions.push(permission.to_string());

        self.set_config(guild.id, guild_config).await
    }

    pub async fn take_permission_role(
        &self,
        guild: &Guild,
        role: RoleId,
        permission: &str,
    ) -> Result<(), ModuleError> {
        let mut guild_config = self.get_config(guild.id).await?;
        let permissions = &mut guild_config.role;
        if !permissions.contains_key(&role) {
            return Ok(());
        }
        let permissions = permissions.get_mut(&role).unwrap();
        permissions.retain(|user_permission| user_permission != permission);

        self.set_config(guild.id, guild_config).await
    }
}
