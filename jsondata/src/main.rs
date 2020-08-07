extern crate csv;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::env;
use std::error::Error;

#[derive(Serialize, Deserialize)]
struct Ptt {
    articles: Vec<PttArticle>
}

#[derive(Serialize, Deserialize)]
struct PttArticle{
    article_id: Option<String>,
    article_title: Option<String>,
    author: Option<String>,
    board: Option<String>,
    content: Option<String>,
    date: Option<String>,
    ip: Option<String>,
    message_count:Option<PttMessageCount>,
    messages:Option<Vec<PttMessages>>,
    error:Option<String>
}

#[derive(Serialize, Deserialize)]
struct PttMessageCount {
    all: Option<i16>,
    boo: Option<i16>,
    count: Option<i16>,
    neutral: Option<i16>,
    push: Option<i16>
}

#[derive(Serialize, Deserialize)]
struct PttMessages {
    push_content: Option<String>,
    push_ipdatetime: Option<String>,
    push_tag: Option<String>,
    push_userid: Option<String>
}

fn main()  -> Result<(), Box<dyn std::error::Error>>{
    let mut wtr = csv::Writer::from_path("test.csv")?;
    wtr.write_record(&["標題","作者","內容","日期","文章類型"])?;
  
    let mut file = File::open("Gossiping-1-1000.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let ptt: Ptt = serde_json::from_str::<Ptt>(&data).unwrap();
    for elem in ptt.articles{
        if !elem.error.is_some() && elem.author.is_some() && elem.article_title.is_some() && elem.content.is_some() && elem.date.is_some() {
      
            let title =  elem.article_title.unwrap();
            println!("{}", title);  
            if title.clone().starts_with("Re:"){
                wtr.write_record(&[title,elem.author.unwrap(), elem.content.unwrap(), elem.date.unwrap(),String::from("回文")])?;

            }else{
                wtr.write_record(&[title,elem.author.unwrap(), elem.content.unwrap(), elem.date.unwrap(),String::from("主文")])?;

            }
                   }
    }
    wtr.flush()?;
    println!("{}", env::args().nth(3).ok_or("Missing argument")?);
    Ok(())
}
