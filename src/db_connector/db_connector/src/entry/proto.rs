mod perm;

use std::error::Error;

use common::common_make_err;
use idl::protos::dbconn::DbConnRequest;
use idl::protos::dbconn::DbConnResponse;
use common_conn::CommonSqlConnection;
use idl::protos::dbconn::db_conn_request::Body as RequestBody;

pub type ProtoEntryFn = &'static (dyn Fn(&mut Box<dyn CommonSqlConnection>, DbConnRequest) -> Result<DbConnResponse, Box<dyn Error>> + Send + Sync);

fn proto_search_perm_sub_entry(body : &RequestBody) -> Result<ProtoEntryFn, Box<dyn Error>>{
    let perm_req= match body {
        RequestBody::Perm(perm) => Ok(perm), 
        _ => common_make_err!(data, ParsingError, "perm parsing error")
    }?;

    if perm_req.body.is_none() {
        return common_make_err!(data, NoDataError, "not exists perm request data");
    }

    let f : ProtoEntryFn = match perm_req.body.as_ref().unwrap() {
        idl::protos::dbconn::db_conn_request_perm::Body::Create(_) => &perm::proto_perm_create_entry,
        idl::protos::dbconn::db_conn_request_perm::Body::Drop(_) => &perm::proto_perm_drop_entry,
        idl::protos::dbconn::db_conn_request_perm::Body::Select(_) => &perm::proto_perm_select_entry,
    };

    Ok(f)
}

pub fn search_proto_entry(action : &'_ str, request : &DbConnRequest) -> Result<ProtoEntryFn, Box<dyn Error>> {
    if request.header.is_none() || request.body.is_none() {
        return common_make_err!(data, NoDataError, "not exists request data");
    }

    let header = request.header.as_ref().unwrap();

    if action != header.action.as_str() {
        return common_make_err!(system, NoSupportError, "not support action:{}", action);
    }
    
    let sub_entry = match action {
        "PERM" => Ok(proto_search_perm_sub_entry(&request.body.as_ref().unwrap())?),
        _ => common_make_err!(system, NoSupportError, "not support action:{}", header.action.as_str())
    }?;

    Ok(sub_entry)
}