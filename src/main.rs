extern crate argparse;
extern crate dialoguer;
extern crate reqwest;
extern crate select;

mod utils;

use dialoguer::{Input};
use argparse::{ArgumentParser, List, /*Store,*/ StoreTrue};
use utils::*;


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
// fn browser(keywords: Vec<String>, tab: String) {
//     //TODO : remove the link queue
//     //TODO : separate the search and the answer checking

//     let mut quit = false;
//     let client = reqwest::Client::new();
//     let mut pages: Vec<reqwest::Url> = Vec::new();

//     pages.push(build_request(keywords, Some(tab), None));
//     while !quit {
//         println!("first unwrap");
//         let last =  match pages.last(){
//             Some(x) => x,
//             None => {
//                 println!("Could not find last value");
//                 quit = true;
//                 &pages[0]
//             }
//         };
//         let mut search_list = stack_search(last, &client);
//         let choice = question_check(&mut search_list);
//         if choice == "Quit" {
//             quit = true;
//         } else if choice == "Result" {
//             pages.pop();
//         } else {
            
//             println!("link {}",choice);
//             match reqwest::Url::parse(&("https://stackoverflow.com/".to_string()+&choice)) {
//                 Ok(x) => pages.push(x),
//                 Err(e) => println!("The link couldn't be processed\nError : {}",e)
//             };
//         }
//     }
// }

fn browser(keywords: Vec<String>){
    let mut quit = false;
    let client = reqwest::Client::new();
    let mut req = build_request(keywords, None, None);
    let mut search_list = stack_search(&req, &client);
    while !quit{
        match question_check(&mut search_list).as_str(){
            "Search Again" => {
                let input = Input::<String>::new()
                    .with_prompt("Your search")
                    .interact()
                    .unwrap();
                let kw = input.split_to_vec();
                req = build_request(kw,None,None);
                search_list = stack_search(&req,&client);
            },
            "Quit" => quit = true,
            x =>  {
                let url = reqwest::Url::parse(&("https://stackoverflow.com/".to_string()+x)).unwrap();
                match display_qa(&url,&client).as_str() {
                    "Return" => {
                        let input = Input::<String>::new()
                            .with_prompt("Your search")
                            .interact()
                            .unwrap();
                        let kw = input.split_to_vec();
                        req = build_request(kw,None,None);
                        search_list = stack_search(&req,&client);
                    },
                    "Quit" => quit = true,
                    _ => {
                        println!("This choice wasn't supposed to happen :c");
                        quit =true;
                    }
                } 
            }
        }
    }
}

