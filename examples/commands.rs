use carapax::prelude::*;
use dotenv::dotenv;
use env_logger;
use futures::Future;
use log;
use std::env;

fn handle_start(context: &Context<Api>, message: &Message, _: Vec<String>) -> HandlerFuture {
    log::info!("handle /start command\n");
    let chat_id = message.get_chat_id();
    let method = SendMessage::new(chat_id, "Hello!");
    let api = context.get();
    HandlerFuture::new(api.execute(&method).then(|x| {
        log::info!("sendMessage result: {:?}\n", x);
        Ok(())
    }))
}

fn handle_user_id(context: &Context<Api>, message: &Message, _: Vec<String>) -> HandlerFuture {
    log::info!("handle /user_id command\n");
    let chat_id = message.get_chat_id();
    let method = SendMessage::new(chat_id, format!("Your ID is: {:?}", message.get_user().map(|u| u.id)));
    let api = context.get();
    HandlerFuture::new(api.execute(&method).then(|x| {
        log::info!("sendMessage result: {:?}\n", x);
        Ok(())
    }))
}

fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("CARAPAX_TOKEN").expect("CARAPAX_TOKEN is not set");
    let proxy = env::var("CARAPAX_PROXY").ok();

    let api = Api::new(token).unwrap();
    let app = App::new(api.clone(), api);

    app.add_handler(Handler::message(
        CommandsHandler::default()
            .add_handler("/start", handle_start)
            .add_handler("/user_id", handle_user_id),
    ))
    .run(RunMethod::poll(Default::default()));
}
