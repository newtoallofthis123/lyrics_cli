use inquire::Text;

pub fn get_input(msg: &str, help: Option<&str>, default: Option<&str>)->String{
    let input = Text::new(msg)
        .with_help_message(
            help.unwrap_or(""),
        )
        .with_default(
            default.unwrap_or(""),
        )
        .prompt();
    let input = input.unwrap();
    input
}

pub fn parse_args() -> (String, String) {
    let args: Vec<String> = std::env::args().collect();
    let mut input = String::new();
    if args.len() > 1 {
        for i in 1..args.len() {
            input.push_str(&args[i]);
            input.push_str(" ");
        }
        (input, args[1].clone())
    }
    else{
        let output = get_input("Get any song's lyrics: ".into(), "Enter the song name: ".into(), "Never Gonna Give You Up".into());
        (output, "lyrics".into())
    }
}

pub fn print_help(){
    bunt::println!("Please provide a song name, no need to put {$underline}quotes{/$} around it ;)");
    bunt::println!("{$blue}{$underline}Usage:{/$}{/$} ");
    bunt::println!("lyrics {$yellow}<Song Name>{/$}");
    bunt::println!("lyrics --help / -h");
    bunt::println!("lyrics --version / -v");
}

pub fn print_version(){
    bunt::println!("lyrics {$yellow}v0.1.0{/$}");
}

pub fn splash_screen(){
bunt::println!("{$green} _                _          {/$}");
bunt::println!("{$red}| |              (_)         {/$}");
bunt::println!("{$blue}| |    _   _ _ __ _  ___ ___ {/$}");
bunt::println!("{$yellow}| |   | | | | '__| |/ __/ __|{/$}");
bunt::println!("{$yellow}| |___| |_| | |  | | (__\\__ \\{/$}");
bunt::println!("{$blue}\\_____/\\__, |_|  |_|\\___|___/{/$}");
bunt::println!("{$red}        __/ |                {/$}");
bunt::println!("{$green}       |___/                 {/$}Finder {$underline}v.0.1{/$}");
}