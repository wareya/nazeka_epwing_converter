#![allow(clippy::ptr_arg)]

use super::jsondict::*;
use super::epwing::*;

use lazy_static::*;

pub (crate) fn convert_wadai_5_heavy_stripping(entry : &EpwingEntry) -> Option<JsonEntry>
{
    if let (Some(definition), Some((reading, spellings))) = (wadai_body_converter_heavy_stripping(&entry.text), wadai_heading_converter(&entry.heading))
    {
        if definition.is_empty()
        {
            panic!("empty {:?}", entry.heading);
        }
        else
        {
            Some(JsonEntry{r:reading, s:spellings, l:definition})
        }
    }
    else
    {
        None
    }
}
pub (crate) fn convert_wadai_5_light_stripping(entry : &EpwingEntry) -> Option<JsonEntry>
{
    if let (Some(definition), Some((reading, spellings))) = (wadai_body_converter_light_stripping(&entry.text), wadai_heading_converter(&entry.heading))
    {
        if definition.is_empty()
        {
            panic!("empty {:?}", entry.heading);
        }
        else
        {
            Some(JsonEntry{r:reading, s:spellings, l:definition})
        }
    }
    else
    {
        None
    }
}
pub (crate) fn convert_wadai_5_no_stripping(entry : &EpwingEntry) -> Option<JsonEntry>
{
    if let (Some(definition), Some((reading, spellings))) = (wadai_body_converter_no_stripping(&entry.text), wadai_heading_converter(&entry.heading))
    {
        if definition.is_empty()
        {
            panic!("empty {:?}", entry.heading);
        }
        else
        {
            Some(JsonEntry{r:reading, s:spellings, l:definition})
        }
    }
    else
    {
        None
    }
}

fn wadai_body_converter_heavy_stripping(body : &String) -> Option<Vec<String>>
{
    let body = wadai_gaiji_fixer(body.as_str());
    
    let mut entries : Vec<String> = body.trim().split('\n').map(|s| s.to_string()).collect();
    let mut definition : Vec<String> = entries.drain(..).map(|mut s| s.drain(..).collect()).collect();
    if !definition.is_empty()
    {
        let first_line = definition.remove(0);
        if !first_line.contains("[ローマ字]")
        {
            return None;
        }
    }
    if definition.is_empty()
    {
        return None;
    }
    
    let first_char = definition[0].chars().next();
    if first_char.is_none()
    {
        println!("{:?}", body);
        panic!();
    }
    let first_char = first_char.unwrap();
    if first_char == '・'
    {
        return Some(definition);
    }
    else if first_char == '►'
    {
        let mut ret = vec!();
        for def in definition
        {
            if let Some(first_char) = def.chars().next()
            {
                if first_char == '・' || first_char == '◨' || first_char == '◧'
                {
                    break;
                }
            }
            ret.push(def);
        }
        return Some(ret);
    }
    else if first_char == '◨' || first_char == '◧'
    {
        let mut ret = vec!();
        for def in definition
        {
            if let Some(first_char) = def.chars().next()
            {
                if first_char == '◨' || first_char == '◧'
                {
                    ret.push(def);
                }
            }
        }
        return Some(ret);
    }
    else if first_char == '1'
    {
        let mut ret = vec!();
        let mut mode = 0;
        for def in definition
        {
            if let Some(first_char) = def.chars().next()
            {
                if first_char == '►' || first_char == '・' || first_char == '◨' || first_char == '◧'
                {
                    mode = 1;
                }
                if first_char as u32 >= 0x30 && first_char as u32 <= 0x39
                {
                    mode = 0;
                }
            }
            if mode == 0
            {
                ret.push(def);
            }
        }
        return Some(ret);
    }
    else
    {
        let mut ret = vec!();
        for def in definition
        {
            if let Some(first_char) = def.chars().next()
            {
                if first_char == '►' || first_char == '・' || first_char == '◨' || first_char == '◧'
                {
                    break;
                }
            }
            ret.push(def);
        }
        return Some(ret);
    }
}

fn wadai_body_converter_light_stripping(body : &String) -> Option<Vec<String>>
{
    let body = wadai_gaiji_fixer(body.as_str());
    
    let mut entries : Vec<String> = body.trim().split('\n').map(|s| s.to_string()).collect();
    let mut definition : Vec<String> = entries.drain(..).map(|mut s| s.drain(..).collect()).collect();
    if !definition.is_empty()
    {
        let first_line = definition.remove(0);
        if !first_line.contains("[ローマ字]")
        {
            return None;
        }
    }
    if definition.is_empty()
    {
        return None;
    }
    
    let first_char = definition[0].chars().next();
    if first_char.is_none()
    {
        println!("{:?}", body);
        panic!();
    }
    let first_char = first_char.unwrap();
    if first_char == '・'
    {
        return Some(definition);
    }
    else if first_char == '◨' || first_char == '◧'
    {
        return Some(definition);
    }
    else if first_char == '1'
    {
        let mut ret = vec!();
        let mut mode = 0; // 0: all; 1: only starting with ►◨◧
        for def in definition
        {
            if let Some(first_char) = def.chars().next()
            {
                if first_char == '・'
                {
                    mode = 1;
                    continue;
                }
                if first_char == '◨' || first_char == '◧' || first_char == '►'
                {
                    mode = 1;
                }
                if first_char as u32 >= 0x30 && first_char as u32 <= 0x39
                {
                    mode = 0;
                }
            }
            if mode == 0 || first_char == '►' || first_char == '◨' || first_char == '◧'
            {
                ret.push(def);
            }
        }
        return Some(ret);
    }
    else
    {
        let mut ret = vec!();
        let mut mode = 0; // 0: accept all; 1: only accept lines starting with ►◨◧
        for def in definition
        {
            if let Some(first_char) = def.chars().next()
            {
                if first_char == '・'
                {
                    mode = 1;
                    continue;
                }
                if first_char == '◨' || first_char == '◧'
                {
                    mode = 1;
                }
            }
            if mode == 0 || first_char == '►' || first_char == '◨' || first_char == '◧'
            {
                ret.push(def);
            }
        }
        return Some(ret);
    }
}
fn wadai_body_converter_no_stripping(body : &String) -> Option<Vec<String>>
{
    let body = wadai_gaiji_fixer(body.as_str());
    
    let mut entries : Vec<String> = body.trim().split('\n').map(|s| s.to_string()).collect();
    let mut definition : Vec<String> = entries.drain(..).map(|mut s| s.drain(..).collect()).collect();
    if !definition.is_empty()
    {
        let first_line = definition.remove(0);
        if !first_line.contains("[ローマ字]")
        {
            return None;
        }
    }
    if definition.is_empty()
    {
        return None;
    }
    
    let first_char = definition[0].chars().next();
    if first_char.is_none()
    {
        println!("{:?}", body);
        panic!();
    }
    
    return Some(definition);
}

fn wadai_heading_converter(heading : &String) -> Option<(String, Vec<String>)>
{
    let mut heading = wadai_gaiji_fixer(heading.as_str());
    
    let first_char = heading.chars().next().unwrap();
    if first_char == '¶' || !heading.contains('＜') || !heading.contains('＞')
    {
        return None;
    }
    
    heading = heading.split('＜').nth(1).unwrap().split('＞').nth(0).unwrap().to_string();
    
    let mut spellings;
    let mut reading;
    if heading.contains('【')
    {
        spellings = heading.split('【').nth(1).unwrap().split('】').nth(0).unwrap().split('・').map(str::to_string).collect();
        reading = heading.split('【').nth(0).unwrap().to_string();
    }
    else
    {
        spellings = vec!("".to_string());
        reading = heading;
    }
    reading = reading.trim_end_matches(|c| c as u32 >= 0xFF10 && c as u32 <= 0xFF19).trim_start_matches('-').to_string();
    
    for spelling in spellings.iter_mut()
    {
        *spelling = spelling.trim_start_matches('-').to_string();
    }
    
    Some((reading, spellings))
}

fn wadai_gaiji_fixer(text : &str) -> String
{
    let mut text = text.to_string();
    
    for mapping in GAIJI_MAPPING.iter()
    {
        text = text.replace(mapping.0, mapping.1);
    }
    text
}

lazy_static! {
    static ref GAIJI_MAPPING : Vec<(&'static str, &'static str)> = vec!(
        ("{{n_41267}}", "﹢"),
        ("{{n_41269}}", "*"),
        ("{{n_41270}}", "ᐦ"),
        ("{{n_41284}}", "Á"),
        ("{{n_41285}}", "É"),
        ("{{n_41287}}", "Ó"),
        ("{{n_41288}}", "Ú"),
        ("{{n_41290}}", "á"),
        ("{{n_41291}}", "é"),
        ("{{n_41292}}", "í"),
        ("{{n_41293}}", "ó"),
        ("{{n_41294}}", "ú"),
        ("{{n_41295}}", "ý"),
        ("{{n_41313}}", "À"),
        ("{{n_41314}}", "È"),
        ("{{n_41319}}", "à"),
        ("{{n_41320}}", "è"),
        ("{{n_41321}}", "ì"),
        ("{{n_41322}}", "ò"),
        ("{{n_41323}}", "ù"),
        ("{{n_41505}}", "Ö"),
        ("{{n_41506}}", "Ü"),
        ("{{n_41508}}", "ä"),
        ("{{n_41509}}", "ë"),
        ("{{n_41510}}", "ï"),
        ("{{n_41511}}", "ö"),
        ("{{n_41512}}", "ü"),
        ("{{n_41513}}", "ÿ"),
        ("{{n_41515}}", "Â"),
        ("{{n_41516}}", "Ê"),
        ("{{n_41517}}", "Î"),
        ("{{n_41520}}", "â"),
        ("{{n_41521}}", "ê"),
        ("{{n_41522}}", "î"),
        ("{{n_41523}}", "ô"),
        ("{{n_41524}}", "û"),
        ("{{n_41525}}", "ā"),
        ("{{n_41526}}", "ē"),
        ("{{n_41527}}", "ī"),
        ("{{n_41528}}", "ō"),
        ("{{n_41529}}", "ū"),
        ("{{n_41530}}", "ȳ"),
        ("{{n_41532}}", "Ç"),
        ("{{n_41533}}", "ç"),
        ("{{n_41534}}", "ɘ́"),
        ("{{n_41538}}", "ɔ́"),
        ("{{n_41561}}", "˜"),
        ("{{n_41566}}", "ã"),
        ("{{n_41567}}", "ñ"),
        ("{{n_41581}}", "ʌ"),
        ("{{n_41582}}", "ø"),
        ("{{n_41583}}", "ə"),
        ("{{n_41585}}", "ε"),
        ("{{n_41587}}", "ɔ"),
        ("{{n_41588}}", "℧"),
        ("{{n_41590}}", "ð"),
        ("{{n_41593}}", "ŋ"),
        ("{{n_41594}}", "ː"),
        ("{{n_41596}}", "Ø"),
        ("{{n_41762}}", "\\"),
        ("{{n_41768}}", "˘"),
        ("{{n_41773}}", "Ŭ"),
        ("{{n_41775}}", "ă"),
        ("{{n_41776}}", "ĕ"),
        ("{{n_41777}}", "ğ"),
        ("{{n_41778}}", "ĭ"),
        ("{{n_41779}}", "ŏ"),
        ("{{n_41780}}", "ŭ"),
        ("{{n_41784}}", "Č"),
        ("{{n_41788}}", "Š"),
        ("{{n_41791}}", "č"),
        ("{{n_41792}}", "ě"),
        ("{{n_41794}}", "ň"),
        ("{{n_41795}}", "ř"),
        ("{{n_41796}}", "š"),
        ("{{n_41797}}", "ž"),
        ("{{n_41804}}", "ą"),
        ("{{n_41805}}", "ę"),
        ("{{n_41811}}", "ș"),
        ("{{n_41812}}", "ț"),
        ("{{n_41822}}", "Ś"),
        ("{{n_41823}}", "ć"),
        ("{{n_41824}}", "ń"),
        ("{{n_41825}}", "ś"),
        ("{{n_41826}}", "ź"),
        ("{{n_42061}}", "‘"),
        ("{{n_42063}}", "Ł"),
        ("{{n_42068}}", "ł"),
        ("{{n_42071}}", "õ"),
        ("{{n_42075}}", "Å"),
        ("{{n_42076}}", "å"),
        ("{{n_42077}}", "ů"),
        ("{{n_42081}}", "Ḥ"),
        ("{{n_42089}}", "ḍ"),
        ("{{n_42090}}", "ḥ"),
        ("{{n_42092}}", "ṃ"),
        ("{{n_42093}}", "ṇ"),
        ("{{n_42095}}", "ṣ"),
        ("{{n_42102}}", "İ"),
        ("{{n_42104}}", "Ż"),
        ("{{n_42109}}", "ṅ"),
        ("{{n_42287}}", "‴"),
        ("{{n_42316}}", "Ō"),
        ("{{n_42322}}", "b̄"),
        ("{{n_42324}}", "d̅"),
        ("{{n_42325}}", "h̄"),
        ("{{n_42327}}", "s̅"),
        ("{{n_42330}}", "z̅"),
        ("{{n_42344}}", "〚"),
        ("{{n_42345}}", "〛"),
        ("{{n_42356}}", "ǔ"),
        ("{{n_42357}}", "ż"),
        ("{{n_42358}}", "Ž"),
        ("{{n_42359}}", "ž"),
        ("{{w_45380}}", "☞"),
        ("{{w_45397}}", "æ"),
        ("{{w_45402}}", "œ"),
        ("{{w_45406}}", "Æ"),
        ("{{w_45429}}", "©"),
        ("{{w_45613}}", "<"),
        ("{{w_45614}}", ">"),
        ("{{w_45629}}", "┏"),
        ("{{w_45653}}", "⛤"),
        ("{{w_45662}}", "嗉"),
        ("{{w_45665}}", "圳"),
        ("{{w_45666}}", "拼"),
        ("{{w_45667}}", "攩"),
        ("{{w_45671}}", "烤"),
        ("{{w_45673}}", "玢"),
        ("{{w_45674}}", "癤"),
        ("{{w_45675}}", "皶"),
        ("{{w_45676}}", "磠"),
        ("{{w_45677}}", "稃"),
        ("{{w_45681}}", "蔲"),
        ("{{w_45684}}", "顬"),
        ("{{w_45685}}", "骶"),
        ("{{w_45689}}", "榍"),
        ("{{w_45857}}", "倻"),
        ("{{w_45870}}", "噯"),
        ("{{w_45876}}", "垜"),
        ("{{w_45898}}", "愷"),
        ("{{w_45900}}", "擤"),
        ("{{w_45906}}", "晷"),
        ("{{w_45909}}", "枘"),
        ("{{w_45910}}", "不"),
        ("{{w_45913}}", "楣"),
        ("{{w_45916}}", "梲"),
        ("{{w_45919}}", "桛"),
        ("{{w_45921}}", "楤"),
        ("{{w_45922}}", "橅"),
        ("{{w_45923}}", "檉"),
        ("{{w_45933}}", "淄"),
        ("{{w_46125}}", "煆"),
        ("{{w_46135}}", "珅"),
        ("{{w_46137}}", "琛"),
        ("{{w_46141}}", "痤"),
        ("{{w_46142}}", "癭"),
        ("{{w_46143}}", "瘭"),
        ("{{w_46152}}", "窠"),
        ("{{w_46154}}", "笯"),
        ("{{w_46155}}", "筠"),
        ("{{w_46156}}", "簎"),
        ("{{w_46157}}", "糝"),
        ("{{w_46161}}", "翟"),
        ("{{w_46163}}", "翮"),
        ("{{w_46166}}", "腊"),
        ("{{w_46168}}", "舢"),
        ("{{w_46169}}", "芷"),
        ("{{w_46177}}", "蒴"),
        ("{{w_46181}}", "蕙"),
        ("{{w_46190}}", "蚉"),
        ("{{w_46191}}", "蝲"),
        ("{{w_46197}}", "豇"),
        ("{{w_46198}}", "跑"),
        ("{{w_46200}}", "跗"),
        ("{{w_46201}}", "跆"),
        ("{{w_46202}}", "蒁"),
        ("{{w_46372}}", "鄱"),
        ("{{w_46374}}", "鄧"),
        ("{{w_46388}}", "卍"),
        ("{{w_46390}}", "𨫤"),
        ("{{w_46391}}", "鈹"),
        ("{{w_46398}}", "顥"),
        ("{{w_46404}}", "駃"),
        ("{{w_46405}}", "騠"),
        ("{{w_46406}}", "髁"),
        ("{{w_46409}}", "魳"),
        ("{{w_46410}}", "鱏"),
        ("{{w_46411}}", "鱓"),
        ("{{w_46414}}", "鱮"),
        ("{{w_46415}}", "鰶"),
        ("{{w_46416}}", "魬"),
        ("{{w_46417}}", "𩸽"),
        ("{{w_46418}}", "鯥"),
        ("{{w_46419}}", "鰙"),
        ("{{w_46422}}", "鮄"),
        ("{{w_46423}}", "鱵"),
        ("{{w_46424}}", "鷴"),
        ("{{w_46425}}", "鶍"),
        ("{{w_46426}}", "鵟"),
        ("{{w_46428}}", "鼯"),
        ("{{w_46449}}", "▶"),
        ("{{w_46459}}", "㧍"),
        ("{{w_46460}}", "嘈"),
        ("{{w_46461}}", "愈"),
        ("{{w_46462}}", "淝"),
        ("{{w_46634}}", "灤"),
        ("{{w_46635}}", "焮"),
        ("{{w_46636}}", "獮"),
        ("{{w_46637}}", "瓚"),
        ("{{w_46638}}", "絓"),
        ("{{w_46639}}", "芎"),
        ("{{w_46650}}", "薏"),
        ("{{w_46651}}", "辶"),
        ("{{w_46652}}", "醞"),
        ("{{w_46653}}", "挵"),
        ("{{w_46654}}", "飥"),
        ("{{w_46655}}", "鬐"),
        ("{{w_46656}}", "俏"),
        ("{{w_46657}}", "啐"),
        ("{{w_46658}}", "塼"),
        ("{{w_46659}}", "濰"),
        ("{{w_46660}}", "磲"),
        ("{{w_46661}}", "篊"),
        ("{{w_46662}}", "菀"),
        ("{{w_46663}}", "芩"),
        ("{{w_46664}}", "𧿹"),
        ("{{w_46665}}", "鈸"),
        ("{{w_46666}}", "驎"),
        ("{{w_46667}}", "硨"),
        ("{{w_46668}}", "蘞"),
        ("{{w_46669}}", "梣"),
        ("{{w_46670}}", "槵"),
        ("{{w_46671}}", "橉"),
        ("{{w_46672}}", "莧"),
        ("{{w_46682}}", "彔"),
        ("{{w_46683}}", "噦"),
        ("{{w_46684}}", "袘"),
        ("{{w_46685}}", "餺"),
        ("{{w_46686}}", "►"),
        ("{{w_46688}}", "棈"),
        ("{{w_46689}}", "▷"),
        ("{{w_46695}}", "[ローマ字]"),
        ("{{w_46699}}", "◧"),
        ("{{w_46700}}", "◨"),
    );
}
