#![feature(trivial_bounds)]

use uuid::Uuid;
use diesel::SqliteConnection;
use crate::models::Dummy;
use crate::schema::dummy::dsl::dummy;

mod models;
mod schema;



fn get_dummy(id: Uuid, conn: &mut SqliteConnection){
    use diesel::prelude::*;

    dummy
        .find(&id)
        .first::<Dummy>(conn);
}

fn main() {

}
