use std::env;

use serenity::{all::{Context, EventHandler, GatewayIntents, Message, Ready}, async_trait, Client};

use crate::metrics::metrics::{BOT_SENT_MESSAGES_COUNTER, HUMAN_BOT_REQUEST_MSG_COUNTER, TOTAL_HUMAN_MESSAGES_COUNTER, TOTAL_INCOMING_MESSAGES_COUNTER};

use crate::event_handler::ping_handler;

use super::data_fetch_handler;

pub struct DiscordEventHandler;

#[async_trait]
impl EventHandler for DiscordEventHandler {
    // Set a handler for the `message` event. This is called whenever a new message is received.
    //
    // Event handlers are dispatched through a threadpool, and so multiple events can be
    // dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        TOTAL_INCOMING_MESSAGES_COUNTER.inc();
        if msg.author.bot {
            BOT_SENT_MESSAGES_COUNTER.inc();
        } else {
            TOTAL_HUMAN_MESSAGES_COUNTER.inc();
        }

        let content = &msg.content;
        if content.chars().next().unwrap() == '!' {
            HUMAN_BOT_REQUEST_MSG_COUNTER.inc();
        }

        route_message(ctx, msg).await;
    }

    // Set a handler to be called on the `ready` event. This is called when a shard is booted, and
    // a READY payload is sent by Discord. This payload contains data like the current user's guild
    // Ids, current user data, private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

async fn route_message(ctx: Context, msg: Message) {
    let content = &msg.content;
    if content == "!ping" {
        ping_handler::handle_ping(&ctx, &msg).await;
    }else if content == "!getitall" {
        data_fetch_handler::handle_fetch_all_data(&ctx, &msg).await;
    }
}

pub async fn register_run_discord_client() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    // Create a new instance of the Client, logging in as a bot. This will automatically prepend
    // your bot token with "Bot ", which is a requirement by Discord for bot users.
    let mut client =
        Client::builder(&token, intents).event_handler(DiscordEventHandler).await.expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until
    // it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}