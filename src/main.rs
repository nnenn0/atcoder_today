use clap::Clap;
use rand::Rng;
use serde::{Serialize, Deserialize};
use url::Url;
use std::fs::File;
use std::fs::remove_file;
use std::io::Write;
use std::path::Path;
use std::io::{BufReader, BufWriter};
use url_open::UrlOpen;

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

#[derive(Debug, Serialize, Deserialize)]
struct Setting {
    latest: u32,
    done_contests: Vec<u32>,
}

fn main() {
    let setting_json = "setting.json";
    let opts = Opts::parse();
    if opts.reset {
        if Path::exists(Path::new(setting_json)) {
            remove_file(setting_json).unwrap();
        }
        println!("setting.json is deleted!!");
    }
    if !Path::new(setting_json).exists() {
        match opts.latest {
            Some(latest) => {
                let setting = Setting {
                    latest: latest,
                    done_contests: vec![],
                };
                let file = File::create(setting_json).unwrap();
                let mut writer = BufWriter::new(file);
                writer.write(serde_json::to_string(&setting).unwrap().as_bytes()).unwrap();
                println!("new setting.json!");
            },
            None => panic!("please set latest contests(-l)")
        }
    }
    let file = File::open(setting_json).unwrap();
    let reader = BufReader::new(&file);
    let mut setting: Setting = serde_json::from_reader(reader).unwrap();
    let latest = &setting.latest;
    let done_contests = &setting.done_contests;
    loop {
        let today_contest = rand::thread_rng().gen_range(1..=*latest);
        if done_contests.contains(&today_contest) {
            continue;
        }
        else {
            println!("{}", today_contest);
            setting.done_contests.push(today_contest);
            setting.done_contests.sort();
            let file = File::create(setting_json).unwrap();
            let mut writer = BufWriter::new(&file);
            writer.write(serde_json::to_string(&setting).unwrap().as_bytes()).unwrap();
            println!("{:?}", setting);
            println!("update setting.json!");
            let abc_url = "https://atcoder.jp/contests/abc";
            let abc_tasks = "/tasks";
            let url = format!("{}{}{}", abc_url, today_contest, abc_tasks);
            Url::parse(&url).unwrap().open();
            break;
        }
    }
}
