extern crate reqwest;
extern crate select;
extern crate dialoguer;


use dialoguer::{theme::ColorfulTheme, Select};
use select::document::Document;
use select::predicate::Class;
use std::fmt::{Display, Formatter, Result};
use std::string::ToString;

pub fn build_request(keywords: Vec<String>, tab: Option<String>, page: Option<i32>) -> reqwest::Url
where
    Vec<String>: VecExtension,
{
    let p = match page {
        Some(x) => x,
        None => 1,
    };
    let t = match tab {
        Some(x) => x,
        None => "Relevance".to_string(),
    };
    
    return reqwest::Url::parse_with_params(
        "https://stackoverflow.com/search?",
        &[
            ("page", p.to_string()),
            ("tab", t.to_string()),
            ("q", keywords.to_spaced_string()),
        ],
    )
    .unwrap();
}

pub fn stack_search(url: &reqwest::Url, client: &reqwest::Client) -> Vec<QuestionChoice> {

    
    let mut result: Vec<QuestionChoice> = Vec::new();
    let resp = client.get(&url.to_string()).send().unwrap();
    let document = Document::from_read(resp).unwrap();

    for node in document.find(Class("summary")) {
        let link = node
            .find(Class("question-hyperlink"))
            .next()
            .unwrap()
            .attr("href")
            .unwrap();
        let mut question = node
            .find(Class("question-hyperlink"))
            .next()
            .unwrap()
            .text();
        question.remove(0);
        result.push(QuestionChoice {
            question: question,
            link: link.to_string(),
        });
    }
    return result;
}

/// This function displays the questions and answers from a stack overflow link
pub fn display_qa(url: &reqwest::Url, client: &reqwest::Client) -> String{
    
    let resp = client.get(&url.to_string()).send().unwrap();
    let document = Document::from_read(resp).unwrap();
    let question = document.find(Class("post-text")).next().unwrap().text();
    let accepted_answer = document.find(Class("accepted-answer")).next().unwrap().text();
    println!("{}\n\n\n{}",question,accepted_answer);
    let choices = &["Return","Quit"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What to do ?")
        .items(choices)
        .interact()
        .unwrap();
    return choices[selection].to_string();
}

/// Lets the user choose a question from a list of question choices
pub fn question_check(values: &mut Vec<QuestionChoice>) -> String {
 
    let selects = values;
    selects.push(QuestionChoice {
        question: "Return".to_string(),
        link: "Return".to_string(),
    });
    selects.push(QuestionChoice {
        question: "Search Again".to_string(),
        link: "Search Again".to_string(),
    });
    selects.push(QuestionChoice {
        question: "Quit".to_string(),
        link: "Quit".to_string(),
    });
    let tmp: Vec<String> = selects.iter().map(|s| s.to_string()).collect();
    let checks: Vec<&str> = tmp.iter().map(|s| &**s).collect();
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your Question")
        .items(checks.as_slice())
        .interact()
        .unwrap();

    println!("You chose  :{}", selects[selection].question);
    let result = selects[selection].link.clone();
    return result;
}


/// Struct containing the question plus its url
pub struct QuestionChoice {
    pub question: String,
    pub link: String,
}
impl Display for QuestionChoice {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.question)
    }
}

pub trait VecExtension {
    fn to_spaced_string(&self) -> String;
}
impl VecExtension for Vec<String> {
    fn to_spaced_string(&self) -> String {
        let mut tmp = String::new();
        for i in self {
            tmp = tmp + i + " ";
        }
        tmp
    }
}

pub trait SplitToVec {
    fn split_to_vec(&self) -> Vec<String>;
}

impl SplitToVec for String {
    fn split_to_vec(&self) -> Vec<String> {
        let x = self.split_whitespace();
        let result: Vec<String> = x.map(|s| s.to_string()).collect();
        return result;
    }
}
