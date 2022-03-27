use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct LaunchArgs {
    #[clap(short, long, help = "Osu beat-map file to load")]
    pub input: String,
}
