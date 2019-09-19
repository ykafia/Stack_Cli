extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use std::fmt::{Display, Formatter, Result};
use std::string::ToString;

pub fn build_request(keywords: Vec<String>, tab: Option<String>, page: Option<i32>) -> reqwest::Url
where
    Vec<String>: ToQuery,
{
    let p = match page {
        Some(x) => x,
        None => 1,
    };
    let t = match tab {
        Some(x) => x,
        None => "Relevance".to_string(),
    };
    // let mut stack_url = ScrapUri {
    //     base: "https://stackoverflow.com/".to_string(),
    //     extension: "search?".to_string(),
    // };
    // stack_url.extension = stack_url.extension + &"page=" + &p.to_string();
    // stack_url.extension = stack_url.extension + &"&tab=" + &t.to_string();
    // stack_url.extension = stack_url.extension + &"&q=".to_string() + &keywords.to_query();
    // stack_url
    return reqwest::Url::parse_with_params(
        "https://stackoverflow.com/search?",
        &[
            ("page", "1"),
            ("tab", &t.to_string()),
            ("q", &keywords.to_query()),
        ],
    )
    .unwrap();
}

pub fn stack_search(url: reqwest::Url) {
    println!("Receiving response from {}", url.to_string());

    let client = reqwest::Client::new();
    let resp = client.get(&url.to_string()).send().unwrap();
    println!("Checking DOM");
    let document = Document::from_read(v).unwrap();

    // finding all instances of our class of interest
    for node in document.find(Class("summary")) {
        // grabbing the story rank
        let link = node
            .find(Class("question-hyperlink"))
            .next()
            .unwrap()
            .attr("href")
            .unwrap();
        let question = node
            .find(Class("question-hyperlink"))
            .next()
            .unwrap()
            .text();
        let excerpt = node.find(Class("excerpt")).next().unwrap().text();

        // printing out | rank | story headline
        println!("{}\nhttps://stackoverflow.com/{}\n", question, link);
    }
}


pub struct QuestionChoice{
    question:String,
    link:String
}


pub trait ToQuery {
    fn to_query(&self) -> String;
}
impl ToQuery for Vec<String> {
    fn to_query(&self) -> String {
        let mut tmp = String::new();
        for i in self {
            tmp = tmp + i + " ";
        }
        tmp
    }
}
