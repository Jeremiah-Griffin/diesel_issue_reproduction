use diesel::{Insertable, Queryable, Selectable};
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = crate::schema::dummy)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Dummy{
    #[diesel(serialize_as = Vec<u8>, deserialize_as = Uuid)]
    id: Uuid,
    inner: String,
}