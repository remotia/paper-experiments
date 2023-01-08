use std::collections::HashMap;

use clap::Parser;
use serde::Deserialize;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub(crate) struct CommandLineArgs {
    #[arg(short, long)]
    pub config_file_path: String,
}

#[derive(Deserialize, Clone)]
pub struct Configuration {
    pub video_file_path: String,
    pub width: u32,
    pub height: u32,
    pub extraction_tick: u64,
    pub encoder_options: HashMap<String, String>,
    pub transmission: TransmissionConfiguration,
}

#[derive(Deserialize, Clone)]
pub struct TransmissionConfiguration {
    pub server_address: String,
    pub server_port: u16,
    pub latency: u64,
    pub max_frame_delay: u64,
}

pub fn load_config() -> Configuration {
    let args = CommandLineArgs::parse();

    toml::from_str(&std::fs::read_to_string(args.config_file_path).unwrap()).unwrap()
}
