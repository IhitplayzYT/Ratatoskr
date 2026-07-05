#[allow(dead_code,non_camel_case_types,non_snake_case)]

pub mod Conversion{
use std::{
    collections::HashMap,
    sync::RwLock,
};

use once_cell::sync::Lazy;
use rust_decimal::Decimal;
use serde::Deserialize;

use crate::model::app::App::Currency;

pub static CONVERSION_RATES: Lazy<RwLock<HashMap<Currency, Decimal>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

#[derive(Debug, Deserialize)]
struct ApiResponse {
    rates: HashMap<String, Decimal>,
}

pub async fn update_exchange_rates() -> anyhow::Result<()> {
    const URL: &str = "https://api.frankfurter.app/latest?from=USD";

    let response = reqwest::get(URL)
        .await?
        .json::<ApiResponse>()
        .await?;

    let mut map = HashMap::new();

    map.insert(Currency::USD, Decimal::ONE);

    for (code, rate) in response.rates {
        if let Ok(currency) = Currency::try_from(code.as_str()) {
            map.insert(currency, rate);
        }
    }

    *CONVERSION_RATES.write().unwrap() = map;

    Ok(())
}
}