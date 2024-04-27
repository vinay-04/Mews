use reqwest::blocking;
use scraper::{self, selectable::Selectable};

pub struct News {
    pub title: Option<String>,
    pub time: Option<String>,
    pub url: Option<String>,
}

pub fn news_data_scraper() -> Vec<News> {
    let url = "https://www.moneycontrol.com/news/business/stocks/";
    let data = blocking::get(url).unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&data);

    let mut news: Vec<News> = Vec::new();

    let html_product_selector = scraper::Selector::parse("li.clearfix").unwrap();
    let html_products = document.select(&html_product_selector);

    for product in html_products {
        let url = product
            .select(&scraper::Selector::parse("a").unwrap())
            .next()
            .and_then(|a| a.value().attr("href"))
            .map(str::to_owned);
        let title = product
            .select(&scraper::Selector::parse("a").unwrap())
            .next()
            .and_then(|a| a.value().attr("title"))
            .map(str::to_owned);
        let time = product
            .select(&scraper::Selector::parse("span").unwrap())
            .next()
            .and_then(|span| span.text().next())
            .map(str::to_owned);
        let news_data = News { title, time, url };
        news.push(news_data);
    }
    return news;
}
