extern crate reqwest;
extern crate scraper;
extern crate select;
extern crate tokio;

use std::fmt;
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
        .and_then(|c| c.goto("https://www.wikipedia.org/"))
        .and_then(|mut c| {
        // find the source for the Wikipedia globe
        c.find(Locator::Css("img.central-featured-logo"))
        })
        .and_then(|mut img| {
            img.attr("src").map(move |src| (img, src.expect("image should have a src")))
        })
        .and_then(move |(img, src)| {
        // now build a raw HTTP client request (which also has all current cookies)
            img.client().raw_client_for(fantoccini::Method::GET, &src)
        })
        .and_then(|raw| {
            use futures::Stream;
        // we then read out the image bytes
            raw.into_body().map_err(fantoccini::error::CmdError::from).fold(
                Vec::new(),
                |mut pixels, chunk| {
                pixels.extend(&*chunk);
                futures::future::ok::<Vec<u8>, fantoccini::error::CmdError>(pixels)
                },
            )
    })
    .and_then(|pixels| {
        // and voilla, we now have the bytes for the Wikipedia logo!
        assert!(pixels.len() > 0);
        println!("Wikipedia logo is {}b", pixels.len());
        Ok(())
    })
        .map_err(|e| {
            panic!("a WebDriver command failed: {:?}", e);
        });

tokio::run(server);

}