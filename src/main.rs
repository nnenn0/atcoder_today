use clap::Clap;

#[derive(Clap, Debug)]
#[clap(
    name = "Atcoder Today",
    version = "1.0.0",
    author = "nnenn0",
    about = "Make a suggestion for today's contest"
)]

struct Opts {
    #[clap(short, long)]
    reset: bool,

    #[clap(short, long)]
    latest: Option<u32>,

    #[clap(short, long)]
    from_no: Option<u32>,

    #[clap(short, long)]
    to_no: Option<u32>,
}

fn main() {
    let opts = Opts::parse();

    
}
