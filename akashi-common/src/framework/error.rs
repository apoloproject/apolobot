use crate::framework::akashi::{AkashiData, AkashiError};

pub async fn on_error(error: poise::FrameworkError<'_, AkashiData, AkashiError>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!(
                "poise::FrameworkError::Command from {}: {:?}",
                ctx.command().name,
                error
            );
            ctx.say(format!("❌ {}", error)).await.unwrap();
        }
        poise::FrameworkError::CommandPanic { payload, ctx, .. } => {
            println!(
                "poise::FrameworkError::CommandPanic from {}: {:?}",
                ctx.command().name,
                payload
            );
            ctx.say(
                payload
                    .unwrap_or("Unknown error check console".to_string())
                    .to_string(),
            )
            .await
            .unwrap();
        }
        poise::FrameworkError::MissingBotPermissions {
            missing_permissions,
            ctx,
            ..
        } => {
            println!("Missing permissions: {:?}", missing_permissions);
            ctx.say(format!(
                "❌ I lack the following permissions for this action: {:?}",
                missing_permissions
            ))
            .await
            .unwrap();
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}
