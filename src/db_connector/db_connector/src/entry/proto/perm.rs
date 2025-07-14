use std::error::Error;

use common::common_make_err;
use idl::protos::dbconn::DbConnRequest;
use idl::protos::dbconn::DbConnResponse;
use common_conn::CommonSqlConnection;

pub(super) fn proto_perm_create_entry(mut conn : &Box<dyn CommonSqlConnection>, request: DbConnRequest) -> Result<DbConnResponse, Box<dyn Error>> {
    todo!()
}

pub(super) fn proto_perm_drop_entry(mut conn : &Box<dyn CommonSqlConnection>, request: DbConnRequest) -> Result<DbConnResponse, Box<dyn Error>> {
  todo!()
}

pub(super) fn proto_perm_select_entry(mut conn : &Box<dyn CommonSqlConnection>, request: DbConnRequest) -> Result<DbConnResponse, Box<dyn Error>> {
  todo!()
}