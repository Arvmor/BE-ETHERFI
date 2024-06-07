use crate::*;

pub struct Auction {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub price: f32,
    pub end_date: String,
    pub image: String,
}