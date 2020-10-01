use serde::{Serialize, Deserialize};


#[derive(Queryable, Deserialize, Serialize)]
pub struct Groups {
    pub id: i32,
    pub name: String
}

#[derive(Queryable, Deserialize, Serialize)]
pub struct Teachers {
    pub id: i32,
    pub name: String
}