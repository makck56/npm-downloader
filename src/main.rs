/*
 * @Description: description
 * @Date: 2022-05-21 10:28:47
 * @LastEditors: maicq
 * @LastEditTime: 2022-05-28 19:30:30
 */

mod downloader;
mod parser;
use core::time::Duration;
use std::sync::Arc;
use std::sync::Mutex;
use std::rc::Rc;
use std::fs::read_dir;
use std::fs::create_dir;
use std::thread;
use crate::parser::NpmParser;
use crate::parser::PnpmParser;

fn main() {
    match read_dir("download"){
        Err(_) => {
            match create_dir("download") {
                Err(why) => panic!("couldn't create {}", why),
                Ok(()) => println!("create download dir success"),
            }
        },
        Ok(_) => println!("create download dir success")
    }
    let pnpm_parser = PnpmParser {
        file_path: "1".to_string(),
    };
    let mut packages = pnpm_parser.parse(&"pnpm-lock.yaml");
    
    let mut handles = vec![];
    let len = 15;
    let avg = packages.len()/len;
    println!("{}",packages.len()); 
    println!("{}",avg); 
    for i in 0..len {
        let size = if i == len -1 { avg + (len%5) }else { avg };
        let mut split_packs:Vec<String> = packages.splice(..size,[]).collect();

        let handle = thread::spawn(move || {
            loop{
                if split_packs.len() == 0{
                    return
                }
                let name = split_packs.pop().unwrap();
                println!("[thread:{}]remain:{}, {}",i,split_packs.len(),name);
                downloader::download(&name);

            }
        });
        handles.push(handle);
    }
    for handle in handles{
        handle.join();
    }

    println!("{}",packages.len())


}
