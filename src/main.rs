/*
 * @Description: description
 * @Date: 2022-05-21 10:28:47
 * @LastEditors: maicq
 * @LastEditTime: 2022-05-30 15:08:00
 */

mod downloader;
mod parser;
use clap::Parser;
use crate::parser::PackageInfo;
use std::thread;
use crate::parser::NpmParser;
use crate::parser::PnpmParser;
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    file: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 10)]
    thread: usize,
}
fn main() {
    downloader::create_download_dir();
    let args = Args::parse();
    let file = args.file;
    let pnpm_parser = PnpmParser {
        file_path: "1".to_string(),
    };
    let mut packages = pnpm_parser.parse(&file);
    
    let mut handles = vec![];
    let len = packages.len();
    let split_size = args.thread;
    let avg = len/split_size;
    for i in 0..split_size {
        let size = if i == split_size - 1 { avg + (len % split_size) } else { avg };
        let mut split_packs:Vec<PackageInfo> = packages.splice(..size,[]).collect();
        let handle = thread::spawn(move || {
            loop{
                if split_packs.len() == 0{
                    return
                }
                let package_info = split_packs.pop().unwrap();
                println!("[thread:{}]{}/{}  {}",i,split_packs.len(),size,package_info.file_name);
                downloader::download(package_info);

            }
        });
        handles.push(handle);
    }
    for handle in handles{
        handle.join();
    }

    println!("{}",packages.len())


}
