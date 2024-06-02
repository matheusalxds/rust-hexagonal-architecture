use actix_web::web::Json;
use serde::Serialize;

use crate::driving::rest_handler::errors::ApiError;

pub fn string_vec_to_vec_str(list: &Vec<String>) -> Vec<&str> {
    let mut pointer_list = Vec::<&str>::new();

    for item in list {
        pointer_list.push(item);
    }

    pointer_list
}

pub fn str_vec_to_vec_string(pointer_list: Vec<&str>) -> Vec<String> {
    let mut list = Vec::<String>::new();
    for item in pointer_list {
        list.push(String::from(item));
    }
    list
}

pub fn respond_json<T>(data: T) -> Result<Json<T>, ApiError>
where
    T: Serialize,
{
    Ok(Json(data))
}
