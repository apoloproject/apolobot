use std::process::Command;

use akashi_common::framework::akashi::{AkashiContext, AkashiError};

/// Show this embed
#[poise::command(slash_command, prefix_command, category = "Util")]
pub async fn help(ctx: AkashiContext<'_>, command: Option<String>) -> Result<(), AkashiError> {
    let commit = Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .map(|x| (String::from_utf8_lossy(x.stdout.as_slice()))[..8].to_owned())
        .unwrap_or("idk".to_owned());

    // get help configuration
    let help_cfg = poise::builtins::PrettyHelpConfiguration {
        include_description: true,
        extra_text_at_bottom: &format!("version {commit}"),
        ..Default::default()
    };

    poise::builtins::pretty_help(ctx, command.as_deref(), help_cfg).await?;
    Ok(())
}
