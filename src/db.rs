use mongodb::{Client, 
    options::ClientOptions, 
    Database, 
    error::Error as MongoError,
    bson::{doc, Bson},
    options::FindOneOptions,
};

use crate::utils;

use  futures::stream::StreamExt;

pub struct DbProcessor {
    pub url: String,
}

impl  DbProcessor {
    pub async fn db_connect(&self) -> mongodb::error::Result<Database> {
        let uri = &self.url;
        println!("URI: {}", uri);
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
            let mut name_str: String;
            if let Some(name) = document.get("nama").and_then(Bson::as_str) {
                name_str = String::from(name);
            }
            else {
                name_str = String::from("");
            }
            let mut nominal: i64;
            if let Some(total) = document.get("nominal").and_then(Bson::as_i64) {
                nominal = total;
                total_u += nominal;
            }
            else {
                nominal = 0;
            }
            let money_i = nominal.to_string();
            let money = utils::convert_format_money(money_i);
            let str_fmt = format!("{}. {}\nRp {}\n\n", i, name_str, money);
            res.push(str_fmt);
            i += 1;
        }
        let total_str = utils::convert_format_money(total_u.to_string());
        Ok((res, total_str))
    }
}