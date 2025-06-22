use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ReturnResponse {
    pub status: i32,
    pub message: String,
    pub data: Option<String>
}

