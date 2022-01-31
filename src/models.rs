use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use super::schema::messages;
use super::schema::messages::dsl::messages as all_messages;

#[derive(Serialize, Queryable)] 
pub struct Message {
	pub id: i32,
	pub username: String,
	pub message: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "messages"]
pub struct NewMessage {
	pub username: String,
	pub message: String,
}

#[derive(Deserialize)] 
pub struct UserData {
	pub username: String,
	// pub message: Json;
}

impl Message {
	pub fn get_all_messages(conn: &PgConnection) -> Vec<Message> {
		all_messages
			.order(messages::id_message.desc())
			.load::<Message>(conn)
			.expect("error!")
	}
	pub fn insert_message(message: NewMessage, conn: &PgConnection) -> bool {
		diesel::insert_into(messages::table) 
			.values(&message)
			.execute(conn)
			.is_ok()
	}

	// pub fn delete_message(message: DeleteMessage, conn: &PgConnection) -> bool {
	// 	diesel::delete(messages::table) 
	// 		.values(&message)
	// 		.execute(conn)
	// 		.is_ok()
	// }
	
	pub fn get_message_by_username(message: UserData, conn: &PgConnection) -> Vec<Message> {
		all_messages
			.filter(messages::message.eq(message.username))
			.load::<Message>(conn)
			.expect("error!")
	}
}