mod commands;
mod math;
mod session;
mod utils;

use anyhow::Result;
use serenity::{
    all::{
        Command, CreateInteractionResponse, 
        CreateInteractionResponseMessage, GatewayIntents, 
        Interaction, Message, Ready,
    },
    async_trait,
    prelude::*,
};
use std::env;
use tracing::{error, info};

use crate::session::SessionManager;

struct Handler {
    session_manager: SessionManager,
}

impl Handler {
    fn new() -> Self {
        Self {
            session_manager: SessionManager::new(),
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        // register slash commands globally
        let commands = vec![
            commands::session::register(),
        ];

        for command in commands {
            if let Err(why) = Command::create_global_command(&ctx.http, command)
                .await
            {
                error!("Cannot create slash command: {}", why);
            }
        }

        info!("Slash commands registered successfully!");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let content = match command.data.name.as_str() {
                "mathbot" => {
                    commands::session::run(&ctx, &command, &self.session_manager).await
                }
                _ => "Unknown command".to_string(),
            };

            if let Err(why) = command
                .create_response(&ctx.http, CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new().content(content)
                ))
                .await
            {
                error!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        // handle direct messages in active sessions
        if !msg.author.bot {
            if let Some(response) = self.session_manager.handle_message(&msg).await {
                if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
                    error!("Error sending message: {:?}", why);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // load environment variables
    dotenv::dotenv().ok();

    // get bot token from environment
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a Discord bot token in the environment (DISCORD_TOKEN)");

    // create client
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler::new())
        .await
        .expect("Error creating client");

    info!("Starting math-rs Discord bot...");

    // start the client
    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }

    Ok(())
}