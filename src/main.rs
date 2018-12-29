extern crate reqwest;
extern crate scraper;
extern crate select;
extern crate tokio;

use std::fmt;
use select::document::Document;
use select::predicate::{Predicate, Attr, Class, Name};
use fantoccini::{Client, Locator};
use futures::future::Future;

pub static scores_way: &str = "http://www.scoresway.com/?sport=home&page=matches&date=2018-12-17";

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

    let url: &str = &make_url(scores_way);
    let c = Client::new("http://localhost:4444");

    let res = reqwest::get(url).unwrap();
    let document = Document::from_read(res).unwrap();
    /*
    for node in document.find(Name("h3").descendant(Name("span"))) {
        println!("{:#?}", node.text());
    }
    */

    let server = c
        .map_err(|e| {
            unimplemented!("failed to connect to WebDriver: {:?}", e)
        })
        .and_then(move |c| c.goto(&make_url(scores_way)))
        .and_then(|mut c| {
            c.find_all(Locator::Css(".group-head.clickable"))
        })
        .and_then(|elements| {
            for el in elements {
                el.click();
            }

            Ok(())
            //img.attr("src").map(move |src| (img, src.expect("image should have a src")))
        })
        .and_then(|mut c| {
            //c.find_all(Locator::Css(".group-head.clickable.expanded.loaded"))
            // find new Content
            // get HTML
            // Profit

             Ok(())
        })
        .map_err(|e| {
            panic!("a WebDriver command failed: {:?}", e);
        });

tokio::run(server);

}