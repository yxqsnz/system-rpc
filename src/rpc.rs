use std::num::NonZeroU32;
use std::time::{Duration, SystemTime};

use tokio::sync::mpsc::Receiver;

use crate::message::{Distro, Message};
use discord::activity::{ActivityBuilder, Assets};
use discord::wheel::{UserState, Wheel};
use discord::Discord;
use discord::DiscordApp;
use discord::Subscriptions;
use discord_sdk as discord;
pub const APP_ID: i64 = 939200723269353552;
pub async fn init(mut events: Receiver<Message>) -> anyhow::Result<()> {
    let (wheel, handler) = Wheel::new(Box::new(|err| tracing::error!(error = ?err, "err")));
    let mut user = wheel.user();
    let client = Discord::new(
        DiscordApp::PlainId(APP_ID),
        Subscriptions::ACTIVITY,
        Box::new(handler),
    )?;
    tracing::info!("waiting for handshake...");
    user.0.changed().await?;
    let user = match &*user.0.borrow() {
        UserState::Connected(user) => {
            tracing::info!(user = ?user, "connected to discord");
            user
        }
        UserState::Disconnected(err) => {
            tracing::error!(error = ?err, "error connecting to discord");
            panic!()
        }
    };
    let mut bat_text = String::from("");
    let mut cpu_text = String::from("0%");
    let mut mem_text = String::from("0%");
    let mut distro = Distro::Unknown;
    while let Some(message) = events.recv().await {
        match message {
            Message::UpdateDistro(d) => distro = d,
            Message::UpdateCPU(usage) => cpu_text = format!("{usage}%"),
            Message::UpdateRAM(usage) => mem_text = format!("{usage}%"),
            Message::UpdateBatText(bat) => bat_text = bat,
            Message::Commit => {
                let texts = [
                    format!("CPU: {cpu_text}"),
                    format!("Memory: {mem_text}"),
                    format!("Battery: {bat_text}"),
                ];
                for text in texts {
                    let acty = ActivityBuilder::default()
                        .details(format!("On {distro:?}"))
                        .state(text)
                        //.party("party#1", NonZeroU32::new(1), NonZeroU32::new(2), discord::activity::PartyPrivacy::Public)
                        .assets(match distro {
                            Distro::Arch => Assets::default().large(
                                String::from("arch"),
                                Some(String::from("I'm Using Arch BTW")),
                            ),
                            Distro::Debian => Assets::default().large(
                                String::from("debian"),
                                Some(String::from("The most stable system"))
                            ),
                            Distro::NixOS => Assets::default().large(
                                 String::from("nix"),
                                 Some(String::from("Operating System for Chads"))
                            ),
                            _ => Assets::default(),
                        })
                        .start_timestamp(SystemTime::now());
                    client.update_activity(acty).await?;
                    tokio::time::sleep(Duration::from_secs(2)).await;
                }
            }
            _ => panic!("unhandled"),
        }
    }
    Ok(())
}
