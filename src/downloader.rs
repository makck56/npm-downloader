/*
 * @Description: description
 * @Date: 2022-05-23 09:44:53
 * @LastEditors: maicq
 * @LastEditTime: 2022-05-30 14:47:42
 */
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
use std::fs::create_dir;
use std::fs::read_dir;
use crate::PackageInfo;
use std::path::Path;

use std::{
    fs::File,
    io::{Read, Write},
};

pub fn create_download_dir(){
    match read_dir("download"){
        Err(_) => {
            match create_dir("download") {
                Err(why) => panic!("couldn't create {}", why),
                Ok(()) => println!("create download dir success"),
            }
        },
        Ok(_) => println!("create download dir success")
    }
}

#[allow(dead_code)]
#[tokio::main]
pub async fn request_tarball(name: &str) -> Result<String> {
    println!("{}", name);
    let resp = reqwest::get(name).await?.text().await?;
    // println!("{:#?}", resp);
    let obj = json::parse(&resp).unwrap();
    if let json::JsonValue::Object(dist) = &obj["dist"] {
        if let json::JsonValue::String(tarball) = &dist["tarball"] {
            println!("tarball:{}", tarball);
            let client = reqwest::Client::new();
            let body = client.get(tarball).send().await?.bytes().await?;
            let path = Path::new(tarball.rsplit_once("/").unwrap().1);
            let mut file = match File::create(&path) {
                Err(why) => panic!("couldn't create {}", why),
                Ok(file) => file,
            };
            let content = body.bytes();
            let data: std::result::Result<Vec<_>, _> = content.collect();
            file.write_all(&data.unwrap())?;
        }
    }
    Ok("1".into())
}

#[tokio::main]
pub async fn download(package_info:PackageInfo) -> Result<()> {
    let client = reqwest::Client::new();
    let body = client.get(package_info.url).send().await?.bytes().await?;
    let file_path = "download/".to_string() + &package_info.file_name;
    let path = Path::new(&file_path);
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };
    let content = body.bytes();
    let data: std::result::Result<Vec<_>, _> = content.collect();
    file.write_all(&data.unwrap())?;
    Ok(())
}