use backend::*;

#[get("/")]
async fn index() -> impl Responder {

    HttpResponse::Ok().json("We are live!")
}

#[actix_web::main]
async fn main() -> Result<()> {
    
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .service(index)
    })
    .bind("0.0.0.0:1337")?
    .run()
    .await?;

    Ok(())
}