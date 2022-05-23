/*
 * @Description: description
 * @Date: 2022-05-23 09:44:53
 * @LastEditors: maicq
 * @LastEditTime: 2022-05-23 17:02:32
 */
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
use std::path::Path;

use std::{
    fs::File,
    io::{Read, Write},
};

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
pub async fn download(url: &str) -> Result<()> {
    let client = reqwest::Client::new();
    let body = client.get(url).send().await?.bytes().await?;
    let file_path = "download/".to_string() + url.rsplit_once("/").unwrap().1.into();
    let path = Path::new(&file_path);
    println!("path:{:?}",path);
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };
    let content = body.bytes();
    let data: std::result::Result<Vec<_>, _> = content.collect();
    file.write_all(&data.unwrap())?;
    Ok(())
}