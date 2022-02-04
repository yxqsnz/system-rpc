#[derive(Clone, Copy, Debug)]
pub enum Distro {
    Arch,
    Debian,
    NixOS,
    Unknown,
}
impl From<String> for Distro {
    fn from(s: String) -> Self {
        match s
            .to_lowercase()
            .replace('"', "")
            .replace(" ", "_")
            .replace('\0', "")
            .trim()
        {
            "arch_linux" => Self::Arch,
            "arch" => Self::Arch,
            "archlinux" => Self::Arch,
            "debian" => Self::Debian,
            "nixos" => Self::NixOS,
            distro => {
                tracing::warn!("found a unknown distro: {distro}");
                Self::Unknown
            }
        }
    }
}
#[derive(Clone, Debug)]
pub enum Message {
    UpdateCPU(u8),
    UpdateRAM(usize),
    UpdateBatText(String),
    UpdateDistro(Distro),
    Commit,
}
