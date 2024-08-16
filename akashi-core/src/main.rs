#![forbid(unsafe_code)]

use std::env::var;
use std::sync::Arc;
use std::time::Duration;

use poise::serenity_prelude;
use poise::serenity_prelude::ActivityData;

use akashi_commands::register_all_commands;
use akashi_common::framework::akashi::{initialize_data, initialize_owners, AkashiError};
use akashi_common::framework::error::on_error;
use akashi_common::framework::events::event_handler;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let options = poise::FrameworkOptions {
        commands: register_all_commands(),
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some(";".into()),
            edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                Duration::from_secs(30),
            ))),
            additional_prefixes: vec![
                poise::Prefix::Literal("akashi"),
                poise::Prefix::Literal("nya"), // Chiko: Don't get me wrong, Akashi is a catgirl.
                #[cfg(debug_assertions)]
                poise::Prefix::Literal("dev"),
            ],
            mention_as_prefix: true,
            ..Default::default()
        },
        on_error: |e| Box::pin(on_error(e)),
        event_handler: |framework, event| Box::pin(event_handler(framework, event)),
        owners: initialize_owners(),
        command_check: Some(|ctx| {
            Box::pin(async move {
                if ctx.author().id == 858367536240394259 {
                    // Sawako is not allowed to make use of Akashi anyway
                    return Err(AkashiError::from("You can't use Akashi"));
                }

                Ok(true)
            })
        }),
        allowed_mentions: Some(
            serenity_prelude::CreateAllowedMentions::default()
                .all_users(false)
                .everyone(false)
                .all_roles(false),
        ),
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, _, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                ctx.set_activity(Some(ActivityData::listening(format!(
                    "{} guilds",
                    ctx.cache.guild_count()
                ))));

                Ok(initialize_data())
            })
        })
        .options(options)
        .build();

    let token = var("DISCORD_TOKEN").expect("DISCORD_TOKEN was not set");
    let intents = serenity_prelude::GatewayIntents::non_privileged()
        | serenity_prelude::GatewayIntents::MESSAGE_CONTENT
        | serenity_prelude::GatewayIntents::GUILD_EMOJIS_AND_STICKERS;

    let mut cache_settings = serenity_prelude::cache::Settings::default();

    cache_settings.cache_users = false;
    cache_settings.cache_channels = false;
    cache_settings.cache_guilds = true;

    let client = serenity_prelude::ClientBuilder::new(token, intents)
        .framework(framework)
        .cache_settings(cache_settings)
        .await;

    client.unwrap().start().await.unwrap();
}
