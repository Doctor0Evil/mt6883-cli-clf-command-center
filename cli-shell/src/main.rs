mod prompt;
mod repl;

#[tokio::main]
async fn main() {
    repl::run_repl().await;
}
