use std::env;

use serenity::http::Http;

use serenity::model::id::GuildId;

use pomodoro_timer_discord::{commands, CommandDef, PREFIX};
use serenity::prelude::SerenityError;

#[tokio::main]
async fn main() -> Result<(), SerenityError> {
    let token = env::var("DISCORD_TOKEN").unwrap();
    let http = Http::new(token.as_ref());
    let user = http.get_current_user().await?;
    http.set_application_id(user.id.0);
    let result = GuildId(829909188720853040)
        .create_application_command(http.as_ref(), |command| {
            command
                .name(PREFIX)
                .description("ボモドーロタイマー用コマンド");
            commands().iter().for_each(|&c| {
                command.create_option(|option| c.option(option));
            });
            command
        })
        .await;
    if let Err(why) = result {
        eprintln!("{}", why);
    };
    Ok(())
}
