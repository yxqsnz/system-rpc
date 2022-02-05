use anyhow::Result;
use system_rpc::{rpc, updater};
use tokio::sync::mpsc;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
const BUFFER_SIZE: usize = 8;
fn setup() {
    let subscriber = FmtSubscriber::builder()
        .pretty()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Can't Set global subscriber")
}
#[tokio::main]
async fn main() -> Result<()> {
    setup();
    tracing::info!("starting...");
    let (sender, rec) = mpsc::channel(BUFFER_SIZE);
    tokio::spawn(updater::init(sender));
    rpc::init(rec).await?;
    Ok(())
}
