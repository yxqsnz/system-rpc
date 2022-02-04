use anyhow::Result;

use tokio::{fs::File, io::AsyncReadExt};

use crate::message::Distro;

pub async fn get_current_system() -> Result<Distro> {
    let mut f = File::open("/etc/os-release").await?;
    let mut buf = String::new();
    f.read_to_string(&mut buf).await?;
    if let Some(name) = buf.split('\n').next() {
        if let Some((name, distro)) = name.split_once("=") {
            let distro = distro.split(" ").next().unwrap_or_default();
            tracing::debug!("found distro in {name}: {distro}");
            buf = distro.to_string();
        }
    }

    Ok(Distro::from(buf))
}
pub async fn get_bat_usage() -> Result<u8> {
    let mut f = File::open("/sys/class/power_supply/BAT0/capacity").await?;
    let mut buf = String::new();
    f.read_to_string(&mut buf).await?;
    Ok(buf.trim().parse()?)
}
