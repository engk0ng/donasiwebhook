use crate::errors::{AppErrorType::*, AppError};
use crate::models::{Message};
use std::error::Error;
use async_std::task;
use surf;
use std::env;

use slog::{crit, o, Logger};

pub async fn update(req: &Message, log: Logger, token: String, command: String) -> Result<String, Box<dyn Error>> {
    let url_post = format!("https://api.telegram.org/bot{}{}", token, command);
    task::block_on(async {
        let res = surf::post(url_post)
        .body_json(req)?
        .await
        .map_err(|err|{
            let sublog = log.new(o!("cause" => err.to_string()));
            crit!(sublog, "Request failed");
            Box::new(AppError {cause: None, message: Some(err.to_string()), error_type: NotFoundError})
        });

        match res {
            Ok(mut r)  => {
                let r_str = &r.body_string().await.unwrap();
                Ok(r_str.into())
            },
            Err(e) => Err(e.into()),
        }
    })
}