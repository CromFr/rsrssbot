extern crate serde;
extern crate serde_yaml;

trait Frontend {
    fn init(config: &serde_yaml::Value);
}


pub struct FrontendStdOut {}

impl Frontend for FrontendStdOut {
    #[no_mangle]
    fn init(config: &serde_yaml::Value) {
        println!("HERE COME THE CAT");
    }
}