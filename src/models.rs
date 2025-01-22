use diesel::{prelude::*, sql_types::Timestamp};
use crate::schema::posts;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

pub struct Control {
    pub business_id: i32,
    pub control_type: String,
    pub control_id: i32,
    pub entity_id: i32 
}

pub struct Relation {
    pub business_id: i32,
    pub control_type: String,
    pub parent_id: i32,
    pub entity_id: i32,
}

pub struct Channel {
    pub title: String,
    pub status: String,
    pub created_date: Timestamp,
    pub count: i32,
}

