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
mod rebalance;

#[post("/")]
async fn root_post(req: String) -> impl Responder {
    HttpResponse::Ok().body(rebalance::rebalance(&req))
}

#[get("/")]
async fn root_get() -> impl Responder {
    HttpResponse::Ok().body("rebalancer")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting rebalancer webservice @ 127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
            .service(root_post)
            .service(root_get)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}