use rusqlite::{params, Connection, Result};

// select the last command use it with a resemblance
pub fn update_commands(val: String) {
    let conn = Connection::open("src/database/database.db").unwrap();
    let sql = "UPDATE commands SET last_time_used=strftime('%f', 'now')+ strftime('%s', 'now') WHERE command=?1;";
    conn.execute(sql, params![val]).unwrap();
}
pub fn insert_command(val: String) {
    let sql = "UPDATE commands SET last_time_used=strftime('%f', 'now')+ strftime('%s', 'now') WHERE command=?1;";
    let conn = Connection::open("src/database/database.db").unwrap();
    conn.execute(sql, params![val]).unwrap();
}
pub fn select_commands(val: String) -> String {
    let conn = Connection::open("src/database/database.db").unwrap();
    let sql = "SELECT command FROM commands WHERE command like ?1 ORDER BY last_time_used DESC  1;";

    let mut rows = conn.prepare(sql).unwrap().query(params![val+"%"]).unwrap();
    let mut command_output = String::new();
    while let Some(row) = rows.next().unwrap() {
        command_output = row.get(0).unwrap()
    }
    return command_output;
}