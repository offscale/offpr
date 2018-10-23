extern crate github_rs;
extern crate serde_json;

extern crate structopt;

use std::error::Error;
/*use github_rs::client::{Executor, Github};
use serde_json::Value;*/

use structopt::StructOpt;

fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<Error>>
where
    T: std::str::FromStr,
    T::Err: Error + 'static,
    U: std::str::FromStr,
    U::Err: Error + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{}`", s))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    // The number of occurences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: u8,

    #[structopt(short = "D", parse(try_from_str = "parse_key_val"))]
    defines: Vec<(String, String)>,
}

#[derive(Debug)]
struct ConfigItem {
    route: String,
    github_access_token: String,
    gitlab_access_token: String,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
    
    let config_items: Vec<ConfigItem> =  opt.defines.chunks(3)
       .filter(|chunk|chunk.len()==3)
       .map(|chunk|ConfigItem{
            route: chunk[0].1.clone(),
            github_access_token: chunk[1].1.clone(),
            gitlab_access_token: chunk[2].1.clone()})
        .collect();

    println!("config_items = {:?}", config_items);

    /*
    let client = Github::new(opt.token).unwrap();
    let me = client.get()
                   .user()
                   .execute::<Value>();
    match me {
        Ok((headers, status, json)) => {
            println!("{}", headers);
            println!("{}", status);
            if let Some(json) = json{
                println!("{}", json);
            }
        },
        Err(e) => println!("{}", e)
    }
    */
}
