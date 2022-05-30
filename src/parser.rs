/*
 * @Description: description
 * @Date: 2022-05-23 09:30:03
 * @LastEditors: maicq
 * @LastEditTime: 2022-05-30 14:44:48
 */

use std::fs;
use yaml_rust::yaml;
use yaml_rust::YamlLoader;
pub struct PackageInfo{
	pub file_name:String,
	pub url:String,
}
pub trait NpmParser {
	fn parse(&self, file_path: &str) -> Vec<PackageInfo>;
}

pub struct PnpmParser {
	pub file_path: String,
}

impl NpmParser for PnpmParser {
	fn parse(&self, file_path: &str) -> Vec<PackageInfo> {
		let mut result = vec![];
		let file_str = fs::read_to_string(file_path).unwrap();
		let docs = YamlLoader::load_from_str(&file_str).unwrap();
		let doc = &docs[0];
		if let &yaml::Yaml::Hash(map) = &doc {
			let packages = map.get(&yaml::Yaml::String("packages".into()));
			if let Some(v) = packages {
				if let &yaml::Yaml::Hash(dep) = &v {
					let keys = dep.keys();
					for k in keys {
						if let &yaml::Yaml::String(package_names) = &k {
							let rspilt = "/";
							let (registry, full_name) = package_names.split_once("/").unwrap();
							let temp_version = package_names.rsplit_once("/").unwrap().1;
							if registry == "" {
								let (name, version) = if temp_version.contains("_"){
									let (n,v) = full_name.rsplit_once("_").unwrap().0.rsplit_once(rspilt).unwrap();
									(n,v)
								}else{
									
									let (n,v) = full_name.rsplit_once(rspilt).unwrap();
									(n,v)
								};
								let package_name = if name.contains("@") {
									name.split_once("/").unwrap().1
								} else {
									name
								};
								let mut uri = "https://registry.npmjs.org/".to_string();
								uri += name;
								uri += "/-/";
								uri += package_name;
								uri += "-";
								uri += version;
								uri += ".tgz";
								let file_name = name.to_string().replace("/","_") + "_" + version +  ".tgz";
								let package_info = PackageInfo {
									file_name: file_name,
									url: uri
								};
								result.push(package_info);
							} else {
								if let &yaml::Yaml::Hash(pack_info) = &dep.get(k).unwrap() {
									if let &yaml::Yaml::Hash(resolution) = &pack_info
										.get(&yaml::Yaml::String("resolution".into()))
										.unwrap()
									{
										let tarball_option = resolution.get(&yaml::Yaml::String("tarball".into()));
										//TODO other registry download file name
										result.push(PackageInfo {
											file_name: tarball_option.unwrap().as_str().unwrap().into(),
											url:tarball_option.unwrap().as_str().unwrap().into()
										});
									}
								}
							}
							// println!("{:?}", result)
						}
					}
				} else {
					println!("packages node format error!")
				}
			} else {
				println!("can not find packages node")
			}
		}
		return result;
	}
}

// pub fn parse(file_path: &str) -> Vec<String> {
// 	let mut result = vec![];
// 	let file_str = fs::read_to_string(file_path).unwrap();
// 	let docs = YamlLoader::load_from_str(&file_str).unwrap();
// 	let doc = &docs[0];

// 	if let &yaml::Yaml::Hash(map) = &doc {
// 		let packages = map.get(&yaml::Yaml::String("packages".into()));
// 		if let Some(v) = packages {
// 			if let &yaml::Yaml::Hash(dep) = &v {
// 				let keys = dep.keys();
// 				for k in keys {
// 					if let &yaml::Yaml::String(package_names) = &k {
// 						let rspilt = if package_names.contains("_") {
// 							"_"
// 						} else {
// 							"/"
// 						};
// 						let (registry, full_name) = package_names.split_once("/").unwrap();
// 						if registry == "" {
// 							let (name, version) = if rspilt == "/" {
// 								let (i_name, i_version) = full_name.rsplit_once(rspilt).unwrap();
// 								(i_name, i_version)
// 							} else {
// 								let (name, _) = full_name.rsplit_once(rspilt).unwrap();
// 								name.rsplit_once("/").unwrap()
// 							};

// 							let package_name = if name.contains("@") {
// 								name.split_once("/").unwrap().1
// 							} else {
// 								name
// 							};

// 							let mut uri = "https://registry.npmjs.org/".to_string();
// 							uri += name;
// 							uri += "/-/";
// 							uri += package_name;
// 							uri += "-";
// 							uri += version;
// 							uri += ".tgz";
// 							result.push(uri);
// 						} else {
// 							if let &yaml::Yaml::Hash(pack_info) = &dep.get(k).unwrap() {
// 								if let &yaml::Yaml::Hash(resolution) = &pack_info
// 									.get(&yaml::Yaml::String("resolution".into()))
// 									.unwrap()
// 								{
// 									let tarball_option = resolution.get(&yaml::Yaml::String("tarball".into()));
// 									result.push(tarball_option.unwrap().as_str().unwrap().into());
// 								}
// 							}
// 						}

// 						// println!("{:?}", result)
// 					}
// 				}
// 			} else {
// 				println!("packages node format error!")
// 			}
// 		} else {
// 			println!("can not find packages node")
// 		}
// 	}
// 	return result;
// }
