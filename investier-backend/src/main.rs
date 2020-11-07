use actix_web::{web, get, post, middleware, App, HttpServer, HttpResponse};
use serde::{Deserialize};

use std::sync::Mutex;
use std::collections::HashMap;

mod stock;

use stock::*;

#[derive(Deserialize)]
struct StockPostData {
    symbol: String,
    amount: i32,
}

struct StockState {
    stocks: Mutex<HashMap<StockSymbol, Stock>>,
}

impl StockState {
    fn new() -> Self {
        Self {
            stocks: Mutex::new(HashMap::new()),
        }
    }
}

#[get("/")]
async fn index() -> String {
    String::from("ok")
}

#[post("/stocks")]
async fn post_stock(new_stock_data: web::Json<StockPostData>, data: web::Data<StockState>) -> String {
    let symbol = StockSymbol(String::from(new_stock_data.symbol.clone()));

    let mut stocks = data.stocks.lock().unwrap();
    let stock = stocks.get_mut(&symbol);
    match stock {
        Some(v) => {
            v.add_amount(new_stock_data.amount);
            format!("{} amount: {}", v.get_symbol(), v.get_amount())
        },
        None => {
            let stock = Stock::new(symbol.clone(), new_stock_data.amount);
            stocks.insert(symbol.clone(), stock);
            format!("New {} amount: {}",
                stocks.get(&symbol).unwrap().get_symbol(), new_stock_data.amount)
        },
    }
}

#[get("/stocks")]
async fn get_stocks(data: web::Data<StockState>) -> HttpResponse {
    let stocks = data.stocks.lock().unwrap();
    let stock_values = stocks.values().collect::<Vec<_>>();
    HttpResponse::Ok().json(stock_values)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(StockState::new());

    HttpServer::new(move || {
        // move counter into the closure
        App::new()
            .wrap(middleware::Compress::default())
            // Note: using app_data instead of data
            .app_data(state.clone()) // <- register the created data
            .service(post_stock)
            .service(get_stocks)
            .service(index)
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}