extern crate reqwest;
extern crate scraper;
extern crate select;
extern crate tokio;


use select::document::Document;
use select::predicate::{Predicate, Attr, Class, Name};
use fantoccini::{Client, Locator};
use futures::future::Future;

fn make_url(_stuff: &str) -> String {
    let sport = "soccer";
    let page = "matches";
    let date = "2018-12-17";

    let result = format!("http://www.scoresway.com/?sport={}&page={}&date={}", sport, page , date);
    result
}

fn main() {
    let scores_way = "http://www.scoresway.com/?sport=home&page=matches&date=2018-12-17";
    let _bin = "http://httpbin.org/";
    let _url1 = "http://www.blankwebsite.com/";

    let url: &str = &make_url(scores_way)[..];
    let c = Client::new(url);

    let res = reqwest::get(url).unwrap();
    let document = Document::from_read(res).unwrap();

    for node in document.find(Name("h3").descendant(Name("span"))) {
        println!("{:#?}", node.text());
    }

    let server = c
        .map_err(|e| {
            unimplemented!("failed to connect to WebDriver: {:?}", e)
        })
        .and_then(|c| {
            // first, go to the Wikipedia page for Foobar
            c.goto("https://en.wikipedia.org/wiki/Foobar")
        })
        .and_then(|mut c| c.current_url().map(move |url| (c, url)))
        .and_then(|(mut c, url)| {
            assert_eq!(url.as_ref(), "https://en.wikipedia.org/wiki/Foobar");
            // click "Foo (disambiguation)"
            c.find(Locator::Css(".mw-disambig"))
        })
        .and_then(|e| e.click())
        .and_then(|mut c| {
            // click "Foo Lake"
            c.find(Locator::LinkText("Foo Lake"))
        })
        .and_then(|e| e.click())
        .and_then(|mut c| c.current_url())
        .and_then(|url| {
            assert_eq!(url.as_ref(), "https://en.wikipedia.org/wiki/Foo_Lake");
            Ok(())
        })
        .map_err(|e| {
            panic!("a WebDriver command failed: {:?}", e);
        });

tokio::run(server);

}