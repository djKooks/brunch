use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::fmt::Debug;

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub struct RouteConfig {
    pub route: HashMap<String, String>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Config {
    route: Vec<RouteInfo>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct RouteInfo {
    from: String,
    to: String,
    limit: u16,
}

impl RouteConfig {
    pub fn new(_file_path: &str) -> Result<Self, ()> {

        match File::open(_file_path) {
            Ok(file_data) => {
                let mut buf_reader = BufReader::new(file_data);
                let mut contents = String::new();
                
                buf_reader.read_to_string(&mut contents).unwrap();
                let config_data: Config = serde_yaml::from_str(&contents).unwrap();
                // println!("{}, {}, {}",config_data.route[0].from, config_data.route[0].to, config_data.route[0].limit);

                let mut test_map = HashMap::new();
                for route in config_data.route {
                    test_map.insert(route.from, route.to);
                }

                Ok(RouteConfig {
                    route: test_map
                })
            },
            Err(err) => {
                panic!("Fail config parsing!")
            }
        }
    }
}