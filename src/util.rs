use std::fs::File;
use textnonce::TextNonce;
use serde_json::from_reader;
use avocado::prelude::*;
use crate::entities::*;

pub fn generate_db_name() -> AvocadoResult<TextNonce> {
    TextNonce::sized_urlsafe(32).map_err(
        |msg| AvocadoError::new(AvocadoErrorKind::MongoDbError, msg)
    )
}

pub fn populate_database(db: &Database) -> AvocadoResult<()> {
    populate_collection(db.existing_collection::<Crate>(), "data/crates.json")?;
    populate_collection(db.existing_collection::<User>(), "data/users.json")?;
    Ok(())
}

fn populate_collection<T: Doc>(coll: Collection<T>, file: &str) -> AvocadoResult<()> {
    let file = File::open(file).map_err(
        |e| AvocadoError::new(AvocadoErrorKind::JsonTranscoding, e.to_string())
    )?;
    let docs: Vec<T> = from_reader(file)?;

    coll.insert_many(&docs)?;

    Ok(())
}
