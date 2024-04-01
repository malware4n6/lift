use clap::Parser;
use lift::file_details::file_details::FileDetails;
use std::fs;

#[derive(Parser)]
#[command(author = None, version = None, about = None, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    recursive: bool,

    #[arg(short, long, default_value_t = false)]
    shortname: bool,

    #[arg(num_args(0..))]
    paths: Option<Vec<String>>,
}

pub fn main() {
    // let args: Vec<String> = env::args().skip(1).collect();
    let args = Args::parse();
    let fd = FileDetails::new();
    if let Some(paths) = args.paths {
        for p in paths.iter() {
            fd.show(p, args.recursive, args.shortname);
        }
    } else {
        let mut paths: Vec<_> = fs::read_dir("./").unwrap().map(|r| r.unwrap()).collect();
        paths.sort_by_key(|entry| entry.path());
        for path in paths {
            let p = format!("{}", path.path().display());
            fd.show(&p, args.recursive, args.shortname);
        }
    }
}
