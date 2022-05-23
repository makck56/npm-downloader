/*
 * @Description: description
 * @Date: 2022-05-21 10:28:47
 * @LastEditors: maicq
 * @LastEditTime: 2022-05-23 17:42:55
 */


mod dep_parser;
mod downloader;
use std::fs::create_dir;

fn main() {
    match create_dir("download"){
        Err(why) => panic!("couldn't create {}", why),
        Ok(()) => println!("create download dir success"),
    }
    

   let packages = dep_parser::parse(&"pnpm-lock.yaml");
    for uri in 0..packages.len() {
        println!("{:?}",packages[uri]);
        let r = downloader::download(&packages[uri]);
        println!("{:?}",r);
    } 
}
