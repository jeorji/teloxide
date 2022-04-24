// This example demonstrates how to deal with messages and callback queries
// within a single dialogue.
//
// # Example
// ```
// - /start
// - Let's start! What's your full name?
// - John Doe
// - Select a product:
//   [Apple, Banana, Orange, Potato]
// - <A user selects "Banana">
// - John Doe, product 'Banana' has been purchased successfully!
// ```

use teloxide::{
    dispatching::dialogue::{self, InMemStorage},
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    utils::command::BotCommands,
};

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone)]
pub enum State {
    Start,
    ReceiveFullName,
    ReceiveProductChoice { full_name: String },
}

impl Default for State {
    fn default() -> Self {
        Self::Start
    }
}

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "start the purchase procedure.")]
    Start,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting dialogue_bot...");

    let bot = Bot::from_env().auto_send();

    Dispatcher::builder(
        bot,
        dialogue::enter::<Update, InMemStorage<State>, State, _>()
            .branch(
                Update::filter_message()
                    .branch(
                        teloxide::handler![State::Start]
                            .filter_command::<Command>()
                            .endpoint(handle_command),
                    )
                    .branch(teloxide::handler![State::ReceiveFullName].endpoint(receive_full_name))
                    .branch(dptree::endpoint(invalid_state)),
            )
            .branch(
                Update::filter_callback_query().chain(
                    teloxide::handler![State::ReceiveProductChoice { full_name }]
                        .endpoint(receive_product_selection),
                ),
            ),
    )
    .dependencies(dptree::deps![InMemStorage::<State>::new()])
    .build()
    .setup_ctrlc_handler()
    .dispatch()
    .await;
}

async fn handle_command(
    bot: AutoSend<Bot>,
    msg: Message,
    cmd: Command,
    dialogue: MyDialogue,
) -> HandlerResult {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
        }
        Command::Start => {
            bot.send_message(msg.chat.id, "Let's start! What's your full name?").await?;
            dialogue.update(State::ReceiveFullName).await?;
        }
    }

    Ok(())
}

async fn receive_full_name(
    bot: AutoSend<Bot>,
    msg: Message,
    dialogue: MyDialogue,
) -> HandlerResult {
    match msg.text().map(ToOwned::to_owned) {
        Some(full_name) => {
            let products = InlineKeyboardMarkup::default().append_row(
                vec!["Apple", "Banana", "Orange", "Potato"].into_iter().map(|product| {
                    InlineKeyboardButton::callback(product.to_owned(), product.to_owned())
                }),
            );

            bot.send_message(msg.chat.id, "Select a product:").reply_markup(products).await?;
            dialogue.update(State::ReceiveProductChoice { full_name }).await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Please, send me your full name.").await?;
        }
    }

    Ok(())
}

async fn receive_product_selection(
    bot: AutoSend<Bot>,
    q: CallbackQuery,
    dialogue: MyDialogue,
    full_name: String,
) -> HandlerResult {
    if let Some(product) = &q.data {
        bot.send_message(
            dialogue.chat_id(),
            format!("{full_name}, product '{product}' has been purchased successfully!"),
        )
        .await?;
        dialogue.exit().await?;
    }

    Ok(())
}

async fn invalid_state(bot: AutoSend<Bot>, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Unable to handle the message. Type /help to see the usage.")
        .await?;
    Ok(())
}
