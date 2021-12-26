use oanda_lib::oanda_coms_lib::{OandaClient, Candle};

use pyo3::prelude::*;

use std::env;
use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static!{
    static ref client : OandaClient = {
        let account : String = env::var("ACCOUNT").unwrap();
        let key : String = env::var("KEY").unwrap();
        OandaClient::new(key, account)
        };
}

#[pymodule]
fn oanda_coms_lib(_py : Python, m: &PyModule) -> PyResult<()> {

    m.add_function(wrap_pyfunction!(get_instrument_candles, m)?)?;
    m.add_function(wrap_pyfunction!(get_instrument_candle_range, m)?)?;
    m.add_function(wrap_pyfunction!(get_open_trades, m)?)?;
    m.add_function(wrap_pyfunction!(post_order, m)?)?;
    m.add_function(wrap_pyfunction!(remove_order, m)?)?;

    Ok(())
}

fn pack_candles_into_map( candles : Vec<Candle> ) -> HashMap<String,Vec<String>>{

    let mut output_map = HashMap::<String,Vec<String>>::new();

    let mut time_vec : Vec<String> = Vec::new();
    let mut volume_vec : Vec<String> = Vec::new();
    let mut close_vec : Vec<String> = Vec::new();
    let mut high_vec : Vec<String> = Vec::new();
    let mut low_vec : Vec<String> = Vec::new();
    let mut open_vec : Vec<String> = Vec::new();

    for candle in candles{

        time_vec.push(candle.get_time().to_owned());
        volume_vec.push(candle.get_volume().to_string());
        close_vec.push(candle.get_mid()[0].to_string());
        high_vec.push(candle.get_mid()[1].to_string());
        low_vec.push(candle.get_mid()[2].to_string());
        open_vec.push(candle.get_mid()[3].to_string());

    }

    output_map.insert("time".to_string(),time_vec);
    output_map.insert("volume".to_string(),volume_vec);
    output_map.insert("close".to_string(),close_vec);
    output_map.insert("high".to_string(),high_vec);
    output_map.insert("low".to_string(),low_vec);
    output_map.insert("open".to_string(),open_vec);

    output_map

}

#[pyfunction]
fn get_instrument_candles(instrument : &str, count : i32 ,granularity : &str) -> PyResult<HashMap<String,Vec<String>>> {

    let candles = client.get_instrument_candles( instrument, count, granularity);

    Ok(pack_candles_into_map(candles))
}

#[pyfunction]
fn get_instrument_candle_range(instrument : &str, from : &str, to : &str, granularity : &str) -> PyResult<HashMap<String,Vec<String>>> {

    let candles = client.get_instrument_candle_range( instrument, from, to, granularity);
    Ok(pack_candles_into_map(candles))
}

#[pyfunction]
fn get_open_trades() -> PyResult<HashMap<String,HashMap<String,String>>> {

    let mut open_trades = &mut client.get_open_trades()["trades"];

    let mut output_trades = HashMap::<String,HashMap<String,String>>::new();
    let mut single_trade : HashMap<String,String>;

    for trade in open_trades.members() {
        single_trade = HashMap::new();
        for item in trade.entries(){
            single_trade.insert(item.0.to_string(), item.1.to_string());
        }
        output_trades.insert(trade["id"].to_string(), single_trade);
    }

    Ok(output_trades)
}


#[pyfunction]
fn post_order(instrument : &str, units : i32 , order_type : &str) -> PyResult<String> {

    let transaction_id = client.post_order( instrument, units, order_type)["orderCreateTransaction"]["id"].to_string();

    Ok(transaction_id)
}

#[pyfunction]
fn remove_order(units : i32 , trade_id : i32) -> PyResult<HashMap<String,String>> {

    let transaction_id = client.remove_order(units, trade_id);

    let mut output_dict : HashMap<String,String> = HashMap::<String,String>::new();

    if(!transaction_id["orderCancelTransaction"].is_null()){

        output_dict.insert("orderCancelTransaction".to_string(), transaction_id["orderCancelTransaction"]["reason"].to_string());

    } else{

        output_dict.insert("orderCreateTransaction".to_string(), transaction_id["orderCreateTransaction"]["id"].to_string());

    }


    Ok(output_dict)
}
