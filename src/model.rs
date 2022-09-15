use crate::schema::users;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Queryable, Insertable, Deserialize, Serialize)]
#[table_name = "users"]
pub struct User {
    pub email: String,
}
