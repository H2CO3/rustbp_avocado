#![allow(unused_variables, unused_imports)]
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate avocado;
#[macro_use]
extern crate avocado_derive;
extern crate chrono;
extern crate semver;
extern crate textnonce;

use semver::Version;

// TODO(H2CO3): demonstrate prelude
use avocado::prelude::*;

use entities::*;
use util::{ generate_db_name, populate_database };

mod entities;
mod util;

// 1. Simple insertion of entities
fn simple_insertion(db: &Database) -> AvocadoResult<()> {
    println!("#1. Simple Insertion");
    println!("--------------------");

    // TODO(H2CO3): create User collection

    let u = User {
        id: Uid::new_oid()?,
        login: String::from("H2CO3"),
        name: String::from("Arpad"),
        github_profile: None,
    };

    // TODO(H2CO3): insert entity: let inserted_id = ...
    // println!("Inserted User entity with ID {}", inserted_id);

    println!();

    Ok(())
}

// 2. Collections & Simple Queries. Cursors, Type-Safe Literals.
fn simple_queries(db: &Database) -> AvocadoResult<()> {
    println!("#2. Simple Queries");
    println!("------------------");

    let crates: Collection<Crate> = db.existing_collection();

    // TODO(H2CO3): find crates with >= 100 downloads, then:
    /*
    for popular_crate in popular_crates {
        println!("Popular crate (>= 100 downloads): {:#?}", popular_crate?);
    }
     */

    use avocado::literal::RegexOpts;
    // TODO(H2CO3): find 1st batch of MongoDB-related crates
    // println!("First few MongoDB-related crates: {:#?}", first_batch);

    println!();

    Ok(())
}

// 3. Advanced Queries
fn advanced_queries(db: &Database) -> AvocadoResult<()> {
    println!("#3. Advanced Queries");
    println!("--------------------");

    #[derive(Debug, Clone)]
    struct CrateNamesForVersion(Version);

    // TODO(H2CO3): impl this impl
    /*
    impl Query<Crate> for CrateNamesForVersion {
        type Output = ...;

        fn filter(&self) -> Document {
            ...
        }

        fn transform(mut raw: Document) -> AvocadoResult<Bson> {
            ...
        }

        // Mention default impl of this method here:
        // just returns `<T as Doc>::query_options()`
        fn options() -> FindOptions {
            FindOptions {
                projection: Some(...),
                sort: Some(...),
                ..Default::default()
            }
        }
    }
     */
    let crates: Collection<Crate> = db.existing_collection();
    let version = Version::new(0, 3, 3);

    // TODO(H2CO3): then run this query
    // for crate_name in crates.find_many(CrateNamesForVersion(version))? {
    //     println!("Crate '{}'", crate_name?);
    // }

    println!();

    Ok(())
}

fn main() -> AvocadoResult<()> {
    let client = Client::with_uri("mongodb://188.166.16.189:39041/")?;
    let db_name = generate_db_name()?;
    let db = client.db(&db_name);

    println!("Using database {}", db_name);
    println!();

    // Pay no attention to that man behind the curtain!
    populate_database(&db)?;

    simple_insertion(&db)?;
    simple_queries(&db)?;
    advanced_queries(&db)?;

    Ok(())
}
