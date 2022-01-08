use bson::{doc, ordered::OrderedDocument};
use mongodb::{error::Error, Collection};

#[derive(Clone)]
pub struct PuzzleService {
    collection: Collection,
}

impl PuzzleService {
    pub fn new(collection: Collection) -> PuzzleService {
        PuzzleService { collection }
    }

    pub fn get(&self) -> Result<Option<OrderedDocument>, Error> {
        self.collection.find_one(doc! {}, None)
    }
}

