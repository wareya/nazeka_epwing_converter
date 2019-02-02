use super::jsondict::*;
use super::epwing::*;

use regex::Regex;

fn shinmeikai_heading_rendering(reading: &String, spellings : &Vec<String>) -> String
{
    let mut out = reading.clone();
    for spelling in spellings
    {
        if spelling.as_str() != ""
        {
            out = format!("{}【{}】", out, spelling);
        }
    }
    out
}

fn shinmeikai_strip_second_heading(definition : &mut Vec<String>, reading: &String, spellings : &Vec<String>)
{
    if !definition.is_empty()
    {
        definition[0] = definition[0].replacen(&shinmeikai_heading_rendering(&reading, &spellings), "Info: ", 1);
        if definition[0].as_str() == "Info: "
        {
            definition.remove(0);
        }
    }
}

fn shinmeikai_strip_examples(definition : &mut Vec<String>)
{
    let re = Regex::new("^「.*」$").unwrap();
    let mut i = 0;
    while i < definition.len()
    {
        if re.is_match(&definition[i])
        {
            definition.remove(i);
        }
        else
        {
            i += 1;
        }
    }
}

pub (crate) fn convert_shinmeikai_5(entry : &EpwingEntry) -> Option<JsonEntry>
{
    if let (Some(mut definition), Some((reading, spellings))) = (shinmeikai_body_converter(&entry.text), shinmeikai_heading_converter(&entry.heading))
    {
        shinmeikai_strip_second_heading(&mut definition, &reading, &spellings);
        Some(JsonEntry{r:reading, s:spellings, l:definition})
    }
    else
    {
        None
    }
}

pub (crate) fn convert_shinmeikai_5_no_examples(entry : &EpwingEntry) -> Option<JsonEntry>
{
    if let (Some(mut definition), Some((reading, spellings))) = (shinmeikai_body_converter(&entry.text), shinmeikai_heading_converter(&entry.heading))
    {
        shinmeikai_strip_second_heading(&mut definition, &reading, &spellings);
        shinmeikai_strip_examples(&mut definition);
        Some(JsonEntry{r:reading, s:spellings, l:definition})
    }
    else
    {
        None
    }
}

fn shinmeikai_is_kana_toc(list : &Vec<String>) -> bool
{
    let re = Regex::new("^・.*〜.*$").unwrap();
    if list.len() > 2
    {
        for entry in &list[2..]
        {
            if !re.is_match(entry)
            {
                return false;
            }
        }
        return true;
    }
    return false;
}
fn shinmeikai_is_kana_kanji_ref(list : &Vec<String>) -> bool
{
    let re = Regex::new("^（.*）[ ]*→【字音語の造語成分】$").unwrap();
    if list.len() == 2
    {
        return re.is_match(&list[1]);
    }
    return false;
}

fn shinmeikai_body_converter(body : &String) -> Option<Vec<String>>
{
    let mut entries : Vec<String> = body.trim().split("\n").map(|s| s.to_string()).collect();
    let definition = entries.drain(..).map(|mut s| s.drain(..).collect()).collect();
    
    if shinmeikai_is_kana_toc(&definition) || shinmeikai_is_kana_kanji_ref(&definition)
    {
        return None;
    }
    
    Some(definition)
}

fn shinmeikai_heading_converter(heading : &String) -> Option<(String, Vec<String>)>
{
    let mut reading = vec!();
    let mut spellings = vec!();
    let heading_chars = heading.chars().collect::<Vec<char>>();
    let mut i = 0;
    let mut mode = 0; // 0 : reading; 1 : spelling; 2 : after spelling
    while i < heading_chars.len()
    {
        match mode
        {
            0 =>
            {
                match heading_chars[i]
                {
                    '［' => return None, // kanji entry
                    '【' =>
                    {
                        spellings.push(vec!());
                        mode = 1;
                    }
                    ' ' | '[' => break,
                    '】' => panic!("unknown state"),
                    c => reading.push(c)
                }
            }
            1 =>
            {
                match heading_chars[i]
                {
                    '】' => mode = 2,
                    '[' | '【' | '［' | ' ' => panic!("unknown state"),
                    c => spellings.last_mut().unwrap().push(c),
                }
            }
            2 =>
            {
                match heading_chars[i]
                {
                    '【' =>
                    {
                        spellings.push(vec!());
                        mode = 1;
                    }
                    _ => break
                }
            }
            _ => panic!("unknown state")
        }
        i += 1;
    }
    let reading = reading.drain(..).collect();
    let mut spellings : Vec<String> = spellings.drain(..).map(|mut s| s.drain(..).collect()).collect();
    if spellings.is_empty()
    {
        spellings.push("".to_string());
    }
    Some((reading, spellings))
}