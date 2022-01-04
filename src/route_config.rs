use std::collections::HashMap;

pub struct RouteConfig {
    pub route: HashMap<String, String>
}

impl RouteConfig {
    pub fn new(_file: &str) -> Self {
        let mut test_map = HashMap::new();
        test_map.insert("/v2".to_owned(), "/proxy".to_owned());
        RouteConfig {
            route: test_map
        }
    }
}