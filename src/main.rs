use diesel::connection::SimpleConnection;
use diesel::prelude::*;
use diesel::SqliteConnection;

mod schema;
use schema::*;

fn main() {
	let conn = &mut SqliteConnection::establish("sqlite.db").unwrap();
	
	conn.batch_execute("
		PRAGMA foreign_keys = ON;
	").unwrap();
	
	let result = diesel::delete(parent::table.filter(parent::id.eq(1)))
		.execute(conn);
	
	let _ = dbg!(result);
}
