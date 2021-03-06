extern crate proxy_config;
extern crate url;

use proxy_config::*;
use url::Url;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() == 0 {
        match get_proxies() {
            Ok(proxies) => {
                for proxy in proxies {
                    println!("{}", proxy);
                }
            },
            Err(e) => {
                 println!("Error getting proxies: {}", e);
                 process::exit(1);
            },
        };
    } else {
        for arg in args {
            match get_proxy_for_url(Url::parse(&arg).unwrap()) {
                Ok(proxy) => println!("{} : {}", arg, proxy),
                Err(e) => {
                    println!("Error getting proxy for URL '{}': {}", arg, e);
                    process::exit(1);
                },
            }
        }
    }
}


