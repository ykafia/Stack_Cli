extern crate argparse;
extern crate dialoguer;
extern crate reqwest;
extern crate select;

mod request_builder;
use request_builder::*;

use argparse::{ArgumentParser, List, Store, StoreTrue};
use dialoguer::{theme::ColorfulTheme, Checkboxes};
// use select::document::Document;
// use select::predicate::{Class, Name, Predicate};

fn main() {
    let mut verbose = false;

    let mut keywords: Vec<String> = Vec::new();
    let mut tab: String = "Relevance".to_string();

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Enter your stackoverflow search keywords.");
        ap.refer(&mut keywords)
            .add_option(&["-k", "--kewords"], List, "List of keywords to find");
        ap.refer(&mut tab).add_option(
            &["-t", "--tab"],
            Store,
            "Type of research, either Relevance, Newest, Active",
        );
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue, "Verbosity");
        ap.parse_args_or_exit();
    }

    // if verbose {
    //     println!("Keywords are :");
    //     for i in keywords{
    //         println!("{}", i);
    //     }

    // }

    // println!("{}",);
    let x = build_request(keywords, Some(tab), Some(1));
    println!("{}", x.to_string());
    stack_search(x);
}

//TODO: create request builder with https://stackoverflow.com/search? page=1 & tab=Relevance & q=rust%20functionnal
//TODO: list result of request on the console
//TODO: find all with classes "question-summary search-result" and display content
//TODO: find class "question" and "answer accepted-answer" and display content
// Note : To erase content of a line : \r does the job of overwriting

fn dialogue(checkboxes:Vec<String>) {
   
    let selections = Checkboxes::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your food")
        .items(&checkboxes[..])
        .interact()
        .unwrap();

    if selections.is_empty() {
        println!("You did not select anything :(");
    } else {
        println!("You selected these things:");
        for selection in selections {
            println!("  {}", checkboxes[selection]);
        }
    }
}
