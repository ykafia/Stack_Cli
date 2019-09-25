extern crate argparse;
extern crate dialoguer;
extern crate reqwest;
extern crate select;

mod utils;

use argparse::{ArgumentParser, List, /*Store,*/ StoreTrue};
use dialoguer::Input;
use utils::*;

use console::Term;

fn main() {
    let mut verbose = false;

    let mut keywords: Vec<String> = Vec::new();
    // let mut tab: String = "Relevance".to_string();

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Enter your stackoverflow search keywords.");
        ap.refer(&mut keywords)
            .add_option(&["-k", "--keywords"], List, "List of keywords to find");
        // ap.refer(&mut tab).add_option(
        //     &["-t", "--tab"],
        //     Store,
        //     "Type of research, either Relevance, Newest, Active",
        // );
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue, "Verbosity");
        ap.parse_args_or_exit();
    }
    if keywords.len() == 0 {
        let input = Input::<String>::new()
            .with_prompt("Your search")
            .interact()
            .unwrap();
        keywords = input.split_to_vec();
    }
    browser(keywords)
}

fn browser(keywords: Vec<String>) {
    let term = Term::stdout();
    let mut quit = false;
    let client = reqwest::Client::new();
    let mut req = build_request(keywords, None, None);
    let mut search_list = stack_search(&req, &client);
    while !quit {
        term.clear_screen().unwrap();
        match question_check(&mut search_list).as_str() {
            "Search Again" => {
                let input = Input::<String>::new()
                    .with_prompt("Your search")
                    .interact()
                    .unwrap();
                let kw = input.split_to_vec();
                req = build_request(kw, None, None);
                search_list = stack_search(&req, &client);
            }
            "Quit" => {
                quit = true;
                term.clear_screen().unwrap();
            }
            x => {
                let url =
                    reqwest::Url::parse(&("https://stackoverflow.com/".to_string() + x)).unwrap();
                match display_qa(&url, &client,&term).as_str() {
                    "Return" => {
                        let input = Input::<String>::new()
                            .with_prompt("Your search")
                            .interact()
                            .unwrap();
                        let kw = input.split_to_vec();
                        req = build_request(kw, None, None);
                        search_list = stack_search(&req, &client);
                    }
                    "Quit" => quit = true,
                    _ => {
                        println!("This choice wasn't supposed to happen :c");
                        quit = true;
                        term.clear_screen().unwrap();
                    }
                }
            }
        }
    }
}
