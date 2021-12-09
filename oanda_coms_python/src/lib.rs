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

    Ok(())
}

fn pack_into_map( candles : Vec<Candle> ) -> HashMap<String,Vec<String>>{

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
    Ok(pack_into_map(candles))
}

#[pyfunction]
fn get_instrument_candle_range(instrument : &str, from : &str, to : &str, granularity : &str) -> PyResult<HashMap<String,Vec<String>>> {

    let candles = client.get_instrument_candle_range( instrument, from, to, granularity);
    Ok(pack_into_map(candles))
}
