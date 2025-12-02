use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]

pub struct Cli {
    pub cmd: Option<String>,

    // short command name
    #[arg(short, long)]
    pub name: Option<String>,

    // Comand
    #[arg(short, long)]
    pub command: Option<String>,

    // show all shorts available
    #[arg(short, long)]
    pub show: bool,
}
