use super::{config::DragonModuleConfigurable, errors::ModuleError};
use crate::core::{module::DragonBotModule, permissions::ModulePermission};
use serenity::all::{GenericId, GuildId, Member};
use std::collections::HashMap;

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
    async fn has_permission_str(
        &self,
        member: &Member,
        namespace: &str,
        permission: &str,
    ) -> Result<bool, ModuleError> {
        let mut guild_config = self
            .get_config::<PermissionsManager>(member.guild_id)
            .await?;
        for id in [member.user.id.get()]
            .into_iter()
            .chain(member.roles.iter().map(|role| role.get()))
        {
            if guild_config
                .namespaces
                .entry(namespace.to_string())
                .or_default()
                .get(&GenericId::new(id))
                .is_some_and(|granted| granted.contains(&permission.to_string()))
            {
                return Ok(true);
            }
        }

        Ok(false)
    }

    async fn give_permission_str(
        &self,
        guild: GuildId,
        target: GenericId,
        namespace: &str,
        permission: &str,
    ) -> Result<(), ModuleError> {
        let permission = permission.to_string();
        let mut guild_config = self.get_config::<PermissionsManager>(guild).await?;

        let namespaces = &mut guild_config.namespaces;
        let namespace = namespaces
            .entry(namespace.to_string())
            .or_insert_with(HashMap::new);

        let permissions = namespace.entry(target).or_insert_with(Vec::new);
        if permissions.contains(&permission) {
            Err(PermissionsError::PermissionAlreadyGiven)?;
            unreachable!()
        }
        permissions.push(permission);

        self.set_config::<PermissionsManager>(guild, guild_config)
            .await
    }

    async fn take_permission_str(
        &self,
        guild: GuildId,
        target: GenericId,
        namespace: &str,
        permission: &str,
    ) -> Result<(), ModuleError> {
        let permission = permission.to_string();
        let mut guild_config = self.get_config::<PermissionsManager>(guild).await?;

        let namespaces = &mut guild_config.namespaces;
        let namespace = namespaces
            .entry(namespace.to_string())
            .or_insert_with(HashMap::new);

        let permissions = namespace.entry(target).or_insert_with(Vec::new);
        if permissions.contains(&permission) {
            Err(PermissionsError::PermissionNotGiven)?;
            unreachable!()
        }
        permissions.retain(|perm| *perm != permission);

        self.set_config::<PermissionsManager>(guild, guild_config)
            .await
    }

    pub async fn has_permission(
        &self,
        member: &Member,
        permission: ModulePermission,
    ) -> Result<bool, ModuleError> {
        self.has_permission_str(member, permission.module(), permission.id())
            .await
    }

    pub async fn give_permission(
        &self,
        guild: GuildId,
        target: GenericId,
        permission: ModulePermission,
    ) -> Result<(), ModuleError> {
        self.give_permission_str(guild, target, permission.module(), permission.id())
            .await
    }

    pub async fn take_permission(
        &self,
        guild: GuildId,
        target: GenericId,
        permission: ModulePermission,
    ) -> Result<(), ModuleError> {
        self.take_permission_str(guild, target, permission.module(), permission.id())
            .await
    }
}
