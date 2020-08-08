extern crate csv;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::env;

#[derive(Serialize, Deserialize,Clone)]
struct Ptt {
    articles: Vec<PttArticle>
}
#[derive(Serialize, Deserialize,Clone)]
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

#[derive(Serialize, Deserialize, Clone)]
struct PttMessageCount {
    all: Option<i16>,
    boo: Option<i16>,
    count: Option<i16>,
    neutral: Option<i16>,
    push: Option<i16>
}

#[derive(Serialize, Deserialize, Clone)]
struct PttMessages {
    push_content: Option<String>,
    push_ipdatetime: Option<String>,
    push_tag: Option<String>,
    push_userid: Option<String>
}

fn main()  -> Result<(), Box<dyn std::error::Error>>{
    let mut wtr = csv::Writer::from_path("HatePolitics-4000-5000.csv")?;
    //wtr.write_record(&["文章編號","標題","作者","內容","日期","文章類型","讚數","噓數","中立數"])?;
  
    let mut file = File::open("HatePolitics-4000-5000.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let ptt: Ptt = serde_json::from_str::<Ptt>(&data).unwrap();
    for elem in ptt.articles{
        if !elem.error.is_some() && elem.author.is_some() && elem.article_title.is_some() && elem.content.is_some() && elem.date.is_some() {
      
            let title =  elem.article_title.unwrap();
            let content =  elem.content.unwrap();
            let article_id =  elem.article_id.unwrap();
            let article_date  = elem.date.unwrap();


         
            let mut push_count = String::from("0");
            let mut boo_count = String::from("0");
            let mut neutral_count = String::from("0");


            if elem.message_count.is_some(){
                let message_count = elem.message_count.unwrap();
                push_count = message_count.push.unwrap().to_string();
                boo_count = message_count.boo.unwrap().to_string();
                neutral_count = message_count.neutral.unwrap().to_string();
            }
          
             
            let keywords = ["新冠肺炎","武漢肺炎","COVID-19","新冠"];
            let mut related = false;
            for k in keywords.iter(){
                if content.contains(k) || title.contains(k) {
                    related = true;
                    println!("{}", k); 
                    break;
                }
            }

   println!("date {}", &article_date ); 
            if related {
                 println!("{}", title); 
                 if title.clone().starts_with("Re:"){
                        wtr.write_record(&[&article_id,&title,&elem.author.unwrap(),&content, &article_date ,&String::from("回文"),&push_count,&boo_count,&neutral_count])?;

                    }else{
                        wtr.write_record(&[&article_id,&title,&elem.author.unwrap(), &content, &article_date ,&String::from("主文"),&push_count,&boo_count,&neutral_count])?;

                    }



                     if elem.messages.is_some(){
                let messages = elem.messages.unwrap();
                for m in messages{
                    let push_content = m.push_content.unwrap();
  
                  wtr.write_record(&[&article_id,&title,&m.push_userid.unwrap(),&push_content, &m.push_ipdatetime.unwrap(),&m.push_tag.unwrap(),&push_count,&boo_count,&neutral_count])?;
               }
            }
           
            }

           
        }
    }
    wtr.flush()?;
    println!("{}", env::args().nth(3).ok_or("Missing argument")?);
    Ok(())
}
