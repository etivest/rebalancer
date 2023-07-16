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

use actix_web::{post, App, HttpResponse, HttpServer, Responder};
mod rebalance;

#[post("/rb")]
async fn rb(req: String) -> impl Responder {
    HttpResponse::Ok().body(rebalance::rebalance(&req))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(rb)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}