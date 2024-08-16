use akashi_common::discord::ansi::Ansi;
use akashi_common::discord::markdown::Markdown;
use akashi_common::discord::table::key_value;
use akashi_common::framework::akashi::{AkashiContext, AkashiError};
use akashi_common::structs::api::HealthResponse;

/// Show Akashi's statistics
#[poise::command(slash_command, prefix_command, category = "Util", guild_cooldown = 2)]
pub async fn stats(ctx: AkashiContext<'_>) -> Result<(), AkashiError> {
    let (client_used_memory, client_cpu_usage, client_guild_count) = get_client_stats(ctx).await;
    let (api_cpu, api_memory, api_uptime, api_version) = get_api_stats(ctx).await?;
    let cache = ctx.data().custom_cache.lock().await;
    let images = format!(
        "{}{}",
        cache.images.size().fg_yellow(),
        format!("/{}", cache.images.capacity().to_string()).fg_black()
    );

    let client_stats: Vec<(String, String)> = vec![
        (
            "Memory".fg_red(),
            format!("{client_used_memory}mb").fg_yellow(),
        ),
        (
            "CPU usage".fg_red(),
            format!("{client_cpu_usage:.1}%").fg_yellow(),
        ),
        (
            "Guilds".fg_red(),
            client_guild_count.to_string().fg_yellow(),
        ),
        ("Images".fg_red(), images),
    ];

    let api_stats: Vec<(String, String)> = vec![
        ("Memory".fg_red(), format!("{api_memory}mb").fg_yellow()),
        ("CPU usage".fg_red(), format!("{api_cpu:.1}%").fg_yellow()),
        ("Uptime".fg_red(), api_uptime.to_string().fg_yellow()),
        ("Version".fg_red(), api_version.fg_yellow()),
    ];

    let client_stats_table = key_value(&client_stats).codeblock("ansi");
    let api_stats_table = key_value(&api_stats).codeblock("ansi");

    ctx.say(format!(
        "-# Client stats\n{}\n\n-# API stats\n{}",
        client_stats_table, api_stats_table
    ))
    .await?;

    Ok(())
}

async fn get_client_stats(ctx: AkashiContext<'_>) -> (u64, f32, usize) {
    let mut system = ctx.data().sysinfo.lock().await;

    system.refresh_all();

    let pid = sysinfo::get_current_pid()
        .map_err(AkashiError::from)
        .unwrap();
    let process = system
        .process(pid)
        .ok_or_else(|| AkashiError::from("idk"))
        .unwrap();

    let used_memory = process.memory() / 1024 / 1024;
    let cpu_usage = process.cpu_usage();
    let guild_count = 0;

    (used_memory, cpu_usage, guild_count)
}

async fn get_api_stats(ctx: AkashiContext<'_>) -> Result<(f32, u64, u128, String), AkashiError> {
    let data = ctx.data();
    let reqwest = data.reqwest.clone();
    let api_url = data.api.clone();

    let res = reqwest.get(format!("{api_url}/health")).send().await;

    match res {
        Ok(r) => {
            let json = r.json::<HealthResponse>().await.unwrap();

            Ok((json.cpu, json.memory, json.uptime, json.version))
        }
        Err(_) => Err(AkashiError::from(
            "An error occurred while getting API stats, seems to be down",
        )),
    }
}
