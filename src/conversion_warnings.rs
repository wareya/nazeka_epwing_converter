pub (crate) fn warn_about_conversion_errors(text : &str)
{
    let entries_start = text.find("entries").unwrap();
    if text[..entries_start].contains("新明解国語辞典　第五版")
    {
        eprintln!("WARNING: zero-epwing rips this dictionary incorrectly.");
        eprintln!("Three entries are missing their definitions, which will cause this converter to crash.");
        eprintln!("You need to add the correct definitions manually. Instructions follow.");
        
        eprintln!("");
        
        eprintln!(r"WARNING: The definitions should not contain literal linebreaks. Remove any that are created by copying the text out of your terminal, and do not convert the \n escape sequnces to real newlines.");
        
        eprintln!("");
        
        eprintln!(r#"from:"#);
        eprintln!(r#""heading": "かもく【科目】クワモク[0]""#);
        eprintln!(r#"to:"#);
        eprintln!(r#""heading": "かもく【科目】クワモク[0]","#);
        eprintln!(r#""text": "(一)幾つかの種類に分けた、一つひとつの項目。\n 「勘定―[5]」\n(二)種類分けした教科の一つひとつ。\n 「高等学校の国語科には国語・国語・古典・現代文・国語表現などの―がある／必修―[5]」\n[表記](二)は、「課目」とも書く。""#);
        
        eprintln!("");
        
        eprintln!(r#"from:"#);
        eprintln!(r#""heading": "こうぶん【高文】カウブン[0]""#);
        eprintln!(r#"to:"#);
        eprintln!(r#""heading": "こうぶん【高文】カウブン[0]","#);
        eprintln!(r#""text": "もと、「高等文官試験」の略。現在の「国家公務員採用試験種」に当たる。""#);
        eprintln!("");
        
        eprintln!(r#"from:"#);
        eprintln!(r#""heading": "キャリア[1]""#);
        eprintln!(r#"to:"#);
        eprintln!(r#""heading": "キャリア[1]","#);
        eprintln!(r#""text": "[一]〔career＝経歴〕\n(一)その方面で実際に場数を踏んで来た経験年数。〔狭義では、競技歴・試合経験を指す〕\n 「―ウーマン」\n(二)国家公務員試験種に合格した者の通称。\n 「―組グミ・ノン―」\n[二]〔carrier〕\n(一)自転車などの荷台。\n(二)品物を運ぶ器具。\n 「―カー」\n(三)航空会社・運輸会社。\n 「フラッグ―」\n(四)病原菌やウイルスに感染しながら発病せず、しかも保菌し続ける人。保菌者。""#);
        eprintln!("");
        
        eprintln!(r"Note: The exact presence of a , on the first line and lack of a , on the second line are mandatory.");
        
        eprintln!("");
    }
}