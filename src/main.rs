extern crate reqwest;
extern crate select;
extern crate dialoguer;
extern crate argparse;

use argparse::{ArgumentParser, List, Store, StoreTrue};
use dialoguer::{theme::ColorfulTheme, Checkboxes};
// use select::document::Document;
// use select::predicate::{Class, Name, Predicate};



fn main() {
    let mut verbose = false;
    // let stack_url = "https://stackoverflow.com/search?".to_string();
    let mut keywords:Vec<String> = Vec::new();
    let mut tab:String = "Relevance".to_string();
    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Greet somebody.");
        ap.refer(&mut keywords)
            .add_option(&["-k", "--kewords"], List,
            "List of keywords to find");
        ap.refer(&mut tab)
            .add_option(&["-t","--tab"], Store,
             "Type of research, either Relevance, Newest, Active");
        ap.refer(&mut verbose)
            .add_option(&["-v","--verbose"], StoreTrue,
            "Verbosity");
        ap.parse_args_or_exit();
    }

    if verbose {
        println!("Keywords are :");
        for i in keywords{
            println!("{}", i);
        }
        
    }
    dialogue();
}

//TODO: create request builder with https://stackoverflow.com/search? page=1 & tab=Relevance & q=rust%20functionnal
//TODO: list result of request on the console
//TODO: find all with classes "question-summary search-result" and display content
//TODO: find class "question" and "answer accepted-answer" and display content
// Note : To erase content of a line : \r does the job of overwriting

// fn hacker_news(url: &str) {

//     let resp = reqwest::get(url).unwrap();
//     assert!(resp.status().is_success());

//     let document = Document::from_read(resp).unwrap();

//     // finding all instances of our class of interest
//     for node in document.find(Class("athing")) {
//         // grabbing the story rank
//         let rank = node.find(Class("rank")).next().unwrap();
//         // finding class, then selecting article title
//         let story = node.find(Class("title").descendant(Name("a")))
//             .next()
//             .unwrap()
//             .text();
//         // printing out | rank | story headline
//         println!("\n | {} | {}\n", rank.text(), story);
//         // same as above
//         let url = node.find(Class("title").descendant(Name("a"))).next().unwrap();
//         // however, we don't grab text
//         // instead find the "href" attribute, which gives us the url
//         println!("{:?}\n", url.attr("href").unwrap());
//     }
// }

// fn stack_overflow(url:String, ){
    
// }



fn dialogue() {
    let checkboxes = &[
        "Ice Cream",
        "Vanilla Cupcake",
        "Chocolate Muffin",
        "A Pile of sweet, sweet mustard",
    ];
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