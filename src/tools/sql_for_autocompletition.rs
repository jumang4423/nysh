use sqlite::{self, Value};
use std::io;
use std::time::SystemTime;
// select the last command use it with a resemblance
fn select_commands(val: String) {
    let conn = sqlite::open("src/database/database.db").unwrap();
    let mut stm = conn
        .prepare(
            "SELECT * FROM commands WHERE command like ?1 ORDER BY last_time_used DESC LIMIT 1;",
        )
        .unwrap()
        .into_cursor();
    // https://www.w3schools.com/sql/sql_wildcards.asp 
    // the like operator its like regex
    stm.bind(&[Value::String(val + &"%")]).unwrap();
    while let Some(row) = stm.next().unwrap() {
        println!("{:?}", row);
    }
}
// update the  last time use it
fn update_commands(val: String) {
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
    conn.prepare("UPDATE commands SET last_time_used=?1 WHERE command=?2;")
        .unwrap()
        .into_cursor()
        .bind(values_insert)
        .unwrap();
}
fn insert_command(val: String) {
    let conn = sqlite::open("src/database/database.db").unwrap();
    let mut stm = conn
        .prepare("INSERT INTO commands(command,last_time_used) SELECT ?1,?2  WHERE NOT EXISTS(SELECT 1 FROM commands WHERE  command = ?1);")
        .unwrap()
        .into_cursor();

    stm.bind(&[
        Value::String(val),
        Value::Integer(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,// im using this for get the time where the coommand has been use it
        ),
    ])
    .unwrap();
    // this look like shit but its necessary
    // for update the row because we are using select , and for some reason it
    // didnt function without this
    while let Some(_) = stm.next().unwrap() {}
}