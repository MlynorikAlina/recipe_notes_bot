use tokio;

use recipes_saver_bot::{run_bot, run_server};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    log::info!("starting recipe notes bot...");

    tokio::select! {
        _ = run_bot() => {
            eprintln!("Some error in bot");
        }

        _ = run_server() => {
            eprintln!("Some error in server");
        }
    }

    
}
