use serenity::model::prelude::application_command::{
    ApplicationCommandInteractionDataOption, ApplicationCommandInteractionDataOptionValue,
    ApplicationCommandOption,
};
use std::collections::HashMap;

#[derive(Debug)]
pub enum Error {
    SerenityError(serenity::Error),
    Other(String),
}

impl From<serenity::Error> for Error {
    fn from(err: serenity::Error) -> Self {
        Error::SerenityError(err)
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::Other(err)
    }
}

impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Error::Other(err.to_string())
    }
}

pub struct CommandOptionParser<'a> {
    options: &'a [ApplicationCommandInteractionDataOption],
    map: HashMap<&'a str, &'a ApplicationCommandInteractionDataOption>,
}

impl<'a> CommandOptionParser<'a> {
    pub fn new(options: &'a [ApplicationCommandInteractionDataOption]) -> Self {
        let map = options
            .iter()
            .map(|option| (option.name.as_ref(), option))
            .collect();
        CommandOptionParser { options, map }
    }

    pub fn get_resolved(&self, key: &str) -> Option<&ApplicationCommandInteractionDataOptionValue> {
        self.map.get(key).and_then(|&value| value.resolved.as_ref())
    }

    pub fn get_integer(&self, key: &str) -> Option<&i64> {
        self.get_resolved(key).and_then(|value| match value {
            ApplicationCommandInteractionDataOptionValue::Integer(value) => Some(value),
            _ => None,
        })
    }
    pub fn get_number(&self, key: &str) -> Option<&f64> {
        self.get_resolved(key).and_then(|value| match value {
            ApplicationCommandInteractionDataOptionValue::Number(value) => Some(value),
            _ => None,
        })
    }
    pub fn get_string(&self, key: &str) -> Option<&String> {
        self.get_resolved(key).and_then(|value| match value {
            ApplicationCommandInteractionDataOptionValue::String(value) => Some(value),
            _ => None,
        })
    }
}
