
pub mod oanda_coms_lib{

    use json::JsonValue;
    use json::object;
    use std::fmt;
    use chrono::{DateTime, Utc, Duration};

    pub struct Candle{
        complete : bool,
        volume : i32,
        time : String,
        mid : Vec<f32>
    }

    impl Candle{

        pub fn get_complete(&self) -> &bool { &self.complete }
        pub fn get_volume(&self) -> &i32 { &self.volume }
        pub fn get_time(&self) -> &String {&self.time }
        pub fn get_mid(&self) -> &Vec<f32> {&self.mid }

    }

    impl fmt::Display for Candle{

        fn fmt(&self, f: &mut fmt::Formatter<'_> ) -> fmt::Result{

            let mut out_string = String::new();
            let string_vec : Vec<String> = self.mid.iter().map(|value| value.to_string()).collect();
            for item in string_vec{
                out_string.push_str(item.as_str());
                out_string.push(',');
            }

            write!(f, "{},{},{},{}", self.time, self.complete, self.volume, out_string)
        }
    }

    pub struct OandaClient{

        client : reqwest::Client,
        async_runtime : tokio::runtime::Runtime,
        key :  String,
        account :  String,

    }

    impl OandaClient{

        pub fn new(key : String, account : String) -> OandaClient{
            OandaClient{ client : reqwest::Client::new(),
                         async_runtime : tokio::runtime::Runtime::new().unwrap(),
                         key : key.to_string(),
                         account : account.to_string()}

        }

        async fn make_get_request(&self, url : String, auth_header : String) -> JsonValue{

            let res = self.client.get(url)
                                 .header("Content-Type", "application/json")
                                 .header("Authorization", auth_header)
                                 .send().await.unwrap();

             let body = res.text().await.unwrap();
             json::parse(&body).unwrap()
        }

        async fn make_post_request(&self, url : String, auth_header : String, body : String) -> JsonValue{

            let res = self.client.post(url)
                                 .header("Content-Type", "application/json")
                                 .header("Authorization", auth_header)
                                 .body(body)
                                 .send().await.unwrap();

             let body = res.text().await.unwrap();
             json::parse(&body).unwrap()
        }

        async fn make_put_request(&self, url : String, auth_header : String, body : String) -> JsonValue{

            let res = self.client.put(url)
                                 .header("Content-Type", "application/json")
                                 .header("Authorization", auth_header)
                                 .body(body)
                                 .send().await.unwrap();

             let body = res.text().await.unwrap();
             json::parse(&body).unwrap()
        }

        pub fn set_key(&mut self, key : String) -> (){

            self.key = key;

        }

        pub fn set_account(&mut self, account : String) -> (){

            self.account = account;

        }

        pub fn get_instrument_candle_range(&self, instrument : &str, from : &str, to : &str, granularity : &str) -> Vec<Candle>{

            let url = format!("https://api-fxpractice.oanda.com/v3/instruments/{}/candles?from={}&to={}&price=M&granularity={}", instrument, from, to, granularity);
            let auth_header = format!("Bearer {}", self.key);
            let mut output = self.async_runtime.block_on(self.make_get_request(url, auth_header));

            let candles : &JsonValue = &output.remove("candles");
            let candles : Vec::<Candle> = unpack_candles(candles);

            candles
        }

        pub fn get_instrument_candles(&self, instrument : &str, count : i32 ,granularity : &str) -> Vec<Candle>{

            let mut lasttime : i64;
            let mut string_holder : String;
            let mut url : String;
            let mut auth_header : String;
            let mut output;

            let mut remaning = count;
            let mut candles : Vec::<Candle> = Vec::with_capacity(remaning as usize);
            let mut temp_candles : Vec::<Candle>;
            let mut current_time = (chrono::offset::Local::now() - Duration::minutes(5)).timestamp();
            //println!("{}", current_time.timestamp());
            while remaning > 5000{

                url = format!("https://api-fxpractice.oanda.com/v3/instruments/{}/candles?count={}&price=M&to={}&granularity={}", instrument, 5000, current_time, granularity);
                auth_header = format!("Bearer {}", self.key);
                output = self.async_runtime.block_on(self.make_get_request(url, auth_header));
                temp_candles = unpack_candles(&output.remove("candles"));
                temp_candles.reverse();
                candles.extend(temp_candles);

                remaning = remaning - 5000;
                string_holder = candles[candles.len()-1].get_time().to_string();
                current_time = DateTime::parse_from_rfc3339(string_holder.as_str()).unwrap().timestamp() - 5;
            }

            if remaning > 0 {
                url = format!("https://api-fxpractice.oanda.com/v3/instruments/{}/candles?count={}&price=M&to={}&granularity={}", instrument, remaning, current_time ,granularity);
                auth_header = format!("Bearer {}", self.key);
                output = self.async_runtime.block_on(self.make_get_request(url, auth_header));
                temp_candles = unpack_candles(&output.remove("candles"));
                temp_candles.reverse();
                candles.extend(temp_candles);

            }

            candles

        }

        pub fn get_open_trades(&self) -> JsonValue{

            let url = format!("https://api-fxpractice.oanda.com/v3/accounts/{}/openTrades", self.account);
            let auth_header = format!("Bearer {}", self.key);
            let mut output = self.async_runtime.block_on(self.make_get_request(url, auth_header));

            output

        }

        pub fn post_order(&self, instrument : &str, units : i32 , order_type : &str) -> JsonValue {

            let url = format!("https://api-fxpractice.oanda.com/v3/accounts/{}/orders", self.account);
            let auth_header = format!("Bearer {}", self.key);

            let order : JsonValue = object!{
                "order" => object!{
                    "units" => units,
                    "instrument" => instrument,
                    "timeInForce" => "FOK",
                    "type" => order_type,
                    "positionFill" => "DEFAULT"

                }
            };

            let mut output = self.async_runtime.block_on(self.make_post_request(url, auth_header, order.to_string()));

            output

        }

        pub fn remove_order(&self, units : i32, trade_id : i32) -> JsonValue {

            let url = format!("https://api-fxpractice.oanda.com/v3/accounts/{}/trades/{}/close", self.account, trade_id);
            let auth_header = format!("Bearer {}", self.key);

            let transaction : JsonValue = object!{
                "units" => units.to_string()
            };

            let mut output = self.async_runtime.block_on(self.make_put_request(url, auth_header, transaction.to_string()));

            output

        }
    }

    pub fn unpack_candles(candles : &JsonValue) -> Vec<Candle>{

        let mut out_candles : Vec<Candle> = Vec::new();
        let mut mid : Vec<f32>;
        for item in candles.members(){
            mid = Vec::new();
            for value in item["mid"].entries(){
                mid.push(value.1.as_str().unwrap().parse::<f32>().unwrap())
            }

            out_candles.push(Candle { complete : item["complete"].as_bool().unwrap(),
                                      volume : item["volume"].as_i32().unwrap(),
                                      time : item["time"].as_str().unwrap().to_string(),
                                      mid : mid});
        }

        out_candles
    }
}
