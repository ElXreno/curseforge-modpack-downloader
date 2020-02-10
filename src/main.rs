#[macro_use]
extern crate clap;
extern crate cloudflare_bypasser;
extern crate reqwest;

use clap::{App, AppSettings, Arg};
use reqwest::header;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::with_name("url")
                .help("URL of modpack")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("destination file")
                .help("Destination file")
                .index(2)
                .required(true),
        )
        .get_matches();
    
    let url = matches.value_of("url").unwrap();
    let destination_file = matches.value_of("destination file").unwrap();
    
    println!("URL: {} | Destination file: {}", url, destination_file);
    
    let mut bypasser = cloudflare_bypasser::Bypasser::new()
            .retry(30)
            .random_user_agent(true)
            .wait(5);
    
    let client = {
        let headers = {
            let (cookie, user_agent);
            loop {
                if let Ok((c, ua)) = bypasser.bypass(url) {
                    cookie = c;
                    user_agent = ua;
                    break;
                }
            }
            
            let mut h = reqwest::header::HeaderMap::new();
            h.insert(reqwest::header::COOKIE, header::HeaderValue::from_str(cookie.to_str()?)?);
            h.insert(reqwest::header::USER_AGENT, header::HeaderValue::from_str(user_agent.to_str()?)?);
            h
        };
        
        reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()?
    };
    
    let text = client.get(url)
                .send()?
                .text()?;

    println!("{}", text);
    
    Ok(())
}
