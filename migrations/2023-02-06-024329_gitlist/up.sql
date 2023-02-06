-- Your SQL goes here
CREATE TABLE git_lists (
  id INTEGER PRIMARY KEY, -- 主键
  name TEXT UNIQUE NOT NULL, -- GIT名称，唯一，不能为 NULL，如: {account}/{projectName}
  url TEXT UNIQUE NOT NULL, -- 地址，唯一，不能为 NULL, 如: https://github.com/{account}/{projectName}.git
  description TEXT, -- 描述
  tags TEXT, -- 标签, 逗号分隔
  is_deleted INTEGER DEFAULT 0, -- 是否删除，0: 未删除，1: 已删除
  created_at INTEGER, -- 创建时间
  updated_at INTEGER, -- 更新时间
  info TEXT, -- 仓库信息, 包含 stargazers_count, watchers_count, forks_count, open_issues_count, topics
  info_updated_at INTEGER -- 仓库信息更新时间
);