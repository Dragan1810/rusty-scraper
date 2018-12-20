extern crate reqwest;
extern crate scraper;
extern crate select;

use select::document::Document;
use select::predicate::{Predicate, Attr, Class, Name};

fn make_url(_stuff: &str) -> String {
    let sport = "soccer";
    let page = "matches";
    let date = "2018-12-17";

    let result = format!("http://www.scoresway.com/?sport={}&page={}&date={}", sport, page , date);
    result
}

fn main() {
    let _bin = "http://httpbin.org/";
    let _url1 = "http://www.blankwebsite.com/";
    let scores_way = "http://www.scoresway.com/?sport=home&page=matches&date=2018-12-17";

    let url: &str = &make_url(scores_way)[..];

    let res = reqwest::get(url).unwrap();
    let document = Document::from_read(res).unwrap();

    for node in document.find(Name("h3").descendant(Name("span"))) {
        println!("{:#?}", node.text());
    }

/*
    for node in document.find(Class("question-summary")).take(5) {
        let question = node.find(Class("question-hyperlink")).next().unwrap();
        let votes = node.find(Class("vote-count-post")).next().unwrap().text();
        let answers = node.find(Class("status").descendant(Name("strong")))
            .next()
            .unwrap()
            .text();
        let tags = node.find(Class("post-tag")).map(|tag| tag.text()).collect::<Vec<_>>();
        let asked_on = node.find(Class("relativetime")).next().unwrap().text();
        let asker = node.find(Class("user-details").descendant(Name("a")))
            .next()
            .unwrap()
            .text();
        println!(" Question: {}", question.text());
        println!("  Answers: {}", answers);
        println!("    Votes: {}", votes);
        println!("   Tagged: {}", tags.join(", "));
        println!(" Asked on: {}", asked_on);
        println!("    Asker: {}", asker);
        println!("Permalink: http://stackoverflow.com{}",
                 question.attr("href").unwrap());
        println!("");
    }

    for node in document.find(Attr("id", "h-related-tags"))
        .next()
        .unwrap()
        .parent()
        .unwrap()
        .find(Name("div"))
        .take(10) {
        let tag = node.find(Name("a")).next().unwrap().text();
        let count = node.find(Class("item-multiplier-count")).next().unwrap().text();
        println!("{} ({})", tag, count);
    }
*/

}