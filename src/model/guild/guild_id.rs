use std::fmt;

#[cfg(feature = "model")]
use futures::stream::Stream;

#[cfg(feature = "model")]
use crate::builder::{
    AddMember,
    CreateApplicationCommand,
    CreateApplicationCommandPermissionsData,
    CreateApplicationCommands,
    CreateChannel,
    CreateScheduledEvent,
    CreateSticker,
    EditAutoModRule,
    EditGuild,
    EditGuildWelcomeScreen,
    EditGuildWidget,
    EditMember,
    EditRole,
    EditScheduledEvent,
    EditSticker,
};
#[cfg(all(feature = "cache", feature = "model"))]
use crate::cache::Cache;
#[cfg(feature = "collector")]
use crate::client::bridge::gateway::ShardMessenger;
#[cfg(feature = "collector")]
use crate::collector::{
    CollectReaction,
    CollectReply,
    MessageCollectorBuilder,
    ReactionCollectorBuilder,
};
#[cfg(feature = "model")]
use crate::http::{CacheHttp, Http, UserPagination};
#[cfg(feature = "model")]
use crate::internal::prelude::*;
#[cfg(feature = "model")]
use crate::json;
#[cfg(feature = "model")]
use crate::json::json;
#[cfg(feature = "model")]
use crate::json::prelude::*;
#[cfg(feature = "model")]
use crate::model::application::command::{Command, CommandPermission};
#[cfg(feature = "model")]
use crate::model::guild::automod::Rule;
use crate::model::prelude::*;

#[cfg(feature = "model")]
impl GuildId {
    /// Gets all auto moderation [`Rule`]s of this guild via HTTP.
    ///
    /// **Note**: Requires the [Manage Guild] permission.
    ///
    /// # Errors
    ///
    /// Returns an [`Error::Http`] if the guild is unavailable.
    ///
    /// [Manage Guild]: Permissions::MANAGE_GUILD
    #[inline]
    pub async fn automod_rules(self, http: impl AsRef<Http>) -> Result<Vec<Rule>> {
        http.as_ref().get_automod_rules(self.0).await
    }

    /// Gets an auto moderation [`Rule`] of this guild by its ID via HTTP.
    ///
    /// **Note**: Requires the [Manage Guild] permission.
    ///
    /// # Errors
    ///
    /// Returns an [`Error::Http`] if a rule with the given ID does not exist.
    ///
    /// [Manage Guild]: Permissions::MANAGE_GUILD
    #[inline]
    pub async fn automod_rule(
        self,
        http: impl AsRef<Http>,
        rule_id: impl Into<RuleId>,
    ) -> Result<Rule> {
        http.as_ref().get_automod_rule(self.0, rule_id.into().0).await
    }

    /// Creates an auto moderation [`Rule`] in the guild.
    ///
    /// **Note**: Requires the [Manage Guild] permission.
    ///
    /// # Examples
    ///
    /// Create a custom keyword filter to block the message and timeout the author.
    ///
    /// ```
    /// use std::time::Duration;
    ///
    /// use serenity::model::guild::automod::{Action, Trigger};
    /// use serenity::model::id::GuildId;
    ///
    /// # async fn run() {
    /// # use serenity::http::Http;
    /// # let http = Http::new("token");
    /// let _rule = GuildId(7)
    ///     .create_automod_rule(&http, |r| {
    ///         r.name("foobar filter")
    ///             .trigger(Trigger::Keyword(vec!["foo*".to_string(), "*bar".to_string()]))
    ///             .actions(vec![Action::BlockMessage, Action::Timeout(Duration::from_secs(60))])
    ///     })
    ///     .await;
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user lacks permission,
    /// or if invalid values are set.
    ///
    /// [Manage Guild]: Permissions::MANAGE_GUILD
    #[inline]
    pub async fn create_automod_rule(
        self,
        http: impl AsRef<Http>,
        f: impl FnOnce(&mut EditAutoModRule) -> &mut EditAutoModRule,
    ) -> Result<Rule> {
        let mut builder = EditAutoModRule::default();
        f(&mut builder);

        let map = json::hashmap_to_json_map(builder.0);

        http.as_ref().create_automod_rule(self.0, &map).await
    }

    /// Edit an auto moderation [`Rule`] by its ID.
    ///
    /// **Note**: Requires the [Manage Guild] permission.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user lacks permission,
    /// or if invalid values are set.
    ///
    /// [Manage Guild]: Permissions::MANAGE_GUILD
    #[inline]
    pub async fn edit_automod_rule(
        self,
        http: impl AsRef<Http>,
        rule_id: impl Into<RuleId>,
        f: impl FnOnce(&mut EditAutoModRule) -> &mut EditAutoModRule,
    ) -> Result<Rule> {
        let mut builder = EditAutoModRule::default();
        f(&mut builder);

        let map = json::hashmap_to_json_map(builder.0);

        http.as_ref().edit_automod_rule(self.0, rule_id.into().0, &map).await
    }

    /// Deletes an auto moderation [`Rule`] from the guild.
    ///
    /// **Note**: Requires the [Manage Guild] permission.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user lacks permission,
    /// or if a rule with that Id does not exist.
    ///
    /// [Manage Guild]: Permissions::MANAGE_GUILD
    #[inline]
    pub async fn delete_automod_rule(
        self,
        http: impl AsRef<Http>,
        rule_id: impl Into<RuleId>,
    ) -> Result<()> {
        http.as_ref().delete_automod_rule(self.0, rule_id.into().0).await
    }

    /// Adds a [`User`] to this guild with a valid OAuth2 access token.
    ///
    /// Returns the created [`Member`] object, or nothing if the user is already a member of the guild.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user lacks permission,
    /// or if invalid values are set.
    #[inline]
    pub async fn add_member(
        self,
        http: impl AsRef<Http>,
        user_id: impl Into<UserId>,
        f: impl FnOnce(&mut AddMember) -> &mut AddMember,
    ) -> Result<Option<Member>> {
        let mut builder = AddMember::default();
        f(&mut builder);

        let map = json::hashmap_to_json_map(builder.0);

        http.as_ref().add_guild_member(self.0, user_id.into().0, &map).await
    }

    /// Ban a [`User`] from the guild, deleting a number of
    /// days' worth of messages (`dmd`) between the range 0 and 7.
    ///
    /// Refer to the documentation for [`Guild::ban`] for more information.
    ///
    /// **Note**: Requires the [Ban Members] permission.
    ///
    /// # Examples
    ///
    /// Ban a member and remove all messages they've sent in the last 4 days:
    ///
    /// ```rust,no_run
    /// use serenity::model::id::{GuildId, UserId};
    ///
    /// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # use serenity::http::Http;
    /// # let http = Http::new("token");
    /// # let user = UserId(1);
    /// // assuming a `user` has already been bound
    /// let _ = GuildId(81384788765712384).ban(&http, user, 4).await;
    /// #    Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a [`ModelError::DeleteMessageDaysAmount`] if the number of
    /// days' worth of messages to delete is over the maximum.
    ///
    /// Also can return [`Error::Http`] if the current user lacks permission.
    ///
    /// [Ban Members]: Permissions::BAN_MEMBERS
    #[inline]
    pub async fn ban(self, http: impl AsRef<Http>, user: impl Into<UserId>, dmd: u8) -> Result<()> {
        self._ban_with_reason(http, user.into(), dmd, "").await
    }

    /// Ban a [`User`] from the guild with a reason. Refer to [`Self::ban`] to further documentation.
    ///
    /// # Errors
    ///
    /// In addition to the reasons [`Self::ban`] may return an error, may
    /// also return [`Error::ExceededLimit`] if `reason` is too long.
    #[inline]
    pub async fn ban_with_reason(
        self,
        http: impl AsRef<Http>,
        user: impl Into<UserId>,
        dmd: u8,
        reason: impl AsRef<str>,
    ) -> Result<()> {
        self._ban_with_reason(http, user.into(), dmd, reason.as_ref()).await
    }

    async fn _ban_with_reason(
        self,
        http: impl AsRef<Http>,
        user: UserId,
        dmd: u8,
        reason: &str,
    ) -> Result<()> {
        if dmd > 7 {
            return Err(Error::Model(ModelError::DeleteMessageDaysAmount(dmd)));
        }

        if reason.chars().count() > 512 {
            return Err(Error::ExceededLimit(reason.to_string(), 512));
        }

        http.as_ref().ban_user(self.0, user.0, dmd, reason).await
    }

    /// Gets a list of the guild's bans.
    ///
    /// **Note**: Requires the [Ban Members] permission.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user lacks permission.
    ///
    /// [Ban Members]: Permissions::BAN_MEMBERS
    #[inline]
    pub async fn bans(self, http: impl AsRef<Http>) -> Result<Vec<Ban>> {
        http.as_ref().get_bans(self.0).await
    }

    /// Gets a list of the guild's audit log entries
    ///
    /// **Note**: Requires the [View Audit Log] permission.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user lacks permission,
    /// or if an invalid value is given.
    ///
    /// [View Audit Log]: Permissions::VIEW_AUDIT_LOG
    #[inline]
    pub async fn audit_logs(
        self,
        http: impl AsRef<Http>,
        action_type: Option<u8>,
        user_id: Option<UserId>,
        before: Option<AuditLogEntryId>,
        limit: Option<u8>,
    ) -> Result<AuditLogs> {
        http.as_ref()
            .get_audit_logs(self.0, action_type, user_id.map(|u| u.0), before.map(|a| a.0), limit)
            .await
    }

    /// Gets all of the guild's channels over the REST API.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user is not in
    /// the guild.
    pub async fn channels(
        self,
        http: impl AsRef<Http>,
    ) -> Result<HashMap<ChannelId, GuildChannel>> {
        let mut channels = HashMap::new();

        // Clippy is suggesting:
        // consider removing
        // `http.as_ref().get_channels(self.0)?()`:
        // `http.as_ref().get_channels(self.0)?`.
        #[allow(clippy::useless_conversion)]
        for channel in http.as_ref().get_channels(self.0).await? {
            channels.insert(channel.id, channel);
        }

        Ok(channels)
    }

    /// Creates a [`GuildChannel`] in the the guild.
    ///
    /// Refer to [`Http::create_channel`] for more information.
    ///
    /// Requires the [Manage Channels] permission.
    ///
    /// # Examples
    ///
    /// Create a voice channel in a guild with the name `test`:
    ///
    /// ```rust,no_run
    /// use serenity::model::channel::ChannelType;
    /// use serenity::model::id::GuildId;
    ///
    /// # async fn run() {
    /// # use serenity::http::Http;
    /// # let http = Http::new("token");
    /// let _channel =
    ///     GuildId(7).create_channel(&http, |c| c.name("test").kind(ChannelType::Voice)).await;
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user lacks permission,
    /// or if invalid values are set.
    ///
    /// [Manage Channels]: Permissions::MANAGE_CHANNELS
    #[inline]
    pub async fn create_channel(
        self,
        http: impl AsRef<Http>,
        f: impl FnOnce(&mut CreateChannel) -> &mut CreateChannel,
    ) -> Result<GuildChannel> {
        let mut builder = CreateChannel::default();
        f(&mut builder);

        let map = json::hashmap_to_json_map(builder.0);

        http.as_ref().create_channel(self.0, &map, None).await
    }

    /// Creates an emoji in the guild with a name and base64-encoded image.
    ///
    /// Refer to the documentation for [`Guild::create_emoji`] for more
    /// information.
    ///
    /// Requires the [Manage Emojis and Stickers] permission.
    ///
    /// # Examples
    ///
    /// See the [`EditProfile::avatar`] example for an in-depth example as to
    /// how to read an image from the filesystem and encode it as base64. Most
    /// of the example can be applied similarly for this method.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user lacks permission,
    /// if the name is too long, or if the image is too big.
    ///
    /// [`EditProfile::avatar`]: crate::builder::EditProfile::avatar
    /// [Manage Emojis and Stickers]: Permissions::MANAGE_EMOJIS_AND_STICKERS
    #[inline]
    pub async fn create_emoji(
        self,
        http: impl AsRef<Http>,
        name: &str,
        image: &str,
    ) -> Result<Emoji> {
        let map = json!({
            "name": name,
            "image": image,
        });

        http.as_ref().create_emoji(self.0, &map, None).await
    }

    /// Creates an integration for the guild.
    ///
    /// Requires the [Manage Guild] permission.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user lacks permission.
    ///
    /// [Manage Guild]: Permissions::MANAGE_GUILD
    #[inline]
    pub async fn create_integration(
        self,
        http: impl AsRef<Http>,
        integration_id: impl Into<IntegrationId>,
        kind: &str,
    ) -> Result<()> {
        let integration_id = integration_id.into();
        let map = json!({
            "id": integration_id.0,
            "type": kind,
        });

        http.as_ref().create_guild_integration(self.0, integration_id.0, &map, None).await
    }

    /// Creates a new role in the guild with the data set, if any.
    ///
    /// See the documentation for [`Guild::create_role`] on how to use this.
    ///
    /// **Note**: Requires the [Manage Roles] permission.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user lacks permission,
    /// or if invalid data is given.
    ///
    /// [Manage Roles]: Permissions::MANAGE_ROLES
    #[inline]
    pub async fn create_role<F>(self, http: impl AsRef<Http>, f: F) -> Result<Role>
    where
        F: FnOnce(&mut EditRole) -> &mut EditRole,
    {
        let mut edit_role = EditRole::default();
        f(&mut edit_role);
        let map = json::hashmap_to_json_map(edit_role.0);

        let role = http.as_ref().create_role(self.0, &map, None).await?;

        if let Some(position) = map.get("position").and_then(Value::as_u64) {
            self.edit_role_position(&http, role.id, position).await?;
        }

        Ok(role)
    }

    /// Creates a new scheduled event in the guild with the data set, if any.
    ///
    /// **Note**: Requires the [Manage Events] permission.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user lacks permission, or if invalid data is given.
    ///
    /// [Manage Events]: Permissions::MANAGE_EVENTS
    pub async fn create_scheduled_event<F>(
        &self,
        http: impl AsRef<Http>,
        f: F,
    ) -> Result<ScheduledEvent>
    where
        F: FnOnce(&mut CreateScheduledEvent) -> &mut CreateScheduledEvent,
    {
        let mut builder = CreateScheduledEvent::default();
        f(&mut builder);

        let map = json::hashmap_to_json_map(builder.0);

        http.as_ref().create_scheduled_event(self.0, &map, None).await
    }

    /// Creates a new sticker in the guild with the data set, if any.
    ///
    /// **Note**: Requires the [Manage Emojis and Stickers] permission.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user lacks permission,
    /// or if invalid data is given.
    ///
    /// [Manage Emojis and Stickers]: crate::model::permissions::Permissions::MANAGE_EMOJIS_AND_STICKERS
    #[inline]
    pub async fn create_sticker<'a, F>(self, http: impl AsRef<Http>, f: F) -> Result<Sticker>
    where
        for<'b> F: FnOnce(&'b mut CreateSticker<'a>) -> &'b mut CreateSticker<'a>,
    {
        let mut create_sticker = CreateSticker::default();
        f(&mut create_sticker);
        let map = json::hashmap_to_json_map(create_sticker.0);

        let file = match create_sticker.1 {
            Some(f) => f,
            None => return Err(Error::Model(ModelError::NoStickerFileSet)),
        };

        let sticker = http.as_ref().create_sticker(self.0, map, file, None).await?;

        Ok(sticker)
    }

    /// Deletes the current guild if the current account is the owner of the
    /// guild.
    ///
    /// Refer to [`Guild::delete`] for more information.
    ///
    /// **Note**: Requires the current user to be the owner of the guild.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user is not the owner of the guild.
    #[inline]
    pub async fn delete(self, http: impl AsRef<Http>) -> Result<PartialGuild> {
        http.as_ref().delete_guild(self.0).await
    }

    /// Deletes an [`Emoji`] from the guild.
    ///
    /// **Note**: Requires the [Manage Emojis and Stickers] permission.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user lacks permission,
    /// or if an Emoji with that Id does not exist.
    ///
    /// [Manage Emojis and Stickers]: Permissions::MANAGE_EMOJIS_AND_STICKERS
    #[inline]
    pub async fn delete_emoji(
        self,
        http: impl AsRef<Http>,
        emoji_id: impl Into<EmojiId>,
    ) -> Result<()> {
        http.as_ref().delete_emoji(self.0, emoji_id.into().0).await
    }

    /// Deletes an integration by Id from the guild.
    ///
    /// **Note**: Requires the [Manage Guild] permission.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user lacks permission,
    /// or if an integration with that Id does not exist.
    ///
    /// [Manage Guild]: Permissions::MANAGE_GUILD
    #[inline]
    pub async fn delete_integration(
        self,
        http: impl AsRef<Http>,
        integration_id: impl Into<IntegrationId>,
    ) -> Result<()> {
        http.as_ref().delete_guild_integration(self.0, integration_id.into().0).await
    }

    /// Deletes a [`Role`] by Id from the guild.
    ///
    /// Also see [`Role::delete`] if you have the `cache` and `model` features
    /// enabled.
    ///
    /// **Note**: Requires the [Manage Roles] permission.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user lacks permission,
    /// or if a role with that Id does not exist.
    ///
    /// [Manage Roles]: Permissions::MANAGE_ROLES
    #[inline]
    pub async fn delete_role(
        self,
        http: impl AsRef<Http>,
        role_id: impl Into<RoleId>,
    ) -> Result<()> {
        http.as_ref().delete_role(self.0, role_id.into().0).await
    }

    /// Deletes a specified scheduled event in the guild.
    ///
    /// **Note**: Requires the [Manage Events] permission.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user lacks permission, or if invalid data is given.
    ///
    /// [Manage Events]: Permissions::MANAGE_EVENTS
    #[inline]
    pub async fn delete_scheduled_event(
        self,
        http: impl AsRef<Http>,
        event_id: impl Into<ScheduledEventId>,
    ) -> Result<()> {
        http.as_ref().delete_scheduled_event(self.0, event_id.into().0).await
    }

    /// Deletes a [`Sticker`] by Id from the guild.
    ///
    /// **Note**: Requires the [Manage Emojis and Stickers] permission.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user lacks permission,
    /// or if a sticker with that Id does not exist.
    ///
    /// [Manage Emojis and Stickers]: crate::model::permissions::Permissions::MANAGE_EMOJIS_AND_STICKERS
    #[inline]
    pub async fn delete_sticker(
        self,
        http: impl AsRef<Http>,
        sticker_id: impl Into<StickerId>,
    ) -> Result<()> {
        http.as_ref().delete_sticker(self.0, sticker_id.into().0, None).await
    }

    /// Edits the current guild with new data where specified.
    ///
    /// Refer to [`Guild::edit`] for more information.
    ///
    /// **Note**: Requires the current user to have the [Manage Guild]
    /// permission.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user lacks permission,
    /// or if an invalid value is set.
    ///
    /// [Manage Guild]: Permissions::MANAGE_GUILD
    #[inline]
    pub async fn edit<F>(&mut self, http: impl AsRef<Http>, f: F) -> Result<PartialGuild>
    where
        F: FnOnce(&mut EditGuild) -> &mut EditGuild,
    {
        let mut edit_guild = EditGuild::default();
        f(&mut edit_guild);
        let map = json::hashmap_to_json_map(edit_guild.0);

        http.as_ref().edit_guild(self.0, &map, None).await
    }

    /// Edits an [`Emoji`]'s name in the guild.
    ///
    /// Also see [`Emoji::edit`] if you have the `cache` and `methods` features
    /// enabled.
    ///
    /// Requires the [Manage Emojis and Stickers] permission.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user lacks permission.
    ///
    /// [Manage Emojis and Stickers]: Permissions::MANAGE_EMOJIS_AND_STICKERS
    #[inline]
    pub async fn edit_emoji(
        self,
        http: impl AsRef<Http>,
        emoji_id: impl Into<EmojiId>,
        name: &str,
    ) -> Result<Emoji> {
        let map = json!({
            "name": name,
        });

        http.as_ref().edit_emoji(self.0, emoji_id.into().0, &map, None).await
    }

    /// Edits the properties of member of the guild, such as muting or
    /// nicknaming them.
    ///
    /// Refer to [`EditMember`]'s documentation for a full list of methods and
    /// permission restrictions.
    ///
    /// # Examples
    ///
    /// Mute a member and set their roles to just one role with a predefined Id:
    ///
    /// ```rust,ignore
    /// guild.edit_member(&context, user_id, |m| m.mute(true).roles(&vec![role_id]));
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user lacks the necessary permissions.
    #[inline]
    pub async fn edit_member<F>(
        self,
        http: impl AsRef<Http>,
        user_id: impl Into<UserId>,
        f: F,
    ) -> Result<Member>
    where
        F: FnOnce(&mut EditMember) -> &mut EditMember,
    {
        let mut edit_member = EditMember::default();
        f(&mut edit_member);
        let map = json::hashmap_to_json_map(edit_member.0);

        http.as_ref().edit_member(self.0, user_id.into().0, &map, None).await
    }

    /// Edits the current user's nickname for the guild.
    ///
    /// Pass [`None`] to reset the nickname.
    ///
    /// Requires the [Change Nickname] permission.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user lacks permission.
    ///
    /// [Change Nickname]: Permissions::CHANGE_NICKNAME
    #[inline]
    pub async fn edit_nickname(
        self,
        http: impl AsRef<Http>,
        new_nickname: Option<&str>,
    ) -> Result<()> {
        http.as_ref().edit_nickname(self.0, new_nickname).await
    }

    /// Edits a [`Role`], optionally setting its new fields.
    ///
    /// Requires the [Manage Roles] permission.
    ///
    /// # Examples
    ///
    /// Make a role hoisted:
    ///
    /// ```rust,ignore
    /// use serenity::model::{GuildId, RoleId};
    ///
    /// GuildId(7).edit_role(&context, RoleId(8), |r| r.hoist(true));
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user lacks permission.
    ///
    /// [Manage Roles]: Permissions::MANAGE_ROLES
    #[inline]
    pub async fn edit_role<F>(
        self,
        http: impl AsRef<Http>,
        role_id: impl Into<RoleId>,
        f: F,
    ) -> Result<Role>
    where
        F: FnOnce(&mut EditRole) -> &mut EditRole,
    {
        let mut edit_role = EditRole::default();
        f(&mut edit_role);
        let map = json::hashmap_to_json_map(edit_role.0);

        http.as_ref().edit_role(self.0, role_id.into().0, &map, None).await
    }

    /// Modifies a scheduled event in the guild with the data set, if any.
    ///
    /// **Note**: Requires the [Manage Events] permission.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user lacks permission, or if invalid data is given.
    ///
    /// [Manage Events]: Permissions::MANAGE_EVENTS
    pub async fn edit_scheduled_event<F>(
        self,
        http: impl AsRef<Http>,
        event_id: impl Into<ScheduledEventId>,
        f: F,
    ) -> Result<ScheduledEvent>
    where
        F: FnOnce(&mut EditScheduledEvent) -> &mut EditScheduledEvent,
    {
        let mut edit_scheduled_event = EditScheduledEvent::default();
        f(&mut edit_scheduled_event);
        let map = json::hashmap_to_json_map(edit_scheduled_event.0);

        http.as_ref().edit_scheduled_event(self.0, event_id.into().0, &map, None).await
    }

    /// Edits a [`Sticker`], optionally setting its fields.
    ///
    /// Requires the [Manage Emojis and Stickers] permission.
    ///
    /// # Examples
    ///
    /// Rename a sticker:
    ///
    /// ```rust,ignore
    /// guild.edit_sticker(&context, StickerId(7), |r| r.name("Bun bun meow"));
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user lacks permission.
    ///
    /// [Manage Emojis and Stickers]: crate::model::permissions::Permissions::MANAGE_EMOJIS_AND_STICKERS
    #[inline]
    pub async fn edit_sticker<F>(
        &self,
        http: impl AsRef<Http>,
        sticker_id: impl Into<StickerId>,
        f: F,
    ) -> Result<Sticker>
    where
        F: FnOnce(&mut EditSticker) -> &mut EditSticker,
    {
        let mut edit_sticker = EditSticker::default();
        f(&mut edit_sticker);
        let map = json::hashmap_to_json_map(edit_sticker.0);

        http.as_ref().edit_sticker(self.0, sticker_id.into().0, &map, None).await
    }

    /// Edits the order of [`Role`]s
    /// Requires the [Manage Roles] permission.
    ///
    /// # Examples
    ///
    /// Change the order of a role:
    ///
    /// ```rust,ignore
    /// use serenity::model::{GuildId, RoleId};
    /// GuildId(7).edit_role_position(&context, RoleId(8), 2);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`Error::Http`] if the current user lacks permission.
    ///
    /// [Manage Roles]: Permissions::MANAGE_ROLES
    #[inline]
    pub async fn edit_role_position(
        self,
        http: impl AsRef<Http>,
        role_id: impl Into<RoleId>,
        position: u64,
    ) -> Result<Vec<Role>> {
        http.as_ref().edit_role_position(self.0, role_id.into().0, position, None).await
    }

    /// Edits the [`GuildWelcomeScreen`].
    ///
    /// # Errors
    ///
    /// Returns an [`Error::Http`] if some mandatory fields are not provided.
    pub async fn edit_welcome_screen<F>(
        &self,
        http: impl AsRef<Http>,
        f: F,
    ) -> Result<GuildWelcomeScreen>
    where
        F: FnOnce(&mut EditGuildWelcomeScreen) -> &mut EditGuildWelcomeScreen,
    {
        let mut map = EditGuildWelcomeScreen::default();
        f(&mut map);

        http.as_ref()
            .edit_guild_welcome_screen(self.0, &Value::from(json::hashmap_to_json_map(map.0)))
            .await
    }

    /// Edits the [`GuildWidget`].
    ///
    /// # Errors
    ///
    /// Returns an [`Error::Http`] if the bot does not have the `MANAGE_GUILD`
    /// permission.
    pub async fn edit_widget<F>(&self, http: impl AsRef<Http>, f: F) -> Result<GuildWidget>
    where
        F: FnOnce(&mut EditGuildWidget) -> &mut EditGuildWidget,
    {
        let mut map = EditGuildWidget::default();
        f(&mut map);

        http.as_ref()
            .edit_guild_widget(self.0, &Value::from(json::hashmap_to_json_map(map.0)))
            .await
    }

    /// Gets all of the guild's roles over the REST API.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user is not in
    /// the guild.
    pub async fn roles(self, http: impl AsRef<Http>) -> Result<HashMap<RoleId, Role>> {
        let mut roles = HashMap::new();

        #[allow(clippy::useless_conversion)]
        for role in http.as_ref().get_guild_roles(self.0).await? {
            roles.insert(role.id, role);
        }

        Ok(roles)
    }

    /// Tries to find the [`Guild`] by its Id in the cache.
    #[cfg(feature = "cache")]
    #[inline]
    pub fn to_guild_cached(self, cache: impl AsRef<Cache>) -> Option<Guild> {
        cache.as_ref().guild(self)
    }

    /// Requests [`PartialGuild`] over REST API.
    ///
    /// **Note**: This will not be a [`Guild`], as the REST API does not send
    /// all data with a guild retrieval.
    ///
    /// # Errors
    ///
    /// Returns an [`Error::Http`] if the current user is not in the guild.
    #[inline]
    pub async fn to_partial_guild(self, http: impl AsRef<Http>) -> Result<PartialGuild> {
        http.as_ref().get_guild(self.0).await
    }

    /// Requests [`PartialGuild`] over REST API with counts.
    ///
    /// **Note**: This will not be a [`Guild`], as the REST API does not send
    /// all data with a guild retrieval.
    ///
    /// # Errors
    ///
    /// Returns an [`Error::Http`] if the current user is not in the guild.
    #[inline]
    pub async fn to_partial_guild_with_counts(
        self,
        http: impl AsRef<Http>,
    ) -> Result<PartialGuild> {
        http.as_ref().get_guild_with_counts(self.0).await
    }

    /// Gets all [`Emoji`]s of this guild via HTTP.
    ///
    /// # Errors
    ///
    /// Returns an [`Error::Http`] if the guild is unavailable.
    #[inline]
    pub async fn emojis(&self, http: impl AsRef<Http>) -> Result<Vec<Emoji>> {
        http.as_ref().get_emojis(self.0).await
    }

    /// Gets an [`Emoji`] of this guild by its ID via HTTP.
    ///
    /// # Errors
    ///
    /// Returns an [`Error::Http`] if an emoji with that Id does not exist.
    #[inline]
    pub async fn emoji(&self, http: impl AsRef<Http>, emoji_id: EmojiId) -> Result<Emoji> {
        http.as_ref().get_emoji(self.0, emoji_id.0).await
    }

    /// Gets all [`Sticker`]s of this guild via HTTP.
    ///
    /// # Errors
    ///
    /// Returns an [`Error::Http`] if the guild is unavailable.
    #[inline]
    pub async fn stickers(&self, http: impl AsRef<Http>) -> Result<Vec<Sticker>> {
        http.as_ref().get_guild_stickers(self.0).await
    }

    /// Gets an [`Sticker`] of this guild by its ID via HTTP.
    ///
    /// # Errors
    ///
    /// Returns an [`Error::Http`] if an sticker with that Id does not exist.
    #[inline]
    pub async fn sticker(&self, http: impl AsRef<Http>, sticker_id: StickerId) -> Result<Sticker> {
        http.as_ref().get_guild_sticker(self.0, sticker_id.0).await
    }

    /// Gets all integration of the guild.
    ///
    /// Requires the [Manage Guild] permission.
    ///
    /// # Errors
    ///
    /// Returns an [`Error::Http`] if the current user lacks permission,
    /// also may return [`Error::Json`] if there is an error in deserializing
    /// the API response.
    ///
    /// [Manage Guild]: Permissions::MANAGE_GUILD
    #[inline]
    pub async fn integrations(self, http: impl AsRef<Http>) -> Result<Vec<Integration>> {
        http.as_ref().get_guild_integrations(self.0).await
    }

    /// Gets all of the guild's invites.
    ///
    /// Requires the [Manage Guild] permission.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user lacks permission,
    /// also may return [`Error::Json`] if there is an error in
    /// deserializing the API response.
    ///
    /// [Manage Guild]: Permissions::MANAGE_GUILD
    #[inline]
    pub async fn invites(self, http: impl AsRef<Http>) -> Result<Vec<RichInvite>> {
        http.as_ref().get_guild_invites(self.0).await
    }

    /// Kicks a [`Member`] from the guild.
    ///
    /// Requires the [Kick Members] permission.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the member cannot be kicked by
    /// the current user.
    ///
    /// [Kick Members]: Permissions::KICK_MEMBERS
    #[inline]
    pub async fn kick(self, http: impl AsRef<Http>, user_id: impl Into<UserId>) -> Result<()> {
        http.as_ref().kick_member(self.0, user_id.into().0).await
    }

    /// # Errors
    ///
    /// In addition to the reasons [`Self::kick`] may return an error,
    /// may also return an error if the reason is too long.
    #[inline]
    pub async fn kick_with_reason(
        self,
        http: impl AsRef<Http>,
        user_id: impl Into<UserId>,
        reason: &str,
    ) -> Result<()> {
        http.as_ref().kick_member_with_reason(self.0, user_id.into().0, reason).await
    }

    /// Leaves the guild.
    ///
    /// # Errors
    ///
    /// May return an [`Error::Http`] if the current user
    /// cannot leave the guild, or currently is not in the guild.
    #[inline]
    pub async fn leave(self, http: impl AsRef<Http>) -> Result<()> {
        http.as_ref().leave_guild(self.0).await
    }

    /// Gets a user's [`Member`] for the guild by Id.
    ///
    /// If the cache feature is enabled the cache will be checked
    /// first. If not found it will resort to an http request.
    ///
    /// # Errors
    ///
    /// Returns an [`Error::Http`] if the user is not in the guild,
    /// or if the guild is otherwise unavailable
    #[inline]
    pub async fn member(
        self,
        cache_http: impl CacheHttp,
        user_id: impl Into<UserId>,
    ) -> Result<Member> {
        let user_id = user_id.into();

        #[cfg(feature = "cache")]
        {
            if let Some(cache) = cache_http.cache() {
                if let Some(member) = cache.member(self.0, user_id) {
                    return Ok(member);
                }
            }
        }

        cache_http.http().get_member(self.0, user_id.0).await
    }

    /// Gets a list of the guild's members.
    ///
    /// Optionally pass in the `limit` to limit the number of results.
    /// Minimum value is 1, maximum and default value is 1000.
    ///
    /// Optionally pass in `after` to offset the results by a [`User`]'s Id.
    ///
    /// # Errors
    ///
    /// Returns an [`Error::Http`] if the API returns an error, may also
    /// return [`Error::NotInRange`] if the input is not within range.
    ///
    /// [`User`]: crate::model::user::User
    #[inline]
    pub async fn members(
        self,
        http: impl AsRef<Http>,
        limit: Option<u64>,
        after: impl Into<Option<UserId>>,
    ) -> Result<Vec<Member>> {
        http.as_ref().get_guild_members(self.0, limit, after.into().map(|x| x.0)).await
    }

    /// Streams over all the members in a guild.
    ///
    /// This is accomplished and equivalent to repeated calls to [`Self::members`].
    /// A buffer of at most 1,000 members is used to reduce the number of calls
    /// necessary.
    ///
    /// # Examples
    /// ```rust,no_run
    /// # use serenity::model::id::GuildId;
    /// # use serenity::http::Http;
    /// #
    /// # async fn run() {
    /// # let guild_id = GuildId::default();
    /// # let ctx = Http::new("token");
    /// use serenity::futures::StreamExt;
    /// use serenity::model::guild::MembersIter;
    ///
    /// let mut members = guild_id.members_iter(&ctx).boxed();
    /// while let Some(member_result) = members.next().await {
    ///     match member_result {
    ///         Ok(member) => println!("{} is {}", member, member.display_name(),),
    ///         Err(error) => eprintln!("Uh oh!  Error: {}", error),
    ///     }
    /// }
    /// # }
    /// ```
    pub fn members_iter<H: AsRef<Http>>(self, http: H) -> impl Stream<Item = Result<Member>> {
        MembersIter::<H>::stream(http, self)
    }

    /// Moves a member to a specific voice channel.
    ///
    /// Requires the [Move Members] permission.
    ///
    /// # Errors
    ///
    /// Returns an [`Error::Http`] if the current user
    /// lacks permission, or if the member is not currently
    /// in a voice channel for this [`Guild`].
    ///
    /// [Move Members]: Permissions::MOVE_MEMBERS
    #[inline]
    pub async fn move_member(
        self,
        http: impl AsRef<Http>,
        user_id: impl Into<UserId>,
        channel_id: impl Into<ChannelId>,
    ) -> Result<Member> {
        let mut map = JsonMap::new();
        map.insert("channel_id".to_string(), from_number(channel_id.into().0));

        http.as_ref().edit_member(self.0, user_id.into().0, &map, None).await
    }

    /// Returns the name of whatever guild this id holds.
    #[cfg(feature = "cache")]
    #[must_use]
    pub fn name(self, cache: impl AsRef<Cache>) -> Option<String> {
        let guild = self.to_guild_cached(&cache)?;
        Some(guild.name)
    }

    /// Disconnects a member from a voice channel in the guild.
    ///
    /// Requires the [Move Members] permission.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user lacks permission,
    /// or if the member is not currently in a voice channel for this guild.
    ///
    /// [Move Members]: Permissions::MOVE_MEMBERS
    #[inline]
    pub async fn disconnect_member(
        self,
        http: impl AsRef<Http>,
        user_id: impl Into<UserId>,
    ) -> Result<Member> {
        let mut map = JsonMap::new();
        map.insert("channel_id".to_string(), NULL);
        http.as_ref().edit_member(self.0, user_id.into().0, &map, None).await
    }

    /// Gets the number of [`Member`]s that would be pruned with the given
    /// number of days.
    ///
    /// Requires the [Kick Members] permission.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user does not have permission.
    ///
    /// [Kick Members]: Permissions::KICK_MEMBERS
    #[inline]
    pub async fn prune_count(self, http: impl AsRef<Http>, days: u16) -> Result<GuildPrune> {
        let map = json!({
            "days": days,
        });

        http.as_ref().get_guild_prune_count(self.0, &map).await
    }

    /// Re-orders the channels of the guild.
    ///
    /// Accepts an iterator of a tuple of the channel ID to modify and its new
    /// position.
    ///
    /// Although not required, you should specify all channels' positions,
    /// regardless of whether they were updated. Otherwise, positioning can
    /// sometimes get weird.
    ///
    /// **Note**: Requires the [Manage Channels] permission.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user lacks permission.
    ///
    /// [Manage Channels]: Permissions::MANAGE_CHANNELS
    #[inline]
    pub async fn reorder_channels<It>(self, http: impl AsRef<Http>, channels: It) -> Result<()>
    where
        It: IntoIterator<Item = (ChannelId, u64)>,
    {
        let items = channels
            .into_iter()
            .map(|(id, pos)| {
                json!({
                    "id": id,
                    "position": pos,
                })
            })
            .collect::<Vec<_>>();

        http.as_ref().edit_guild_channel_positions(self.0, &Value::from(items)).await
    }

    /// Returns a list of [`Member`]s in a [`Guild`] whose username or nickname
    /// starts with a provided string.
    ///
    /// Optionally pass in the `limit` to limit the number of results.
    /// Minimum value is 1, maximum and default value is 1000.
    ///
    /// # Errors
    ///
    /// Returns an [`Error::Http`] if the API returns an error.
    #[inline]
    pub async fn search_members(
        self,
        http: impl AsRef<Http>,
        query: &str,
        limit: Option<u64>,
    ) -> Result<Vec<Member>> {
        http.as_ref().search_guild_members(self.0, query, limit).await
    }

    /// Fetches a specified scheduled event in the guild, by Id. If `with_user_count` is set to
    /// `true`, then the `user_count` field will be populated, indicating the number of users
    /// interested in the event.
    ///
    /// **Note**: Requires the [Manage Events] permission.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user lacks permission, or if the provided Id is
    /// invalid.
    ///
    /// [Manage Events]: Permissions::MANAGE_EVENTS
    pub async fn scheduled_event(
        self,
        http: impl AsRef<Http>,
        event_id: impl Into<ScheduledEventId>,
        with_user_count: bool,
    ) -> Result<ScheduledEvent> {
        http.as_ref().get_scheduled_event(self.0, event_id.into().0, with_user_count).await
    }

    /// Fetches a list of all scheduled events in the guild. If `with_user_count` is set to `true`,
    /// then each event returned will have its `user_count` field populated.
    ///
    /// **Note**: Requires the [Manage Events] permission.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user lacks permission.
    ///
    /// [Manage Events]: Permissions::MANAGE_EVENTS
    pub async fn scheduled_events(
        self,
        http: impl AsRef<Http>,
        with_user_count: bool,
    ) -> Result<Vec<ScheduledEvent>> {
        http.as_ref().get_scheduled_events(self.0, with_user_count).await
    }

    /// Fetches a list of interested users for the specified event.
    ///
    /// If `limit` is left unset, by default at most 100 users are returned.
    ///
    /// **Note**: Requires the [Manage Events] permission.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user lacks permission, or if the provided Id is
    /// invalid.
    ///
    /// [Manage Events]: Permissions::MANAGE_EVENTS
    pub async fn scheduled_event_users(
        self,
        http: impl AsRef<Http>,
        event_id: impl Into<ScheduledEventId>,
        limit: Option<u64>,
    ) -> Result<Vec<ScheduledEventUser>> {
        http.as_ref().get_scheduled_event_users(self.0, event_id.into().0, limit, None, None).await
    }

    /// Fetches a list of interested users for the specified event, with additional options and
    /// filtering. See [`Http::get_scheduled_event_users`] for details.
    ///
    /// **Note**: Requires the [Manage Events] permission.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user lacks permission, or if the provided Id is
    /// invalid.
    ///
    /// [Manage Events]: Permissions::MANAGE_EVENTS
    pub async fn scheduled_event_users_optioned(
        self,
        http: impl AsRef<Http>,
        event_id: impl Into<ScheduledEventId>,
        limit: Option<u64>,
        target: Option<UserPagination>,
        with_member: Option<bool>,
    ) -> Result<Vec<ScheduledEventUser>> {
        http.as_ref()
            .get_scheduled_event_users(self.0, event_id.into().0, limit, target, with_member)
            .await
    }

    /// Returns the Id of the shard associated with the guild.
    ///
    /// When the cache is enabled this will automatically retrieve the total
    /// number of shards.
    ///
    /// **Note**: When the cache is enabled, this function unlocks the cache to
    /// retrieve the total number of shards in use. If you already have the
    /// total, consider using [`utils::shard_id`].
    ///
    /// [`utils::shard_id`]: crate::utils::shard_id
    #[cfg(all(feature = "cache", feature = "utils"))]
    #[inline]
    #[must_use]
    pub fn shard_id(self, cache: impl AsRef<Cache>) -> u64 {
        crate::utils::shard_id(self.0, cache.as_ref().shard_count())
    }

    /// Returns the Id of the shard associated with the guild.
    ///
    /// When the cache is enabled this will automatically retrieve the total
    /// number of shards.
    ///
    /// When the cache is not enabled, the total number of shards being used
    /// will need to be passed.
    ///
    /// # Examples
    ///
    /// Retrieve the Id of the shard for a guild with Id `81384788765712384`,
    /// using 17 shards:
    ///
    /// ```rust
    /// use serenity::model::id::GuildId;
    /// use serenity::utils;
    ///
    /// let guild_id = GuildId(81384788765712384);
    ///
    /// assert_eq!(guild_id.shard_id(17), 7);
    /// ```
    #[cfg(all(feature = "utils", not(feature = "cache")))]
    #[inline]
    #[must_use]
    pub fn shard_id(self, shard_count: u64) -> u64 {
        crate::utils::shard_id(self.0, shard_count)
    }

    /// Starts an integration sync for the given integration Id.
    ///
    /// Requires the [Manage Guild] permission.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user lacks permission,
    /// or if an [`Integration`] with that Id does not exist.
    ///
    /// [Manage Guild]: Permissions::MANAGE_GUILD
    #[inline]
    pub async fn start_integration_sync(
        self,
        http: impl AsRef<Http>,
        integration_id: impl Into<IntegrationId>,
    ) -> Result<()> {
        http.as_ref().start_integration_sync(self.0, integration_id.into().0).await
    }

    /// Starts a prune of [`Member`]s.
    ///
    /// See the documentation on [`GuildPrune`] for more information.
    ///
    /// **Note**: Requires the [Kick Members] permission.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user lacks permission.
    ///
    /// [Kick Members]: Permissions::KICK_MEMBERS
    #[inline]
    pub async fn start_prune(self, http: impl AsRef<Http>, days: u16) -> Result<GuildPrune> {
        http.as_ref().start_guild_prune(self.0, days as u64, None).await
    }

    /// Unbans a [`User`] from the guild.
    ///
    /// **Note**: Requires the [Ban Members] permission.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the current user does not have permission.
    ///
    /// [Ban Members]: Permissions::BAN_MEMBERS
    #[inline]
    pub async fn unban(self, http: impl AsRef<Http>, user_id: impl Into<UserId>) -> Result<()> {
        http.as_ref().remove_ban(self.0, user_id.into().0, None).await
    }

    /// Retrieve's the guild's vanity URL.
    ///
    /// **Note**: Requires the [Manage Guild] permission.
    ///
    /// # Errors
    ///
    /// Will return [`Error::Http`] if the current user lacks permission.
    /// Can also return [`Error::Json`] if there is an error deserializing
    /// the API response.
    ///
    /// [Manage Guild]: Permissions::MANAGE_GUILD
    #[inline]
    pub async fn vanity_url(self, http: impl AsRef<Http>) -> Result<String> {
        http.as_ref().get_guild_vanity_url(self.0).await
    }

    /// Retrieves the guild's webhooks.
    ///
    /// **Note**: Requires the [Manage Webhooks] permission.
    ///
    /// [Manage Webhooks]: Permissions::MANAGE_WEBHOOKS
    ///
    /// # Errors
    ///
    /// Will return an [`Error::Http`] if the bot is lacking permissions.
    /// Can also return an [`Error::Json`] if there is an error deserializing
    /// the API response.
    #[inline]
    pub async fn webhooks(self, http: impl AsRef<Http>) -> Result<Vec<Webhook>> {
        http.as_ref().get_guild_webhooks(self.0).await
    }

    /// Returns a future that will await one message sent in this guild.
    #[cfg(feature = "collector")]
    pub fn await_reply(&self, shard_messenger: impl AsRef<ShardMessenger>) -> CollectReply {
        CollectReply::new(shard_messenger).guild_id(self.0)
    }

    /// Returns a stream builder which can be awaited to obtain a stream of messages in this guild.
    #[cfg(feature = "collector")]
    pub fn await_replies(
        &self,
        shard_messenger: impl AsRef<ShardMessenger>,
    ) -> MessageCollectorBuilder {
        MessageCollectorBuilder::new(shard_messenger).guild_id(self.0)
    }

    /// Await a single reaction in this guild.
    #[cfg(feature = "collector")]
    pub fn await_reaction(&self, shard_messenger: impl AsRef<ShardMessenger>) -> CollectReaction {
        CollectReaction::new(shard_messenger).guild_id(self.0)
    }

    /// Returns a stream builder which can be awaited to obtain a stream of reactions sent in this guild.
    #[cfg(feature = "collector")]
    pub fn await_reactions(
        &self,
        shard_messenger: impl AsRef<ShardMessenger>,
    ) -> ReactionCollectorBuilder {
        ReactionCollectorBuilder::new(shard_messenger).guild_id(self.0)
    }

    /// Creates a guild specific [`Command`]
    ///
    /// **Note**: Unlike global `Command`s, guild commands will update instantly.
    ///
    /// # Errors
    ///
    /// Returns the same possible errors as [`create_global_application_command`].
    ///
    /// [`create_global_application_command`]: Command::create_global_application_command
    pub async fn create_application_command<F>(
        &self,
        http: impl AsRef<Http>,
        f: F,
    ) -> Result<Command>
    where
        F: FnOnce(&mut CreateApplicationCommand) -> &mut CreateApplicationCommand,
    {
        let map = Command::build_application_command(f);
        http.as_ref().create_guild_application_command(self.0, &Value::from(map)).await
    }

    /// Overrides all guild application commands.
    ///
    /// # Errors
    ///
    /// Returns the same possible errors as [`set_global_application_commands`].
    ///
    /// [`set_global_application_commands`]: Command::set_global_application_commands
    pub async fn set_application_commands<F>(
        &self,
        http: impl AsRef<Http>,
        f: F,
    ) -> Result<Vec<Command>>
    where
        F: FnOnce(&mut CreateApplicationCommands) -> &mut CreateApplicationCommands,
    {
        let mut array = CreateApplicationCommands::default();

        f(&mut array);

        http.as_ref().create_guild_application_commands(self.0, &Value::from(array.0)).await
    }

    /// Creates a guild specific [`CommandPermission`].
    ///
    /// **Note**: It will update instantly.
    ///
    /// # Errors
    ///
    /// If there is an error, it will be either [`Error::Http`] or [`Error::Json`].
    pub async fn create_application_command_permission<F>(
        &self,
        http: impl AsRef<Http>,
        command_id: CommandId,
        f: F,
    ) -> Result<CommandPermission>
    where
        F: FnOnce(
            &mut CreateApplicationCommandPermissionsData,
        ) -> &mut CreateApplicationCommandPermissionsData,
    {
        let mut map = CreateApplicationCommandPermissionsData::default();
        f(&mut map);

        http.as_ref()
            .edit_guild_application_command_permissions(
                self.0,
                command_id.into(),
                &Value::from(json::hashmap_to_json_map(map.0)),
            )
            .await
    }

    /// Overrides all application commands permissions.
    ///
    /// # Errors
    ///
    /// If there is an error, it will be either [`Error::Http`] or [`Error::Json`].
    #[deprecated(note = "use `create_appliction_command_permission`.")]
    #[allow(deprecated)]
    pub async fn set_application_commands_permissions<F>(
        &self,
        http: impl AsRef<Http>,
        f: F,
    ) -> Result<Vec<CommandPermission>>
    where
        F: FnOnce(
            &mut crate::builder::CreateApplicationCommandsPermissions,
        ) -> &mut crate::builder::CreateApplicationCommandsPermissions,
    {
        let mut map = crate::builder::CreateApplicationCommandsPermissions::default();
        f(&mut map);

        http.as_ref().edit_guild_application_commands_permissions(self.0, &Value::from(map.0)).await
    }

    /// Get all guild application commands.
    ///
    /// # Errors
    ///
    /// If there is an error, it will be either [`Error::Http`] or [`Error::Json`].
    pub async fn get_application_commands(&self, http: impl AsRef<Http>) -> Result<Vec<Command>> {
        http.as_ref().get_guild_application_commands(self.0).await
    }

    /// Get a specific guild application command by its Id.
    ///
    /// # Errors
    ///
    /// If there is an error, it will be either [`Error::Http`] or [`Error::Json`].
    pub async fn get_application_command(
        &self,
        http: impl AsRef<Http>,
        command_id: CommandId,
    ) -> Result<Command> {
        http.as_ref().get_guild_application_command(self.0, command_id.into()).await
    }

    /// Edit guild application command by its Id.
    ///
    /// # Errors
    ///
    /// If there is an error, it will be either [`Error::Http`] or [`Error::Json`].
    pub async fn edit_application_command<F>(
        &self,
        http: impl AsRef<Http>,
        command_id: CommandId,
        f: F,
    ) -> Result<Command>
    where
        F: FnOnce(&mut CreateApplicationCommand) -> &mut CreateApplicationCommand,
    {
        let map = Command::build_application_command(f);
        http.as_ref()
            .edit_guild_application_command(self.0, command_id.into(), &Value::from(map))
            .await
    }

    /// Delete guild application command by its Id.
    ///
    /// # Errors
    ///
    /// If there is an error, it will be either [`Error::Http`] or [`Error::Json`].
    pub async fn delete_application_command(
        &self,
        http: impl AsRef<Http>,
        command_id: CommandId,
    ) -> Result<()> {
        http.as_ref().delete_guild_application_command(self.0, command_id.into()).await
    }

    /// Get all guild application commands permissions only.
    ///
    /// # Errors
    ///
    /// If there is an error, it will be either [`Error::Http`] or [`Error::Json`].
    pub async fn get_application_commands_permissions(
        &self,
        http: impl AsRef<Http>,
    ) -> Result<Vec<CommandPermission>> {
        http.as_ref().get_guild_application_commands_permissions(self.0).await
    }

    /// Get permissions for specific guild application command by its Id.
    ///
    /// # Errors
    ///
    /// If there is an error, it will be either [`Error::Http`] or [`Error::Json`].
    pub async fn get_application_command_permissions(
        &self,
        http: impl AsRef<Http>,
        command_id: CommandId,
    ) -> Result<CommandPermission> {
        http.as_ref().get_guild_application_command_permissions(self.0, command_id.into()).await
    }

    /// Get the guild welcome screen.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the guild does not have a welcome screen.
    pub async fn get_welcome_screen(&self, http: impl AsRef<Http>) -> Result<GuildWelcomeScreen> {
        http.as_ref().get_guild_welcome_screen(self.0).await
    }

    /// Get the guild preview.
    ///
    /// **Note**: The bot need either to be part of the guild
    /// or the guild needs to have the `DISCOVERABLE` feature.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the bot cannot see the guild preview, see the note.
    pub async fn get_preview(&self, http: impl AsRef<Http>) -> Result<GuildPreview> {
        http.as_ref().get_guild_preview(self.0).await
    }

    /// Get the guild widget.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if the bot does not have `MANAGE_MESSAGES` permission.
    pub async fn get_widget(&self, http: impl AsRef<Http>) -> Result<GuildWidget> {
        http.as_ref().get_guild_widget(self.0).await
    }

    /// Get the widget image URL.
    #[must_use]
    pub fn widget_image_url(&self, style: GuildWidgetStyle) -> String {
        api!("/guilds/{}/widget.png?style={}", self.0, style)
    }

    /// Gets the guild active threads.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Http`] if there is an error in the deserialization, or
    /// if the bot issuing the request is not in the guild.
    pub async fn get_active_threads(&self, http: impl AsRef<Http>) -> Result<ThreadsData> {
        http.as_ref().get_guild_active_threads(self.0).await
    }
}

impl From<PartialGuild> for GuildId {
    /// Gets the Id of a partial guild.
    fn from(guild: PartialGuild) -> GuildId {
        guild.id
    }
}

impl<'a> From<&'a PartialGuild> for GuildId {
    /// Gets the Id of a partial guild.
    fn from(guild: &PartialGuild) -> GuildId {
        guild.id
    }
}

impl From<GuildInfo> for GuildId {
    /// Gets the Id of Guild information struct.
    fn from(guild_info: GuildInfo) -> GuildId {
        guild_info.id
    }
}

impl<'a> From<&'a GuildInfo> for GuildId {
    /// Gets the Id of Guild information struct.
    fn from(guild_info: &GuildInfo) -> GuildId {
        guild_info.id
    }
}

impl From<InviteGuild> for GuildId {
    /// Gets the Id of Invite Guild struct.
    fn from(invite_guild: InviteGuild) -> GuildId {
        invite_guild.id
    }
}

impl<'a> From<&'a InviteGuild> for GuildId {
    /// Gets the Id of Invite Guild struct.
    fn from(invite_guild: &InviteGuild) -> GuildId {
        invite_guild.id
    }
}

impl From<Guild> for GuildId {
    /// Gets the Id of Guild.
    fn from(live_guild: Guild) -> GuildId {
        live_guild.id
    }
}

impl<'a> From<&'a Guild> for GuildId {
    /// Gets the Id of Guild.
    fn from(live_guild: &Guild) -> GuildId {
        live_guild.id
    }
}

/// A helper class returned by [`GuildId::members_iter`]
#[derive(Clone, Debug)]
#[cfg(feature = "model")]
pub struct MembersIter<H: AsRef<Http>> {
    guild_id: GuildId,
    http: H,
    buffer: Vec<Member>,
    after: Option<UserId>,
    tried_fetch: bool,
}

#[cfg(feature = "model")]
impl<H: AsRef<Http>> MembersIter<H> {
    fn new(guild_id: GuildId, http: H) -> MembersIter<H> {
        MembersIter {
            guild_id,
            http,
            buffer: Vec::new(),
            after: None,
            tried_fetch: false,
        }
    }

    /// Fills the `self.buffer` cache of Members.
    ///
    /// This drops any members that
    /// were currently in the buffer, so it should only be called when
    /// `self.buffer` is empty.  Additionally, this updates `self.after` so that
    /// the next call does not return duplicate items.  If there are no more
    /// members to be fetched, then this marks `self.after` as None, indicating
    /// that no more calls ought to be made.
    async fn refresh(&mut self) -> Result<()> {
        // Number of profiles to fetch
        let grab_size: u64 = 1000;

        self.buffer = self.guild_id.members(&self.http, Some(grab_size), self.after).await?;

        // Get the last member.  If shorter than 1000, there are no more results anyway
        self.after = self.buffer.get(grab_size as usize - 1).map(|member| member.user.id);

        // Reverse to optimize pop()
        self.buffer.reverse();

        self.tried_fetch = true;

        Ok(())
    }

    /// Streams over all the members in a guild.
    ///
    /// This is accomplished and equivalent to repeated calls to [`GuildId::members`].
    /// A buffer of at most 1,000 members is used to reduce the number of calls
    /// necessary.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use serenity::model::id::GuildId;
    /// # use serenity::http::Http;
    /// #
    /// # async fn run() {
    /// # let guild_id = GuildId::default();
    /// # let ctx = Http::new("token");
    /// use serenity::futures::StreamExt;
    /// use serenity::model::guild::MembersIter;
    ///
    /// let mut members = MembersIter::<Http>::stream(&ctx, guild_id).boxed();
    /// while let Some(member_result) = members.next().await {
    ///     match member_result {
    ///         Ok(member) => println!("{} is {}", member, member.display_name(),),
    ///         Err(error) => eprintln!("Uh oh!  Error: {}", error),
    ///     }
    /// }
    /// # }
    /// ```
    pub fn stream(http: impl AsRef<Http>, guild_id: GuildId) -> impl Stream<Item = Result<Member>> {
        let init_state = MembersIter::new(guild_id, http);

        futures::stream::unfold(init_state, |mut state| async {
            if state.buffer.is_empty() && state.after.is_some() || !state.tried_fetch {
                if let Err(error) = state.refresh().await {
                    return Some((Err(error), state));
                }
            }

            state.buffer.pop().map(|entry| (Ok(entry), state))
        })
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum GuildWidgetStyle {
    Shield,
    Banner1,
    Banner2,
    Banner3,
    Banner4,
}

impl fmt::Display for GuildWidgetStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Shield => f.write_str("shield"),
            Self::Banner1 => f.write_str("banner1"),
            Self::Banner2 => f.write_str("banner2"),
            Self::Banner3 => f.write_str("banner3"),
            Self::Banner4 => f.write_str("banner4"),
        }
    }
}
