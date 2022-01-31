use super::db::Conn as DbConn;
use rocket_contrib::json::Json;
use super::models::{Message, NewMessage};
use serde_json::Value;
use crate::models::UserData;

#[post("/username", format="application/json")]
pub fn get_all(conn: DbConn) -> Json<Value>{
	let messages = Message::get_all_messages(&conn);
	Json(json!({
		"status": 200,
		"result": messages,
	}))
}

#[post("/newMessage", format = "application/json", data = "<new_message>")]
pub fn new_message(conn: DbConn, new_message: Json<NewMessage>) -> Json<Value> {
	Json(json!({
		"status": Message::insert_message(new_message.into_inner(), &conn),
		"result": Message::get_all_messages(&conn).first(),
	}))
}

#[post("/getUser", format = "application/json", data = "<user_data>")]
pub fn find_username(user_data: Json<UserData>, conn: DbConn) -> Json<Value> {
	Json(json!({
		"status": 200,
		"result": Message::get_message_by_username(user_data.into_inner(), &conn),
	}))

}