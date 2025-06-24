use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use crate::ping::ping;

mod response;
mod ping; 

#[actix_web::main]
async fn main() {
    HttpServer::new(|| {
        App::new().wrap(Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
            )
        .service(pingapi) 
    }).bind("0.0.0.0:8000").unwrap().run().await.unwrap();
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
