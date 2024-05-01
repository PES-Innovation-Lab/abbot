use serenity::all::{Context, Message};

pub async fn send_message(msg: &Message, ctx: &Context, body: &String) {
    if let Err(why) = msg.channel_id.say(&ctx.http, body).await {
        println!("Error sending message: {why:?}");
    }
}