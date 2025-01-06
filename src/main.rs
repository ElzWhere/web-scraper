use reqwest::Client; //

#[tokio::main]
async fn main() {
    let client = Client::new();

    let response = client.get("https://scrapeme.live/shop/")
    .send().await.unwrap();  // we're going to get stuff.

    let html_content = response.text().await.unwrap();  //response gives us txt

    let document = scraper::Html::parse_document(&html_content); // here is our document after being scraped

    let html_product_selector = scraper::Selector::parse("li.product").unwrap(); // we're snagging products form "li.product" html

    let html_products = document.select(&html_product_selector); // we're assigning products

    for product in html_products {      //we're going to iterate over this stuff

        let url = product
            .select(&scraper::Selector::parse("a").unwrap())
            .next()
            .and_then(|a|a.value().attr("href"))
            .map(str::to_owned);

        let image_url = product
            .select(&scraper::Selector::parse("img").unwrap())
            .next()
            .and_then(|a|a.value().attr("src"))
            .map(str::to_owned);

        let product_name = product
            .select(&scraper::Selector::parse("h2").unwrap())
            .next()
            .map(|h2|h2.text().collect::<String>());

        let product_price = product
            .select(&scraper::Selector::parse(".price").unwrap())
            .next()
            .map(|h2|h2.text().collect::<String>());

        println!("name = {:?}, price = {:?}, url = {:?}, image_url = {:?}",
        product_name, product_price, url, image_url)
    }

}