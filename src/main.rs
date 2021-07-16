#![allow(unused)]

use std::time;
use std::thread;

use structopt::StructOpt;
use anyhow::{Context, Result};
use indicatif::{ProgressBar};

#[derive(StructOpt)]
struct Cli {
    // pattern to look for
    pattern: String,
    // the path to the file(s) to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}



fn main() {
    let content = std::fs::read_dir("./testfiles").expect("no such dir");

    let ten_mil = time::Duration::from_millis(1000);

    let pb = ProgressBar::new(100);

    for i in content {
        pb.println(format!("[+] parsing file...#{:?}", &i));
        thread::sleep(ten_mil);
        pb.inc(1);
    }
    pb.finish_with_message("done");
}
