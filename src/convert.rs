#![allow(clippy::ptr_arg)]

use text_io::{try_read, try_scan, read};

use crate::shinmeikai::*;
use crate::wadai::*;
use crate::daijirin::*;
use crate::jsondict::*;
use crate::epwing::*;

type Converter = Fn(&EpwingEntry) -> Option<JsonEntry>;

pub (crate) fn supported() -> Vec<&'static str>
{
    vec!("新明解国語辞典　第五版", "研究社　新和英大辞典　第５版")
}

pub (crate) fn get_converter(name : &String) -> Option<&'static Converter>
{
    match name.as_str()
    {
        "新明解国語辞典　第五版" =>
        {
            eprintln!("There are multiple converters for this dictionary. Select the one you want to use.");
            eprintln!("1: With Examples");
            eprintln!("2: Without Examples");
            eprintln!("Enter a number: ");
            
            let i: i32 = read!();
            match i
            {
                1 => Some(&convert_shinmeikai_5),
                2 => Some(&convert_shinmeikai_5_no_examples),
                _ => None
            }
        }
        "研究社　新和英大辞典　第５版" =>
        {
            eprintln!("There are multiple converters for this dictionary. Select the one you want to use.");
            eprintln!("1: No Stripping");
            eprintln!("2: Light Stripping");
            eprintln!("3: Heavy Stripping");
            eprintln!("Enter a number: ");
            
            let i: i32 = read!();
            match i
            {
                1 => Some(&convert_wadai_5_no_stripping),
                2 => Some(&convert_wadai_5_light_stripping),
                3 => Some(&convert_wadai_5_heavy_stripping),
                _ => None
            }
        }
        "三省堂　スーパー大辞林" =>
         {
             eprintln!("There are multiple converters for this dictionary. Select the one you want to use.");
             eprintln!("1: With waei and eiwa entries; With examples");
             eprintln!("2: Without waei and eiwa entries; With examples");
             eprintln!("3: With waei and eiwa entries; Without examples");
             eprintln!("4: Without waei and eiwa entries; Without examples");
             eprintln!("Enter a number: ");

             let i: i32 = read!();
             match i
             {
                 1 => Some(&convert_daijirin),
                 2 => Some(&convert_daijirin_no_waei_and_eiwa),
                 3 => Some(&convert_daijirin_no_examples),
                 4 => Some(&convert_daijirin_no_waei_and_eiwa_no_examples),
                 _ => None
             }
         }
        _ => None
    }
}
