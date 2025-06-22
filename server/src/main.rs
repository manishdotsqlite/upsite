use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use crate::ping::ping;

mod response;
mod ping; 

#[actix_web::main]
async fn main() {
    HttpServer::new(|| {
        App::new()
        .service(pingapi) 
    }).bind("0.0.0.0:10001").unwrap().run().await.unwrap();
}


#[get("/{url}")]
async fn pingapi(path: web::Path<String>) -> impl Responder {
    let url = path.into_inner();
    println!("{:?}", url);
    let response = ping(&url);
    let return_response = match response {
        Ok(s) => s,
        Err(e) => e
    };
    let response_json = serde_json::to_string(&return_response).unwrap();
    HttpResponse::Ok().json(response_json)
}
