use serde_derive::{Serialize, Deserialize};

use crate::epwing::*;
use crate::convert::*;

pub (crate) struct JsonDict {
    pub (crate) name: String,
    pub (crate) entries: Vec<JsonEntry>
}

#[derive(Serialize, Deserialize)]
pub (crate) struct JsonEntry {
    pub (crate)r: String,
    pub (crate)s: Vec<String>,
    pub (crate) l: Vec<String>,
}

impl JsonDict {
    pub (crate) fn from_epwing(book : &EpwingBook) -> JsonDict
    {
        let name = book.title.clone();
        let mut entries = vec!();
        if let Some(converter) = get_converter(&name)
        {
            for entry in &book.entries
            {
                if let Some(converted) = converter(entry)
                {
                    entries.push(converted);
                }
            }
        }
        else
        {
            eprintln!("Error: There is no converter for this dictionary ({}). Output dictionary will have no entries.", name);
            eprintln!("The following dictionaries are supported:");
            eprintln!("{}", supported().join("\n"));
            panic!();
        }
        JsonDict{name, entries}
    }
    pub (crate) fn jsonify(&self) -> String
    {
        let mut lines = vec!();
        lines.push(format!("[{},", escape_wrap(&self.name)));
        for (i, entry) in self.entries.iter().enumerate()
        {
            lines.push(format!("{}{}", serde_json::to_string_pretty(&entry).unwrap(), if i+1 == self.entries.len() { "" } else { "," }));
        }
        lines.push("]".to_string());
        
        lines.join("\n")
    }
}

fn escape_wrap(text: &str) -> String
{
    let mut ret = String::with_capacity(text.len()*2+2);
    ret.push('"');
    let chars : Vec<char> = text.chars().collect();
    for c in chars
    {
        match c
        {
            '\\' => ret.extend(&['\\', '\\']),
            '\n' => ret.extend(&['\\', 'n']),
            '\r' => ret.extend(&['\\', 'r']),
            '\t' => ret.extend(&['\\', 't']),
            '\"' => ret.extend(&['\\', '"']),
            _ => ret.push(c)
        }
    }
    ret.push('"');
    ret
}
