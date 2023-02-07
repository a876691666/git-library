use super::schema::git_lists;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Insertable, Debug, Serialize)]
pub struct GitList {
    /// 条目id
    pub id: i32,
    /// Git项目名称
    pub name: String,
    /// Git项目地址
    pub url: String,
    /// 描述
    pub description: String,
    /// 标签
    pub tags: String,
    /// 是否删除, 0: 未删除, 1: 已删除
    pub is_deleted: i32,
    /// 创建时间
    pub created_at: i32,
    /// 更新时间
    pub updated_at: i32,
    /// Git项目信息
    pub info: String,
    /// Git项目信息更新时间
    pub info_updated_at: i32,
}
