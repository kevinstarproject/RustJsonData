extern crate csv;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::env;
use std::collections::HashMap;

#[derive(Serialize, Deserialize,Clone)]
struct Ptt {
    articles: Vec<PttArticle>,
    url:Option<String>
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
    error:Option<String>,
    url:Option<String>
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

    let mut months = HashMap::<String, _>::new();

    months.insert("Jan".to_owned(), "01");
    months.insert("Feb".to_owned(), "02");
    months.insert("Mar".to_owned(), "03");
    months.insert("Apr".to_owned(), "04");
    months.insert("May".to_owned(), "05");
    months.insert("Jun".to_owned(), "06");
    months.insert("Jul".to_owned(), "07");
    months.insert("Aug".to_owned(), "08");
    months.insert("Sep".to_owned(), "09");
    months.insert("Oct".to_owned(), "10");
    months.insert("Nov".to_owned(), "11");
    months.insert("Dec".to_owned(), "12");
    let mut json_file = "Gossiping-{start}-{end}.json" ;
    let mut csv_file = "Gossiping-{start}-{end}.csv" ;
    let mut wtr = csv::Writer::from_path("Gossiping.csv")?;
    wtr.write_record(&["文章編號","標題","作者","內容","日期","文章類型","讚數","噓數","中立數"])?;
    for x in 0..26{
        let start = x*1000+9000;
        let end = start+999;
        print!("{}",json_file.replace("{start}",&start.to_string()).replace("{end}",&end.to_string()));
 
   // let mut wtr = csv::Writer::from_path(csv_file.replace("{start}",&start.to_string()).replace("{end}",&end.to_string()))?;
    //wtr.write_record(&["文章編號","標題","作者","內容","日期","文章類型","讚數","噓數","中立數"])?;
  
    let mut file = File::open(json_file.replace("{start}",&start.to_string()).replace("{end}",&end.to_string())).unwrap();
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
          
             
            let keywords = ["新冠肺炎","武漢肺炎","COVID-19"];
            let mut related = false;
            for k in keywords.iter(){
                if content.contains(k) || title.contains(k) {
                    related = true;
                    println!("{}", k); 
                    break;
                }
            }
   let res: Vec<String> = article_date.split_whitespace().map(|s| s.to_string()).collect();
            if &res.len()>&4{
                if !months.contains_key(&res[1]){
                    continue;
                }
                let this_date = format!("{}/{}/{} {}",&res[4],months.get(&res[1]).unwrap(),&res[2],&res[3]);
                

                if related {
                 println!("{}", title); 
                 if title.clone().starts_with("Re:"){
                        wtr.write_record(&[&article_id,&title,&elem.author.unwrap(),&content, &this_date ,&String::from("回文"),&push_count,&boo_count,&neutral_count])?;

                    }else{
                        wtr.write_record(&[&article_id,&title,&elem.author.unwrap(), &content, &this_date ,&String::from("主文"),&push_count,&boo_count,&neutral_count])?;

                    }



                     if elem.messages.is_some(){
                let messages = elem.messages.unwrap();
                for m in messages{
                    let push_content = m.push_content.unwrap();
                    let push_res: Vec<String> = m.push_ipdatetime.unwrap().split_whitespace().map(|s| s.to_string()).collect();
                    if push_res.len() > 2{
                    let push_date = format!("{}/{} {}:00",&res[4],&push_res[1],&push_res[2]);
                    wtr.write_record(&[&article_id,&title,&m.push_userid.unwrap(),&push_content, &push_date,&m.push_tag.unwrap(),&push_count,&boo_count,&neutral_count])?;
               }
                }
            }
           
            }
            }

           
        }
    }
       }
      wtr.flush()?;
    println!("{}", env::args().nth(3).ok_or("Missing argument")?);
    Ok(())
}
