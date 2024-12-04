mod command;
mod constants;
mod modules;

#[tokio::main]
async fn main() {
    command::handle_command().await;
}
