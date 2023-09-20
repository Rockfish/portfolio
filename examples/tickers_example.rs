use polygon_io_client_rust::polygon_client::*;
use polygon_io_client_rust::ticker_details::TickerDetailsRequest;
use polygon_io_client_rust::tickers::{Market, Order, TickersRequest, TickersSort};
use time::macros::date;

#[tokio::main]
async fn main() {
    let poly_client = PolygonClientBuilder::new().auth_key_env("POLYGON_AUTH_KEY").build().unwrap();

    let request = TickersRequest::new()
        .ticker("AAPL")
        .market(Market::Stocks)
        .exchange("XNAS")
        .date(date!(2023 - 06 - 28))
        .order(Order::Desc)
        .sort(TickersSort::Name);

    println!("Request: {:#?}", request);

    let results = poly_client.get_tickers(&request).await;

    println!("results: {results:#?}\n");

    let request = TickerDetailsRequest::new().ticker("BP").date(date!(2023 - 06 - 28));

    println!("Request: {:#?}", request);

    let results = poly_client.get_tickers_details(&request).await;

    println!("results: {results:#?}\n");
}
