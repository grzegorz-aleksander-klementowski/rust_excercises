// src/cli.rs

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "obliczanie-kosztów-cz-rowerowych",
    author = "Grzegorz Aleksander Klementowski <grzegorz.aleksander@klementowski.pl>",
    about = "The app calculate the differnecets between the costs of the sets of ebike conversion. "
)]
pub struct Cli {
    #[arg(short, long, default_value_t = 1190)]
    koło_z_silnikiem_nowy: i32,
    #[arg(short, long, default_value_t = 1390)]
    bateria_nowa: i32,
    #[arg(short, long, default_value_t = 199)]
    ładowarka_nowa: i32,
    #[arg(short, long, default_value_t = 0)]
    podstawa_do_baterii_nowa: i32,
    #[arg(short, long, default_value_t = 219)]
    adapter_nowa: i32,

    #[arg(short, long, default_value_t = 1190)]
    koło_z_silnikiem_poprzedni: i32,
    #[arg(short, long, default_value_t = 1198)]
    bateria_poprzedni: i32,
    #[arg(short, long, default_value_t = 129)]
    ładowarka_poprzedni: i32,
    #[arg(short, long, default_value_t = 125)]
    podstawa_do_baterii_poprzedni: i32,
    #[arg(short, long, default_value_t = 119)]
    adapter_poprzedni: i32,
}
