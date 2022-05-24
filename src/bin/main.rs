use pomodoro_timer_discord::{commands, CommandDef, PREFIX};
use serenity::client::{Context, EventHandler};
use serenity::model::interactions::Interaction;
use serenity::prelude::GatewayIntents;
use serenity::Client;
use std::env;

use serenity::model::prelude::application_command::ApplicationCommandOptionType;

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, context: Context, interaction: Interaction) {
        if let Some(interaction) = interaction.clone().application_command() {
            let data = &interaction.data;
            if data.name != PREFIX {
                return;
            }
            let subcommand = data
                .options
                .get(0)
                .filter(|option| option.kind == ApplicationCommandOptionType::SubCommand)
                .and_then(|option| {
                    commands()
                        .iter()
                        .cloned()
                        .find(|&command| option.name == command.name())
                });
            if let Some(command) = subcommand {
                command
                    .handler(&context, interaction)
                    .await
                    .map_err(|why| {
                        eprintln!("{:?}", why);
                    })
                    .ok();
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").unwrap();
    let intents = GatewayIntents::non_privileged();
    let mut client = Client::builder(token, intents)
        .event_handler(Handler {})
        .await
        .unwrap();
    if let Err(why) = client.start().await {
        eprintln!("{}", why);
    }
}
