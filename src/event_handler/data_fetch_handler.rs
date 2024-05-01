use serenity::all::{Context, Message};

use crate::{database::connection::CANDIDATE_DB, types::entity::Candidate};

use super::utils::send_message;

pub async fn handle_fetch_all_data(ctx: &Context, msg: &Message) {
    send_message(msg, ctx, &"TAKE IT ALLLL!!!".to_string()).await;
    match CANDIDATE_DB.get().unwrap().get_all_candidates().await {
        Ok(candidates) => 
            async {
                for c in candidates {
                    let string_rep = candidate_to_text(&c);
                    send_message(msg, ctx, &string_rep).await;  
                }
            }.await,
        Err(e) => eprintln!("Error while fetching candidates {}", e)
    }
}

fn candidate_to_text(candidate: &Candidate) -> String {
    format!("{:?}", candidate)
}