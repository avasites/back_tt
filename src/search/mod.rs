use actix_web::{Responder, web};

use crate::models::Teachers;
use crate::schema::teachers::dsl::teachers as schema_teachers;

use diesel::prelude::*;
use diesel::expression::sql_literal::sql;

use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct SearchByName{
    name: String
}

pub async fn get_teacher_by_name(form: web::Json<SearchByName>) -> impl Responder  {
 
    let mut vec:Vec<Teachers> = Vec::new();
 
    let connection = crate::establish_connection();
 
    let query = std::format!("name LIKE '{}%'", form.name).to_string();
 
    let result = schema_teachers.filter(sql(&query)).load::<Teachers>(&connection)
    .expect("Error loading posts");
 
    for teacher in result {
        vec.push(Teachers{name: teacher.name, id: teacher.id});
    }
 
    return web::Json(vec);
}