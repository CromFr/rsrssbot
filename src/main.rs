extern crate rss;
extern crate hyper;
extern crate libloading;
extern crate serde;

mod frontend;

use rss::Channel;
use std::str::FromStr;

use hyper::Client;
// use hyper::client::response;
use std::io::Read;

#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
use serde::{Serialize, Serializer, Deserialize, Deserializer};

use std::time::UNIX_EPOCH;


#[derive(Debug)]
struct SystemTime(std::time::SystemTime);
impl std::ops::Deref for SystemTime {
    type Target = std::time::SystemTime;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Serialize for SystemTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let s = format!("{}", self.duration_since(UNIX_EPOCH).unwrap().as_secs());

        serializer.serialize_str(&s)
    }
}
impl Deserialize for SystemTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer
    {
        let s = String::deserialize(deserializer)?;

        use std::time::Duration;
        Ok(SystemTime(UNIX_EPOCH + Duration::from_secs(u64::from_str(&s).unwrap())))
    }
}
fn poll_period_secs_def() -> u32 {
    60
}
#[derive(Serialize, Deserialize, Debug)]
struct Config {
    frontends: Vec<FrontendConfig>,
    feeds: Vec<FeedConfig>,

    #[serde(default = "poll_period_secs_def")]
    poll_period_secs: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct FrontendConfig {
    driver_path: String,
    config: serde_yaml::Value,
    display_feeds: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct FeedConfig {
    name: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct State {
    #[serde(default)]
    feeds: Vec<FeedState>,
}

#[derive(Serialize, Deserialize, Debug)]
struct FeedState {
    url: String,
    last_sent_date: SystemTime,
}


fn main() {
    println!("===================================================================");

    use std::fs::File;
    let mut config_file = File::open("./rsrssbot.yml").unwrap();
    let config = serde_yaml::from_reader::<_, Config>(config_file).unwrap();

    let state_path = "./rsrssbot.state.yml";
    let mut res = File::open(state_path);
    let mut state_file = if res.is_ok() {
        res.unwrap()
    } else {
        File::create(state_path).unwrap()
    };

    let mut state = serde_yaml::from_reader::<_, State>(state_file).unwrap();

    // println!("{:?}", config);

    use std::hash;
    let lib = libloading::Library::new("target/debug/frontend_stdout.dll").unwrap();

    unsafe {
        let lib_init: libloading::Symbol<fn(config: &serde_yaml::Value)> = lib.get(b"init")
            .unwrap();

        lib_init(&config.frontends[0].config);
    }

    // use frontend;

    // let func: lib::Symbol<unsafe extern "C" fn() -> u32> = try!(lib.get(b"my_func"));
    // unsafe {
    //     let func: lib::Symbol<unsafe extern "C" fn() -> u32> = try!(lib.get(b"my_func"));
    // }

    // loop {
    for feed in config.feeds {
        let client = Client::new();
        let mut res = client.get(&feed.url).send().unwrap();

        if res.status == hyper::Ok {
            let mut buffer = String::new();
            let _ = res.read_to_string(&mut buffer);

            let channel = Channel::from_str(&buffer).unwrap();

            println!("{:?}", channel.title);
            println!("{:?}", channel.description);
            println!("{:?}", channel.pub_date);
            println!("{:?}", channel.last_build_date);
            // println!("{:?}", channel.items);
        } else {
            println!("Could not fetch URL {}", feed.url);
        }
    }
    // }




    // if res.status == hyper::Ok {
    //     // println!("{:?}", res);

    //     let mut buffer = String::new();
    //     let read = res.read_to_string(&mut buffer);


    //     let channel = Channel::from_str(&buffer).unwrap();
    //     // let channel = Channel::read_from(res).unwrap();

    //     println!("{:?}", channel.title);
    //     println!("{:?}", channel.description);
    //     println!("{:?}", channel.pub_date);
    //     println!("{:?}", channel.last_build_date);
    //     // println!("{:?}", channel.items);
    // } else {
    //     println!("Could not fetch URL");
    // }


}