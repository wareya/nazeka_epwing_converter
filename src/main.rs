#![allow(clippy::suspicious_else_formatting)]

use std::fs::File;
use std::io::Read;
use std::env;

mod epwing;
mod shinmeikai;
mod wadai;
mod jsondict;
mod convert;
mod conversion_warnings;

use epwing::*;
use jsondict::*;
use conversion_warnings::*;

fn read_to_file(fname : &str) -> String
{
    let mut string = "".to_string();
    File::open(fname).unwrap().read_to_string(&mut string).unwrap();
    string
}

fn main()
{
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2
    {
        eprintln!("usage: ./convert <filename>");
    }
    else
    {
        let text = read_to_file(args[1].as_str());
        warn_about_conversion_errors(&text);
        let d : EpwingRoot = serde_json::from_str(&text).unwrap();
        let d = JsonDict::from_epwing(&d.subbooks[0]);
        println!("{}", d.jsonify());
    }
}
