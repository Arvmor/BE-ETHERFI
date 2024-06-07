use crate::*;

/// Bid struct
#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Bid {
    #[serde(default)]
    pub id: Uuid,
    pub name: String,
    pub amount: i64,
    #[serde(default)]
    pub timestamp: i64,
}

impl From<Bid> for Bson {
    fn from(bid: Bid) -> Self {
        doc! {
            "id": bid.id,
            "name": bid.name,
            "amount": bid.amount,
            "timestamp": bid.timestamp
        }.into()
    }
    
}

/// Create new bid
#[post("/bids/{auction_id}")]
pub async fn create_bid(data: web::Data<AppState>,mut bid: web::Json<Bid>, auction_id: web::Path<String>) -> impl Responder {

    // Sanity check for Auction ID
    let auction_uuid = match Uuid::parse_str(auction_id.as_str()) {
        Ok(uuid) => uuid,
        Err(_) => return HttpResponse::BadRequest().json(APIResponse::fail("Invalid Auction ID"))
    };

    // Sanity check for Bid Amount
    if bid.amount <= 0 {
        return HttpResponse::BadRequest().json(APIResponse::fail("Bid amount must be greater than 0"));
    }

    // Assign a Timestamp
    bid.timestamp = Utc::now().timestamp();

    // Insert into DB
    match data.database.auctions.find_one_and_update(
        doc! { 
            "id": auction_uuid,
            "end_date": { "$gt": Utc::now().timestamp() },
            // winner is null or winner amount is less than bid amount
            "$or": [
                { "winner": { "$eq": null } },
                { "winner.amount": { "$lt": bid.amount } }
            ]
        },
        doc! {
            "$push": { "bids": bid.clone() },
            "$set": { "winner": bid.clone(),  }
        },
        Some(FindOneAndUpdateOptions::builder().return_document(ReturnDocument::After).build())
    ).await {
        Ok(result) => match result {
            Some(auction) => HttpResponse::Ok().json(APIResponse::success(auction)),
            None => HttpResponse::NotFound().json(APIResponse::fail("Bid is Invalid"))
        },
        Err(e) => {
            error!("Error fetching auction: {:?}", e);
            HttpResponse::InternalServerError().json(APIResponse::fail(e.to_string()))
        }
    }

}