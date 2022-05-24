use crate::command::start::StartCommand;
use serenity::builder::CreateApplicationCommandOption;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;

use crate::utils::Error;

mod start;

pub const PREFIX: &str = "timer";

#[serenity::async_trait]
pub trait CommandDef: Sized {
    fn name(&self) -> &'static str;
    fn option<'a>(
        &self,
        option: &'a mut CreateApplicationCommandOption,
    ) -> &'a mut CreateApplicationCommandOption;
    async fn handler(
        &self,
        ctx: &Context,
        interaction: ApplicationCommandInteraction,
    ) -> Result<(), Error>;
}

pub fn commands() -> &'static [&'static impl CommandDef] {
    &[&StartCommand]
}
