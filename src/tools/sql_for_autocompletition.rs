use sqlite::{self, Value};
use std::io;
use std::time::SystemTime;
// select the last command use it with a resemblance
pub fn select_commands(val: String) {
    let sql ="SELECT command FROM commands WHERE command like ?1 ORDER BY last_time_used DESC --LIMIT 1;";
    let conn = sqlite::open("src/database/database.db").unwrap();
    let mut stm = conn.prepare(sql).unwrap().into_cursor();
    // https://www.w3schools.com/sql/sql_wildcards.asp
    // the like operator its like regex

    stm.bind(&[Value::String(val + &"%")]).unwrap();
    while let Some(row) = stm.next().unwrap() {
        println!("{:?}", row[0].as_string());
    }
}
// update the  last time use it
pub fn update_commands(val: String) {
    let sql = "UPDATE commands SET last_time_used=?1 WHERE command=?2;";
    let values_insert = &[
        Value::Integer(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
        ),
        Value::String(val + "%"),
    ];
    let conn = sqlite::open("src/database/database.db").unwrap();
    let mut stm = conn.prepare(sql).unwrap().into_cursor();
    stm.bind(values_insert).unwrap();
    while let Some(_) = stm.next().unwrap() {}
}
// insert the command to the database
pub fn insert_command(val: String) {
    let sql ="INSERT INTO commands(command,last_time_used) SELECT ?1,?2  WHERE NOT EXISTS(SELECT 1 FROM commands WHERE  command = ?1);";
    let conn = sqlite::open("src/database/database.db").unwrap();
    let mut stm = conn.prepare(sql).unwrap().into_cursor();

    stm.bind(&[
        Value::String(val),
        Value::Integer(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64, // im using this for get the time where the coommand has been use it
        ),
    ])
    .unwrap();
    // this look like shit but its necessary
    // for update the row because we are using select , and for some reason it
    // didnt function without this
    while let Some(_) = stm.next().unwrap() {}
}
