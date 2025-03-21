use super::{
    config::Configurable,
    errors::ModuleError,
    tgdb::{TgDb, TgDbError},
};
use crate::{
    core::{
        commands::DragonModuleCommand, module::DragonBotModule,
        permissions::DragonModulePermissions,
    },
    get_module,
};
use discord_link::ByondDiscordLink;
use mysql::{params, prelude::Queryable};
use serenity::all::GuildId;
use std::ops::Deref;

mod config;
mod discord_link;

#[derive(Default)]
pub struct TgVerify;

impl DragonBotModule for TgVerify {
    fn module_id() -> &'static str
    where
        Self: Sized,
    {
        "tg-verify"
    }
}

impl DragonModulePermissions for TgVerify {}
impl DragonModuleCommand for TgVerify {}

impl TgVerify {
    pub async fn query_ckey(
        &self,
        guild: GuildId,
        ckey: &str,
    ) -> Result<Vec<ByondDiscordLink>, ModuleError> {
        let config = self.get_config(guild).await?;
        let discord_link_table = &config.discord_links_table;

        get_module!(tgdb, instance, TgDb);
        Ok(tgdb
            .get_conn(guild)?
            .exec(
                "SELECT * FROM :discord_link_table WHERE ckey = :ckey",
                params! {
                    discord_link_table,
                    ckey,
                },
            )
            .map_err(TgDbError::MysqlError)?)
    }

    pub async fn query_discord_id(
        &self,
        guild: GuildId,
        discord_id: u64,
    ) -> Result<Vec<ByondDiscordLink>, ModuleError> {
        let config = self.get_config(guild).await?;
        let discord_link_table = &config.discord_links_table;

        get_module!(tgdb, instance, TgDb);
        Ok(tgdb
            .get_conn(guild)?
            .exec(
                "SELECT * FROM :discord_link_table WHERE discord_id = :discord_id",
                params! {
                    discord_link_table,
                    discord_id,
                },
            )
            .map_err(TgDbError::MysqlError)?)
    }

    pub async fn query_link_token(
        &self,
        guild: GuildId,
        token: &str,
    ) -> Result<Option<ByondDiscordLink>, ModuleError> {
        let config = self.get_config(guild).await?;
        let discord_link_table = &config.discord_links_table;

        get_module!(tgdb, instance, TgDb);
        Ok(tgdb
            .get_conn(guild)?
            .exec_first(
                "SELECT * FROM :discord_link_table WHERE one_time_token = :token",
                params! {
                    discord_link_table,
                    token,
                },
            )
            .map_err(TgDbError::MysqlError)?)
    }

    pub async fn update_link(
        &self,
        guild: GuildId,
        link: &ByondDiscordLink,
    ) -> Result<(), ModuleError> {
        let config = self.get_config(guild).await?;
        let discord_link_table = &config.discord_links_table;
        let id = &link.id;

        get_module!(tgdb, instance, TgDb);
        let result: Option<ByondDiscordLink> = tgdb
            .get_conn(guild)?
            .exec_first(
                "UPDATE :discord_link_table WHERE id = :id",
                params! {
                    discord_link_table,
                    id,
                },
            )
            .map_err(TgDbError::MysqlError)?;

        if result.is_none() {
            return Err(ModuleError::TgDbError(TgDbError::InternalError(
                "failed to update discord link entry".to_string(),
            )));
        }

        Ok(())
    }
}
