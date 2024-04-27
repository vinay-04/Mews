mod news_scraper;

fn main() {
    let news = news_scraper::news_data_scraper();
    for n in news {
        println!("Title: {}", n.title.unwrap());
        println!("Time: {}", n.time.unwrap());
        println!("URL: {}", n.url.unwrap());
        println!();
    }
}
