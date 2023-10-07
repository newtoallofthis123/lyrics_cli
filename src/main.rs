#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]

use human_panic::setup_panic;

mod cli;
mod web;

#[tokio::main]
async fn main() {
    setup_panic!();
    cli::splash_screen();
    let (input, cmd) = cli::parse_args();
    if cmd == "-h" || cmd == "--help" {
        cli::print_help();
    } else if cmd == "-v" || cmd == "--version" {
        cli::print_version();
    } else {
        let (url, lyrics) = web::get_lyrics(input.as_str()).await;
        bunt::println!("\n--------------------------");
        bunt::println!("{$yellow}{}{/$}", input);
        bunt::println!("--------------------------");
        println!("{lyrics}");
        bunt::println!("\n{$green}Lyrics from: {/$}{}", url);
    }
}
