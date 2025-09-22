use crate::session::SessionManager;
use serenity::{
    all::{
        CommandInteraction, CommandOptionType, CreateCommand, CreateCommandOption,
    },
    prelude::Context,
};

pub fn register() -> CreateCommand {
    CreateCommand::new("mathbot")
        .description("Math bot commands")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "action",
                "What action to perform",
            )
            .required(true)
            .add_string_choice("Start Session", "start")
            .add_string_choice("End Session", "end")
            .add_string_choice("Session Info", "info"),
        )
}

pub async fn run(
    _ctx: &Context,
    command: &CommandInteraction,
    session_manager: &SessionManager,
) -> String {
    let action = command
        .data
        .options
        .iter()
        .find(|option| option.name == "action")
        .and_then(|option| option.value.as_str())
        .unwrap_or("start");

    let user_id = command.user.id;
    let channel_id = command.channel_id;

    match action {
        "start" => session_manager.create_session(user_id, channel_id),
        "end" => session_manager.end_session(user_id),
        "info" => session_manager.get_session_info(user_id),
        _ => "Unknown action. Available actions: start, end, info".to_string(),
    }
}