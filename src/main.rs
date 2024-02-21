use clap::Parser;

#[derive(Parser)]
#[command(version, about = "Example echo in rust", long_about = None)]
struct Cli {
    /// Do not output the trailing newline
    #[arg(short)]
    no_newline: bool,

    /// Enable interpretation of backslash escapes
    #[arg(short = 'e', default_value_t = false)]
    parse_backslash: bool,


    /// Disable interpretation of backslash escapes (default)
    #[arg(short = 'E', default_value_t = true)]
    interpret_backslash: bool,

    #[arg(value_name = "STRING")]
    values: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    for (i, s) in cli.values.iter().enumerate() {
        print!("{}", s);
        if i != cli.values.len() - 1 {
            print!(" ");
        }
    }

    if !cli.no_newline {
        println!("");
    }
}
