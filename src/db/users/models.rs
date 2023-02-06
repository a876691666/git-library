use diesel::prelude::*;
use super::schema::git_lists;
use serde::Serialize;

#[derive(Queryable, Insertable, Debug, Serialize)]
pub struct GitList {
    pub id: i32, // unique id
    pub name: String, // name of the item
    pub url: String, // url of the item
    pub description: String, // description of the item
    pub tags: String, // tags of the item
    pub is_deleted: i32, // is the item deleted
    pub created_at: i32, // create time
    pub updated_at: i32, // update time
    pub info: String, // detailed info of the item
    pub info_updated_at: i32, // update time of the detailed info
}
