use clap::Parser;
use serde::Deserialize;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub(crate) struct CommandLineArgs {
    #[arg(short, long)]
    pub config_file_path: String,
}

#[derive(Deserialize)]
pub struct Configuration {
    pub video_file_path: String,
    pub width: u32,
    pub height: u32,
}

pub fn load_config() -> Configuration {
    let args = CommandLineArgs::parse();

    toml::from_str(&std::fs::read_to_string(args.config_file_path).unwrap()).unwrap()
}
