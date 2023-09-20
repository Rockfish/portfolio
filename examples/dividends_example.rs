use polygon_io_client_rust::common::Order;
use polygon_io_client_rust::dividends::{DividendFrequency, DividendRequest, DividendSort};
use polygon_io_client_rust::polygon_client::PolygonClientBuilder;

#[tokio::main]
async fn main() {
    let poly_client = PolygonClientBuilder::new().auth_key_env("POLYGON_AUTH_KEY").build().unwrap();

    let request = DividendRequest::new()
        .ticker("AAPL")
        .order(Order::Desc)
        .frequency(DividendFrequency::quarterly)
        .sort(DividendSort::Cash_Amount);

    println!("Request: {:#?}", request);

    let results = poly_client.get_dividends(&request).await;

    println!("results: {results:#?}\n");
}
