use std::error::Error;


use crate::utils::r#macro::*;
use common_rs::err::core::*;

use common_rs::db::*;
use idl::protos::dbconn::DbConnRequest;
use idl::protos::dbconn::DbConnResponse;
use idl::protos::dbconn::db_conn_request::Body as RootBody;
use idl::protos::dbconn::db_conn_request_perm::Body as PermBody;
use idl::protos::dbconn::db_conn_response as ResBody;
use idl::protos::dbconn::db_conn_response_perm::Body as ResPermBody;
use idl::protos::dbconn::DbConnResponsePerm;
use idl::protos::dbconn::{SelectObjectPermResponse, SelectObjectPermResponses};

use crate::constant::perm as prem_querys;


#[inline]
fn decode_group_perm(read : bool, write : bool) -> i32 {
    let ret = (read as i32) | ((write as i32) << 1);
    ret
}

#[inline]
fn encode_group_perm(data : i32) -> (bool, bool) {
    let read = data & 0x00000001 == 1;
    let write = data & (0x00000001 << 1) == 1;

    (read, write)
}

pub(super) fn proto_perm_create_entry(conn : &mut Box<dyn CommonSqlConnection>, request: DbConnRequest) -> Result<DbConnResponse, Box<dyn Error>> {
    let perm_data  = enum_option_get_only_one!(RootBody, Perm, request.body)?;
    let create_data = enum_option_get_only_one!(PermBody, Create, perm_data.body)?; 

    conn.execute("begin", &[])?;
    let mut insert_ret = Ok(CommonSqlExecuteResultSet::default());
    for group in create_data.groups {
      
        insert_ret = conn.execute(prem_querys::INSERT_PERM_QUERY, &[
            CommonValue::String(create_data.object.clone()), 
            CommonValue::String(create_data.owner.clone()), 
            CommonValue::String(create_data.desc.clone()) ,
            CommonValue::Int(decode_group_perm(create_data.group_read, create_data.group_write)),
            CommonValue::String(group.clone())
        ]);

        if insert_ret.is_err() { break;}
    }

    if insert_ret.is_ok() {
        let _ = conn.execute("commin", &[]);
    } else {
        let _ = conn.execute("rollback", &[]);
        insert_ret?;
    }

    let mut ret = DbConnResponse::default();
    let mut body = DbConnResponsePerm::default();

    body.body = Some(ResPermBody::Dummy("".to_string()));
    ret.header = request.header;
    ret.body = Some(ResBody::Body::Perm(body));

    Ok(ret)
}

pub(super) fn proto_perm_drop_entry(conn : &mut Box<dyn CommonSqlConnection>, request: DbConnRequest) -> Result<DbConnResponse, Box<dyn Error>> {
    let perm_data  = enum_option_get_only_one!(RootBody, Perm, request.body)?;
    let drop_data = enum_option_get_only_one!(PermBody, Drop, perm_data.body)?; 

    if drop_data.body.is_none() {
        return create_error(COMMON_ERROR_CATEGORY, NO_DATA_ERROR, "".to_string()).as_error();
    }

    let data_and_query = match drop_data.body.as_ref().unwrap() {
        idl::protos::dbconn::drop_object_perm::Body::Object(o) => (o, prem_querys::DELETE_PERM_QUERY_FROM_OBJECT),
        idl::protos::dbconn::drop_object_perm::Body::Owner(o) => (o, prem_querys::DELETE_PERM_QUERY_FROM_OWNER),
        idl::protos::dbconn::drop_object_perm::Body::Groups(g) => (g, prem_querys::DELETE_PERM_QUERY_FROM_GROUP_ID),
    };

    conn.execute(data_and_query.1, &[CommonValue::String(data_and_query.0.clone())])?;

    let mut ret = DbConnResponse::default();
    let mut body = DbConnResponsePerm::default();

    body.body = Some(ResPermBody::Dummy("".to_string()));
    ret.header = request.header;
    ret.body = Some(ResBody::Body::Perm(body));

    Ok(ret)
}

pub(super) fn proto_perm_select_entry(conn : &mut Box<dyn CommonSqlConnection>, request: DbConnRequest) -> Result<DbConnResponse, Box<dyn Error>> {
    let perm_data  = enum_option_get_only_one!(RootBody, Perm, request.body)?;
    let select_data = enum_option_get_only_one!(PermBody, Select, perm_data.body)?; 

    let select_ret = if select_data.groups != "" {
        conn.execute(prem_querys::SELECT_PERM_QUERY_FROM_GROUP_ID, &[CommonValue::String(select_data.groups)])
    } else {
        conn.execute(prem_querys::SELECT_PERM_QUERY_FROM_OBJECT, &[CommonValue::String(select_data.object)])
    }?;

    let mut array = Vec::new();
    for x in select_ret.cols_data {
        let decode_perm = enum_get_only_one!(CommonValue, Int, x[2])?;

        let perms = encode_group_perm(decode_perm);
        let data = SelectObjectPermResponse {
            object : enum_get_only_one_ref!(CommonValue, String, x[0])?,
            groups : enum_get_only_one_ref!(CommonValue, String, x[1])?,
            group_read : perms.0,
            group_write : perms.1
        };

        array.push(data);  
    }

    let mut ret = DbConnResponse::default();
    let mut body = DbConnResponsePerm::default();
    let mut select_res = SelectObjectPermResponses::default();
    select_res.res = array;

    body.body = Some(ResPermBody::Select(select_res));
    ret.header = request.header;
    ret.body = Some(ResBody::Body::Perm(body));

    Ok(ret)
}