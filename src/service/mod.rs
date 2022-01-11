use mongodb::bson::doc;
use mongodb::Collection;
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize)]
pub struct Puzzle {
    puzzle_id: String,
    fen: String,
    moves: String,
    rating: i32,
    rating_deviation: i32,
    popularity: i32,
    n_plays: i32,
    themes: String,
    game_url: String,
}

#[derive(Clone)]
pub struct PuzzleService {
    collection: Collection<Puzzle>,
}

impl PuzzleService {
    pub fn new(collection: Collection<Puzzle>) -> PuzzleService {
        PuzzleService { collection }
    }
    // get a single puzzle from the collection
    pub async fn get_puzzle(&self) -> Result<Option<Puzzle>, mongodb::error::Error> {
        self.collection.find_one(doc! {}, None).await
    }
}
