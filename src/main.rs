mod command;
mod constants;
mod modules;
mod helper;

#[tokio::main]
async fn main() {
    command::handle_command().await;
}
