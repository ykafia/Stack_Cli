extern crate argparse;
extern crate dialoguer;
extern crate reqwest;
extern crate select;

mod request_builder;
use request_builder::*;
use argparse::{ArgumentParser, List, Store, StoreTrue};
use dialoguer::{theme::ColorfulTheme, Select, Input};

fn main() {
    let mut verbose = false;

    let mut keywords: Vec<String> = Vec::new();
    let mut tab: String = "Relevance".to_string();

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Enter your stackoverflow search keywords.");
        ap.refer(&mut keywords)
            .add_option(&["-k", "--keywords"], List, "List of keywords to find");
        ap.refer(&mut tab).add_option(
            &["-t", "--tab"],
            Store,
            "Type of research, either Relevance, Newest, Active",
        );
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue, "Verbosity");
        ap.parse_args_or_exit();
    }
    if keywords.len() == 0 {
        let input = Input::<String>::new().with_prompt("Your search").interact().unwrap();
        keywords = input.split_to_vec();
    }
    browser(keywords,tab)
}
fn browser(keywords: Vec<String>,tab:String) {
    let mut quit = false;
    let client = reqwest::Client::new();
    let mut pages:Vec<reqwest::Url> = Vec::new();
    pages.push(build_request(keywords, Some(tab), None));
    while !quit{
        stack_search(pages.last().unwrap(), &client);
    }
    
}



fn Question_Check(selects: Vec<QuestionChoice>) {
    let tmp: Vec<String> = selects.iter().map(|s| s.to_string()).collect();

    let checks: Vec<&str> = tmp.iter().map(|s| &**s).collect();

    let selections = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your Question")
        .items(checks.as_slice())
        .interact()
        .unwrap();

        println!("You chose  :\n\n{}", selects[selection].question);
    }
}
