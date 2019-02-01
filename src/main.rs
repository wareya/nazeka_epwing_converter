use std::fs::File;
use std::io::Read;
use std::env;

mod epwing;
mod shinmeikai;
mod jsondict;
mod convert;

use epwing::*;
use jsondict::*;

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
        let d : EpwingRoot = serde_json::from_str(&read_to_file(args[1].as_str())).unwrap();
        let d = JsonDict::from_epwing(d.subbooks.get(0).unwrap());
        println!("{}", d.jsonify());
    }
}
