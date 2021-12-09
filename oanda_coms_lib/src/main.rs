use oanda_lib::oanda_coms_lib::OandaClient;
use std::env;

fn main() {

    let key : String = env::var("KEY").unwrap();
    let account : String = env::var("ACCOUNT").unwrap();

    let client = OandaClient::new(key, account);

    let output = client.get_instrument_candles("EUR_USD",25000,"M5");

    println!("{}", output[output.len() - 1]);
    println!("{}", output[0]);
    println!("{}", output.len());

    /*
    for item in output{

        println!("{}",item);

    }*/
}
