use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

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

fn main() {
    let mut file = File::open("Gossiping-1-1000.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let ptt: Ptt = serde_json::from_str::<Ptt>(&data).unwrap();
    for elem in ptt.articles{
        if elem.article_title.is_some() {
            println!("{}", elem.article_title.unwrap());   
        }

    }

 
}
