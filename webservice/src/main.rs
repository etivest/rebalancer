/*
 * Dual Licensed - GPL-3.0 and Proprietary Commercial License
 *
 * This file is part of the project available at https://github.com/etivest/repo.
 * Copyright (c) 2023 etivest.com. All rights reserved.
 *
 * This source code is licensed under the GNU General Public License version 3 (GPL-3.0)
 * as published by the Free Software Foundation. You may obtain a copy of the license
 * in the LICENSE-GPL-3.0 file or at https://www.gnu.org/licenses/gpl-3.0.txt.
 *
 * Commercial licensing options and terms are available. For more information,
 * please contact etivest.com at etivest@etivest.com.
 */

use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};
use bigdecimal::BigDecimal;

// Helper structs for JSON deserializing
#[derive(Serialize, Deserialize, Debug)]
pub struct Asset {
    pub name : String,
    pub current_amount : BigDecimal,
    pub target_percentage : BigDecimal, 
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct AssetList {
    list: Vec<Asset>,
}

#[post("/")]
async fn root_post(req: String) -> impl Responder {
    let json_result: Result<AssetList, serde_json::Error> = serde_json::from_str::<AssetList>(&req);
    match json_result {
        Ok(al) => {
            let mut ral = rebalance::AssetList {
            list: al.list.iter()
            .map(|orig| rebalance::Asset::new(
                orig.name.clone(),
                orig.current_amount.clone(),
                orig.target_percentage.clone(),
            ))
            .collect()
            };

            HttpResponse::Ok().body(rebalance::AssetList::rebalance(&mut ral).unwrap())
        },
        Err(e) => HttpResponse::Ok().body(e.to_string()),
        }
}

#[get("/")]
async fn root_get() -> impl Responder {
    HttpResponse::Ok().body("rebalancer")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting rebalancer webservice @ 0.0.0.0:8080");

    HttpServer::new(|| {
        App::new()
            .service(root_post)
            .service(root_get)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}