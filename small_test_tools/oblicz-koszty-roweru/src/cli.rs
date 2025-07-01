// src/cli.rs

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "obliczanie-kosztów-cz-rowerowych",
    author = "Grzegorz Aleksander Klementowski <grzegorz.aleksander@klementowski.pl>",
    about = "The app calculate the differnecets between the costs of the sets of ebike conversion. "
)]
pub struct Cli {
    #[arg(short, long)]
    pub name: String,

    #[arg(short, long, default_value_t = 1)]
    pub times: u8,
}
