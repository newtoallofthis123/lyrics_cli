use human_panic::setup_panic;

mod cli;
mod spinner;
mod web;
mod scraper;

// use spinner::wrap_spinner;

#[tokio::main]
async fn main() {
    setup_panic!();
    cli::splash_screen();
    let x = scraper::get_search_options("Falling in love with you").await;
    println!("{:?}", x);
    /* let (input, cmd) = cli::parse_args();
    if cmd == "-h" || cmd == "--help" {
        cli::print_help();
    } else if cmd == "-v" || cmd == "--version" {
        cli::print_version();
    } else {
        let (url, lyrics) = wrap_spinner(web::get_lyrics(input.as_str())).await.unwrap();
        bunt::println!("\n--------------------------");
        bunt::println!("{$yellow}{}{/$}", input);
        bunt::println!("--------------------------");
        println!("{lyrics}");
        bunt::println!("\n{$green}Lyrics from: {/$}{}", url);
    } */
}
