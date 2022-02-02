use teloxide::prelude::*;
// lib.rs
use bot::morphology;

#[tokio::main]
async fn main() {
    run().await;
}
   
pub async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting japanese morphology bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::repl(bot, |message| async move {
        let phrase = message.update.text().unwrap().to_string();

        match morphology::get_parts(phrase).await {
            Ok(result) => {
                message.answer(format!("{:#?}", result)).await?;
            },
            Err(err) => {
                log::error!("Error: {err}");
            }
        }
        respond(())
    })
    .await;
}

