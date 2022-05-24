use crate::command::CommandDef;
use crate::utils::{CommandOptionParser, Error};
use serenity::builder::CreateApplicationCommandOption;
use serenity::model::interactions::application_command::ApplicationCommandOptionType;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::InteractionResponseType::ChannelMessageWithSource;
use serenity::prelude::Context;
use std::time::Duration;
use tokio::time::sleep;

const NAME: &str = "start";

pub struct StartCommand;

#[serenity::async_trait]
impl CommandDef for StartCommand {
    fn name(&self) -> &'static str {
        NAME
    }

    fn option<'a>(
        &self,
        option: &'a mut CreateApplicationCommandOption,
    ) -> &'a mut CreateApplicationCommandOption {
        option
            .name(NAME)
            .description("タイマーを開始します")
            .kind(ApplicationCommandOptionType::SubCommand)
            .add_sub_option({
                let mut option = CreateApplicationCommandOption::default();
                option
                    .name("duration")
                    .description("タイマーの長さを分で指定します。指定がなければ30分。")
                    .kind(ApplicationCommandOptionType::Integer)
                    .min_int_value(0);
                option
            })
    }

    async fn handler(
        &self,
        ctx: &Context,
        interaction: ApplicationCommandInteraction,
    ) -> Result<(), Error> {
        let option = CommandOptionParser::new(&interaction.data.options);
        let duration = *option.get_integer("duration").unwrap_or(&30) as u64;
        interaction
            .create_interaction_response(ctx, |response| {
                response
                    .kind(ChannelMessageWithSource)
                    .interaction_response_data(|data| data.content("もくもく開始"))
            })
            .await?;
        sleep(Duration::from_secs(duration*60)).await;
        interaction
            .channel_id
            .send_message(ctx, |m| m.content("もくもく終了です"))
            .await?;
        Ok(())
    }
}
