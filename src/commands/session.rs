use crate::session::SessionManager;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::interaction::application_command::ApplicationCommandInteraction,
    prelude::Context,
};

pub fn register() -> CreateApplicationCommand {
    let mut command = CreateApplicationCommand::default();
    command
        .name("mathbot")
        .description("Math bot commands")
        .create_option(|option| {
            option
                .name("action")
                .description("What action to perform")
                .kind(serenity::model::prelude::command::CommandOptionType::String)
                .required(true)
                .add_string_choice("Start Session", "start")
                .add_string_choice("End Session", "end")
                .add_string_choice("Session Info", "info")
        });
    
    command
}

pub async fn run(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
    session_manager: &SessionManager,
) -> String {
    let action = command
        .data
        .options
        .get(0)
        .and_then(|option| option.value.as_ref())
        .and_then(|value| value.as_str())
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