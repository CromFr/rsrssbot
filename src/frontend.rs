

extern crate serde_yaml;


trait Frontend {

    fn init(config: &serde_yaml::Value);
}