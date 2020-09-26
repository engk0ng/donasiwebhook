use crate::errors::{AppError, AppErrorType::*};
use mongodb::{Client, 
    options::ClientOptions, 
    Database, 
    error::Error as MongoError,
    bson::{doc, Bson},
    options::FindOneOptions,
};

use std::error::Error;
use std::env;
use async_std::task;

use slog::{crit, o, Logger};

use std::result::Result;

use crate::models::{DataRekap, Rekap};

use crate::utils;

use  futures::stream::StreamExt;

pub struct DbProcessor {
    pub url: String,
}

impl  DbProcessor {
    pub async fn db_connect(&self) -> mongodb::error::Result<Database> {
        let uri = &self.url;
        //println!("URI: {}", uri);
        let split = uri.split("/");
        let arr = split.collect::<Vec<&str>>();
        
        let mut client_options = ClientOptions::parse(self.url.as_str()).await?;
        client_options.app_name = Some("My App".to_string());

        let client = Client::with_options(client_options).unwrap();
        let last_path = arr.last().unwrap();    
        let db = client.database(*last_path);

        Ok(db)  
    }

    pub async fn get_data_donasi(&self) -> mongodb::error::Result<(Vec<String>, String)> {
        let db = self.db_connect().await?;
        let collection = db.collection("rekap");
        let mut curson = collection.find(None, None).await?;

        let mut i: i32 = 1;
        let mut res = Vec::<String>::new();
        let mut total_u: i64 = 0;
        while let Some(result) = curson.next().await {
            let document = result.unwrap();
            let name_str: String;
            if let Some(name) = document.get("nama").and_then(Bson::as_str) {
                name_str = String::from(name);
            }
            else {
                name_str = String::from("");
            }
            let nominal: i64;
            if let Some(total) = document.get("nominal").and_then(Bson::as_i64) {
                nominal = total;
                total_u += nominal;
            }
            else {
                nominal = 0;
            }
            let money_i = nominal.to_string();
            //println!("{}", money_i);
            let money = utils::convert_format_money(money_i);
            let str_fmt = format!("{}. {}\nRp {}\n\n", i, name_str, money);
            res.push(str_fmt);
            i += 1;
        }
        let total_str = utils::convert_format_money(total_u.to_string());
        //println!("Total: {}", &total_str);
        Ok((res, total_str))
    }

    pub async fn rekap(&self, log: Logger) -> Result<(Vec<String>, String), Box<dyn Error>> {
        let token: String = match env::var("TOKEN") {
            Ok(s) => s,
            Err(e) => "".to_string(),
        };
    
        task::block_on(async {
            let res = surf::get(self.url.clone())
            .set_header("Access-Control-Allow-Origin", "*")
            .set_header("Content-Type", "application/json")
            .set_header("Authorization", token)
            .await
            .map_err(|err|{
                let sublog = log.new(o!("cause" => err.to_string()));
                crit!(sublog, "Request rekap failed");
                Box::new(AppError {cause: None, message: Some(err.to_string()), error_type: NotFoundError})
            });
    
            match res {
                Ok(mut r) => {
                    let str = &r.body_string().await.unwrap();
                    let jsn: DataRekap = serde_json::from_str(str.as_str())?;
                    let mut res = Vec::<String>::new();
                    if jsn.status == "Ok" {
                        let mut i: i32 = 1;
                        let mut total_u: i64 = 0;
                        for item in &jsn.data {
                            let name_str: String = item.name.clone();
                            let nominal: i64 = item.nominal;
                            total_u += nominal;
                            let money_i = nominal.to_string();
                            //println!("{}", money_i);
                            let money = utils::convert_format_money(money_i);
                            let str_fmt = format!("{}. {}\nRp {}\n\n", i, name_str, money);
                            res.push(str_fmt);
                            i += 1;
                        }
                        let total_str = utils::convert_format_money(total_u.to_string());
                        Ok((res, total_str))
                    }
                    else {
                        Ok((res, "".to_string()))
                    }
                },
                Err(e) => Err(e.into())
            }
        })
    }
}
