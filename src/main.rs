use backend::*;

#[get("/")]
async fn index() -> impl Responder {

    HttpResponse::Ok().json("We are live!")
}

#[actix_web::main]
async fn main() -> Result<()> {

    // Load ENVs
    dotenv().ok();

    // Connect to DB
    let app_state = web::Data::new(AppState::new().await);

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
            .app_data(app_state.clone())
            // Base route
            .service(index)
            // Auction routes
            .service(get_auctions)
            .service(get_auction)
            .service(create_auction)
            .service(update_auction)
            .service(delete_auction)
    })
    .bind("0.0.0.0:1337")?
    .run()
    .await?;

    Ok(())
}

#[cfg(test)]
mod base_tests {
    use super::*;
    use actix_web::test;

    /// Test if the backend is live
    #[actix_web::test]
    async fn test_index() {
        
        // Send a GET request to / 
        let app = test::init_service(App::new().service(index)).await;
        let req = test::TestRequest::get().uri("/").to_request();
        
        // Response
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    /// Test if the database is connected
    #[actix_web::test]
    async fn test_database_status() {
        dotenv().ok();

        let app_state = AppState::new().await;

        assert!(app_state.database.auctions.count_documents(None, None).await.is_ok());
    }
}