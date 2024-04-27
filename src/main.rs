mod news_scraper;

fn main() {
    let news = news_scraper::news_data_rss();
    for n in news.iter() {
        println!("{:?}", n.title);
    }
}
