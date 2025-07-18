
pub const CREATE_QUERYS : &'static [&'static str] = &[
    "CREATE TABLE IF NOT EXISTS object_perm (object VARCHAR(256), owner VARCHAR(128), desc VARCHAR(1024), perm INT4, group_id CHAR(20), PRIMARY KEY(object, group_id))",
    "CREATE INDEX IF NOT EXISTS object_perm_i1 ON object_perm (object, owner)"
];

pub const INSERT_PERM_QUERY : &'static str = "INSERT INTO object_perm(object, owner, desc, perm, group_id) VALUES ($1, $2, $3, $4, SHA1($5))";

pub const DELETE_PERM_QUERY_FROM_OBJECT : &'static str = "DELETE FROM object_perm WHERE object = $1";

pub const DELETE_PERM_QUERY_FROM_GROUP_ID : &'static str = "DELETE FROM object_perm WHERE group_id = SHA1($1)";

pub const DELETE_PERM_QUERY_FROM_OWNER : &'static str = "DELETE FROM object_perm WHERE onwer = $1";

pub const SELECT_PERM_QUERY_FROM_GROUP_ID : &'static str = "SELECT object, group_id, perm FROM object_perm WHERE group_id = SHA1($1)";

pub const SELECT_PERM_QUERY_FROM_OBJECT : &'static str = "SELECT object, group_id, perm FROM object_perm WHERE object = $1";
