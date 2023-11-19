use std::collections::HashMap;

use reqwest::Client;
use scraper::{Html, Selector};

const DUMB_URL: &str = "https://farside.link/dumb";

pub fn format_url(url: &str)->String{
    reqwest::Url::parse(url).unwrap().to_string()
}

async fn get_page_html(url: &str) -> String {
    let client = Client::new();
    let query = format_url(url);
    let res = client.get(query).send().await.unwrap();
    res.text().await.unwrap()
}

fn clean_text(text: &str)->String{
    text.replace(['\n', '\t'], "").trim().to_string()
}

pub async fn get_search_options(term: &str)->HashMap<usize, (String, String)>{
    let search_html = get_page_html(&format!("{DUMB_URL}/search?q={term}")).await;
    let search_doc = Html::parse_document(&search_html);

    let selector = Selector::parse("a#search-item").unwrap();

    let search_links = search_doc.select(&selector);
    let mut search_options = HashMap::new();

    for (i, link) in search_links.enumerate() {
        let link_text = clean_text(&link.text().collect::<Vec<_>>().join(""));
        let link_href = link.value().attr("href").unwrap().to_string();
        search_options.insert(i, (link_text, link_href));
    }

    search_options
}

pub async fn get_lyrics(slug: &str)->String{
    let lyrics_html = get_page_html(&format!("{DUMB_URL}{slug}")).await;

    let lyrics_doc = Html::parse_document(&lyrics_html);
    let selector = Selector::parse("div#lyrics").unwrap();

    let lyrics = lyrics_doc.select(&selector);

    let lyrics = lyrics
        .map(|lyric| lyric.text().collect::<Vec<_>>().join("\n"))
        .collect::<String>();

    //split by \"
    // let lyrics = lyrics.split("\"").collect::<Vec<_>>().join("\n");

    lyrics
}
