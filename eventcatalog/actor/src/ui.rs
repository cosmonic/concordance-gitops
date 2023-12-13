use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "../out"]
pub struct Asset;
