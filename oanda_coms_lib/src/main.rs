use oanda_lib::oanda_coms_lib::OandaClient;
use std::env;
use std::collections::HashMap;

fn main() {

    let key : String = env::var("KEY").unwrap();
    let account : String = env::var("ACCOUNT").unwrap();

    let client = OandaClient::new(key, account);

    //let output = client.get_instrument_candles("EUR_USD",70000,"M5");


    //println!("{}", client.post_order("EUR_USD",100,"MARKET")["orderCreateTransaction"]["id"]);

    let mut thingy = client.remove_order(100, 13);
    println!("{}", thingy);
    //println!("{}", thingy[0]);
    //println!("{}", thingy[0]);

    //let mut open_trades = &mut client.get_open_trades()["trades"];

    /*
    let mut trades = HashMap::<String,HashMap<String,String>>::new();
    let mut single_trade : HashMap<String,String>;

    for trade in thingy.members() {
        single_trade = HashMap::new();
        for item in trade.entries(){
            single_trade.insert(item.0.to_string(), item.1.to_string());
        }
        trades.insert(trade["id"].to_string(), single_trade);
    }


    for (key_1, value_1) in trades{

        for (key_2, value_2) in value_1.iter(){

            println!("{}, {}", key_2, value_2);

        }

    }
    */

    /*
    for item in output{

        println!("{}",item);

    }
    */
}
