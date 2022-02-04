use crate::message::Message;
use crate::util::{self, get_bat_usage};
use std::time::Duration;
use tokio::process::Command;
use tokio::sync::mpsc::Sender;
use tokio::time;
pub async fn init(sender: Sender<Message>) -> anyhow::Result<()> {
    let distro = util::get_current_system().await?;
    tracing::debug!("current distro: {distro:?}");
    sender.send(Message::UpdateDistro(distro)).await?;
    loop {
        let free_out = Command::new("sh")
            .arg("-c")
            .arg("free | grep Mem | awk '{print $3/$2 * 100.0}'")
            .output()
            .await?
            .stdout;
        let parsed_output = String::from_utf8(free_out)?;
        let parsed_output = parsed_output.trim();
        tracing::debug!(out = ?parsed_output, "free output parsed");
        let percent_used_mem = parsed_output.parse::<f64>()?;

        let top_out = Command::new("sh")
            .arg("-c")
            .arg("top -n 1 | awk '{ print $4 }' | head -n3 | tail -n1")
            .output()
            .await?
            .stdout;
        let parsed_output = String::from_utf8(top_out)?;
        let parsed_output = parsed_output.trim();
        tracing::debug!(out = ?parsed_output, "top output parsed");
        let cpu_usage = parsed_output.parse::<f64>()?;
        tracing::debug!(usage = ?cpu_usage, "cpu usage collection done.");
        sender.send(Message::UpdateCPU(cpu_usage as u8)).await?;
        sender
            .send(Message::UpdateRAM(percent_used_mem as usize))
            .await?;
        let bat_usage = get_bat_usage().await?;
        let bat_text;
        if bat_usage == 69 {
            bat_text = format!("69...nice");
        } else {
            bat_text = bat_usage.to_string();
        }
        sender.send(Message::UpdateBatText(format!("{bat_text}%"))).await?;
        sender.send(Message::Commit).await?;
        time::sleep(Duration::from_secs(5)).await;
    }
}
