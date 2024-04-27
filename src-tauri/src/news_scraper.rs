use reqwest::blocking;
use rss::Channel;

pub struct News {
    pub title: Option<String>,
    pub time: Option<String>,
    pub url: Option<String>,
    pub description: Option<String>,
}

// --------------------------------------------------------------------------------------   WEBSCRAPED DATA FROM MONEYCONTROL.COM  --------------------------------------------------------------------------------------
// pub fn news_data_scraper() -> Vec<News> {
//     let url = "https://www.moneycontrol.com/news/business/stocks/";
//     let data = blocking::get(url).unwrap().text().unwrap();
//     let document = scraper::Html::parse_document(&data);

//     let mut news: Vec<News> = Vec::new();

//     let html_product_selector = scraper::Selector::parse("li.clearfix").unwrap();
//     let html_products = document.select(&html_product_selector);

//     for product in html_products {
//         let url = product
//             .select(&scraper::Selector::parse("a").unwrap())
//             .next()
//             .and_then(|a| a.value().attr("href"))
//             .map(str::to_owned);
//         let title = product
//             .select(&scraper::Selector::parse("a").unwrap())
//             .next()
//             .and_then(|a| a.value().attr("title"))
//             .map(str::to_owned);
//         let time = product
//             .select(&scraper::Selector::parse("span").unwrap())
//             .next()
//             .and_then(|span| span.text().next())
//             .map(str::to_owned);
//         let description = product
//             .select(&scraper::Selector::parse("p").unwrap())
//             .next()
//             .and_then(|p| p.text().next())
//             .map(str::to_owned);
//         let news_data = News {
//             title,
//             time,
//             url,
//             description,
//         };
//         news.push(news_data);
//     }
//     return news;
// }
// --------------------------------------------------------------------------------------   XXXXXXXXXXXXXXXXXXXXX  --------------------------------------------------------------------------------------

pub fn news_data_rss() -> Vec<News> {
    let url = "https://www.moneycontrol.com/rss/latestnews.xml";
    let data = blocking::get(url).unwrap().text().unwrap();
    let channel = Channel::read_from(data.as_bytes()).unwrap();
    let mut news: Vec<News> = Vec::new();
    for item in channel.items {
        let title = item.title.unwrap();
        let url = item.link.unwrap();
        let time = item.pub_date.unwrap();
        let description = item.description.unwrap();
        let news_data = News {
            title: Some(title),
            time: Some(time),
            url: Some(url),
            description: Some(description),
        };
        news.push(news_data);
    }
    return news;
}
