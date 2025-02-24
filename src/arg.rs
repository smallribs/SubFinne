use clap::{Parser, CommandFactory};

/// A SubDomian Finder Tool
#[derive(Parser, Debug)]
#[command(
    name = "SubFinne",
    version = "1.0.0",
    author = "smallribs"
)]
pub struct Args {
    /// Please Input Main Domain
    #[arg(short, long, help = "Please Input Main Domain")]
    pub domian: Option<String>,
    /// Please Input Dictionary File
    #[arg(long, help = "Please Input Dictionary File")]
    pub dict: Option<String>,
    /// Please Input rule
    #[arg(short, long, help = "Please Input rule")]
    pub rule: Option<String>,
    /// Please Input Thread Pool Size
    #[arg(short, long, help = "Please Input Thread Pool Size")]
    pub pool_size: Option<usize>,
}

pub fn parse_args() -> Args {
    if std::env::args().len() == 1 {
        Args::command().print_help().unwrap();
        println!();
        std::process::exit(0);
    }

    Args::parse()
}