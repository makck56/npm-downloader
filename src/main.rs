/*
 * @Description: description
 * @Date: 2022-05-21 10:28:47
 * @LastEditors: maicq
 * @LastEditTime: 2022-05-23 17:42:55
 */

mod downloader;
mod parser;
use crate::parser::NpmParser;
use crate::parser::PnpmParser;
use std::fs::create_dir;

fn main() {
    match create_dir("download") {
        Err(why) => panic!("couldn't create {}", why),
        Ok(()) => println!("create download dir success"),
    }

    let pnpm_parser = PnpmParser {
        file_path: "1".to_string(),
    };
    let packages = pnpm_parser.parse(&"pnpm-lock.yaml");
    for uri in 0..packages.len() {
        println!("{:?}", packages[uri]);
        let r = downloader::download(&packages[uri]);
        println!("{:?}", r);
    }
}
