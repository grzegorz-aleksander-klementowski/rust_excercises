// src/cli.rs

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "obliczanie-kosztów-cz-rowerowych",
    author = "Grzegorz Aleksander Klementowski <grzegorz.aleksander@klementowski.pl>",
    about = "The app calculate the differnecets between the costs of the sets of ebike conversion. "
)]
pub struct Cli {
    //nowy zakup (zazwyczaj się wpisuje)
    #[arg(short, long, default_value_t = 1190)]
    pub koło_z_silnikiem_nowy: i32,
    #[arg(short, long, default_value_t = 1390)]
    pub bateria_nowa: i32,
    #[arg(short, long, default_value_t = 199)]
    pub ładowarka_nowa: i32,
    #[arg(short, long, default_value_t = 0)]
    pub podstawa_do_baterii_nowa: i32,
    #[arg(short, long, default_value_t = 219)]
    pub adapter_nowa: i32,

    // porzedni zakup (zazwyczaj się nie wpisuje)
    #[arg(short, long, default_value_t = 1190)]
    pub koło_z_silnikiem_poprzedni: i32,
    #[arg(short, long, default_value_t = 1198)]
    pub bateria_poprzedni: i32,
    #[arg(short, long, default_value_t = 129)]
    pub ładowarka_poprzedni: i32,
    #[arg(short, long, default_value_t = 125)]
    pub podstawa_do_baterii_poprzedni: i32,
    #[arg(short, long, default_value_t = 119)]
    pub adapter_poprzedni: i32,
}
