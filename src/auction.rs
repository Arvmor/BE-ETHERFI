use crate::*;

/// Auction struct
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Auction {
    #[serde(default)]
    pub id: Uuid,
    #[serde(default)]
    pub status: AuctionStatus,
    pub name: String,
    #[serde(default)]
    pub bids: HashMap<u64, Bid>,
    #[serde(default)]
    pub starting_price: u64,
    pub end_date: i64,
}

/// Auction status
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum AuctionStatus {
    #[default]
    Active,
    Ended,
}

/// Bid struct
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Bid {
    pub id: Uuid,
    pub name: String,
    pub amount: u64,
    pub timestamp: i64,
}


/// Create new auction
#[post("/auctions")]
pub async fn create_auction(data: web::Data<AppState>,mut auction: web::Json<Auction>) -> impl Responder {

    // Sanity check for Auction End Date
    if auction.end_date <= Utc::now().timestamp() {
        return HttpResponse::BadRequest().json(APIResponse::fail("End date cannot be in the past"));
    }
    
    // Assign a UUID
    auction.id = Uuid::new();

    // Insert into DB
    match data.database.auctions.insert_one(
        auction.clone(), 
        None
    ).await {
        Ok(_) => HttpResponse::Created().json(APIResponse::success(auction)),
        Err(e) => {
            error!("Error creating auction: {:?}", e);
            HttpResponse::InternalServerError().json(APIResponse::fail(e.to_string()))
        }
    }

}

/// Update an auction
#[patch("/auctions/{id}")]
pub async fn update_auction(data: web::Data<AppState>, id: web::Path<String>, auction: web::Json<Auction>) -> impl Responder {

    // Sanity check for Auction ID
    let auction_uuid = match Uuid::parse_str(id.as_str()) {
        Ok(uuid) => uuid,
        Err(_) => return HttpResponse::BadRequest().json(APIResponse::fail("Invalid Auction ID"))
    };

    // Update the auction
    match data.database.auctions.find_one_and_replace(
        doc! {"id": auction_uuid}, 
        auction.into_inner(), 
        None
    ).await {
        Ok(result) => match result {
            Some(auction) => HttpResponse::Ok().json(APIResponse::success(auction)),
            None => HttpResponse::NotFound().json(APIResponse::fail("Auction not found"))
        },
        Err(e) => {
            error!("Error updating auction: {:?}", e);
            HttpResponse::InternalServerError().json(APIResponse::fail(e.to_string()))
        }
    }

}

/// Delete an auction
#[delete("/auctions/{id}")]
pub async fn delete_auction(data: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
    // Sanity check for Auction ID
    let auction_uuid = match Uuid::parse_str(id.as_str()) {
        Ok(uuid) => uuid,
        Err(_) => return HttpResponse::BadRequest().json(APIResponse::fail("Invalid Auction ID"))
    };

    // Delete the auction
    match data.database.auctions.find_one_and_delete(
        doc! {"id": auction_uuid},
        None
    ).await {
        Ok(result) => match result {
            Some(auction) => HttpResponse::Ok().json(APIResponse::success(auction)),
            None => HttpResponse::NotFound().json(APIResponse::fail("Auction not found"))
        },
        Err(e) => {
            error!("Error deleting auction: {:?}", e);
            HttpResponse::InternalServerError().json(APIResponse::fail(e.to_string()))
        }
    }
}

/// Get all auctions
#[get("/auctions")]
pub async fn get_auctions(data: web::Data<AppState>) -> impl Responder {

    let mut all_auctions: Vec<Auction> = Vec::new();

    // Get all transactions
    let mut cursor = match data.database.auctions.find(None, None).await {
        Ok(cursor) => cursor,
        Err(e) => {
            error!(?e);
            return HttpResponse::InternalServerError().json(APIResponse::fail(format!("Database failed fetch auctions {e}")))
        }
    };

    while cursor.advance().await.unwrap() {
        all_auctions.push(cursor.deserialize_current().unwrap());
    }

    HttpResponse::Ok().json(APIResponse::success(json!({
        "count": all_auctions.len(),
        "auctions": all_auctions
    })))
}

/// Get a single auction
#[get("/auctions/{id}")]
pub async fn get_auction(data: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
    // Sanity check for Auction ID
    let auction_uuid = match Uuid::parse_str(id.as_str()) {
        Ok(uuid) => uuid,
        Err(_) => return HttpResponse::BadRequest().json(APIResponse::fail("Invalid Auction ID"))
    };

    // Get the auction
    match data.database.auctions.find_one(
        doc! {"id": auction_uuid},
        None
    ).await {
        Ok(result) => match result {
            Some(auction) => HttpResponse::Ok().json(APIResponse::success(auction)),
            None => HttpResponse::NotFound().json(APIResponse::fail("Auction not found"))
        },
        Err(e) => {
            error!("Error fetching auction: {:?}", e);
            HttpResponse::InternalServerError().json(APIResponse::fail(e.to_string()))
        }
    }
}

#[cfg(test)]
mod auction_tests {
    use super::*;
    use actix_web::{http::StatusCode, test};
    use serde_json::Value;

    /// Test creating a new auction
    #[actix_web::test]
    async fn test_create_auction() {

        // init
        dotenv().ok();
        let app_state = web::Data::new(AppState::new().await);
        let app = test::init_service(App::new().service(create_auction).app_data(app_state)).await;
        
        // Create a new auction
        let new_auction = Auction {
            name: "Test Auction".to_string(),
            starting_price: 100,
            end_date: Utc::now().timestamp() + 1000,
            .. Default::default()
        };

        // Send a POST request to /auctions
        let req = test::TestRequest::post()
            .uri("/auctions")
            .set_json(&new_auction)
            .to_request();
        
        // Sanity Check the Response
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body: APIResponse<Auction> = test::read_body_json(resp).await;
        assert_eq!(body.message, Auction { id: body.message.id, ..new_auction } );
    }

    /// Test updating an auction
    #[actix_web::test]
    async fn test_update_auction() {
        // init
        dotenv().ok();
        let app_state = web::Data::new(AppState::new().await);
        let app = test::init_service(App::new().service(update_auction).app_data(app_state.clone())).await;
 
        // Create a new auction
        let new_auction = Auction {
            name: "Test Auction".to_string(),
            starting_price: 100,
            end_date: Utc::now().timestamp() + 1000,
            .. Default::default()
        };

        // Insert into DB
        app_state.database.auctions.insert_one(new_auction.clone(), None).await.unwrap();
    }

    /// Test deleting an auction
    #[actix_web::test]
    async fn test_delete_auction() {
        // init
        dotenv().ok();
        let app_state = web::Data::new(AppState::new().await);
        let app = test::init_service(App::new().service(delete_auction).app_data(app_state.clone())).await;
        
        // Create a new auction
        let new_auction = Auction {
            name: "Test Auction".to_string(),
            starting_price: 100,
            end_date: Utc::now().timestamp() + 1000,
            .. Default::default()
        };

        // Insert into DB
        app_state.database.auctions.insert_one(new_auction.clone(), None).await.unwrap();

        // Send a DELETE request to /auctions/{id}
        let req = test::TestRequest::delete()
            .uri(&format!("/auctions/{}", new_auction.id))
            .to_request();
        
        // Sanity Check the Response
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body: APIResponse<Auction> = test::read_body_json(resp).await;
        assert_eq!(body.message, new_auction);


        // test a faulty delete
        let req = test::TestRequest::delete()
        .uri(&format!("/auctions/{}", Uuid::new()))
        .to_request();
        
        // Sanity Check the Response
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        let body: APIResponse<Value> = test::read_body_json(resp).await;
        assert_eq!(body.status, -1);

    }

    /// Test getting all auctions
    #[actix_web::test]
    async fn test_get_auctions() {
        // init
        dotenv().ok();
        let app_state = web::Data::new(AppState::new().await);
        let app = test::init_service(App::new().service(get_auctions).app_data(app_state.clone())).await;
        
        // Send a GET request to /auctions
        let req = test::TestRequest::get().uri("/auctions").to_request();
        
        // Sanity Check the Response
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body: APIResponse<Value> = test::read_body_json(resp).await;
        assert!(body.message["count"].as_i64().unwrap() >= 0);
    }
}