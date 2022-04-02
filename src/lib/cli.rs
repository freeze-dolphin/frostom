use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct LaunchArgs {
    #[clap(short, long, help = "Osu map to load")]
    pub input: String,

    #[clap(long, help = "Print map contents")]
    pub raw: bool,
}
