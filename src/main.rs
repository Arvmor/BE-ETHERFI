use backend::*;

#[get("/")]
async fn index() -> impl Responder {

    HttpResponse::Ok().json("We are live!")
}

#[actix_web::main]
async fn main() -> Result<()> {

    // Terminal Logger
    let terminal_layer = layer()
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_target(false)
        .without_time()
        .with_line_number(false)
        .with_file(false)
        .with_filter(LevelFilter::INFO);

    Registry::default()
        .with(terminal_layer)
        .init();
    
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
            .service(index)
    })
    .bind("0.0.0.0:1337")?
    .run()
    .await?;

    Ok(())
}