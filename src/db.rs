use crate::*;

pub struct BECollection {
    pub auctions: Collection<Auction>,
}

pub struct AppState {
    pub database: BECollection,
}

impl AppState {
    pub async fn new() -> AppState {
        let options = ClientOptions::parse_with_resolver_config(&endpoint_mongodb(), ResolverConfig::cloudflare()).await.unwrap();
        let client = Client::with_options(options).unwrap();

        // Auctions Collection
        let auctions_collection: Collection<Auction> = client
            .database("AuctionDB")
            .collection("Auctions");

        AppState {
            database: BECollection {
                auctions: auctions_collection,
            }
        }
    }
}