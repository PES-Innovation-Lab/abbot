use serenity::all::{Context, Message};

pub async fn handle_ping(ctx: &Context, msg: &Message) {
    let author = &msg.author.name;
    let message_content = format!("MOSHI MOSH {author}. UwU <3");
    if let Err(why) = msg.channel_id.say(&ctx.http, message_content).await {
        println!("Error sending message for ping: {why:?}");
    }
}