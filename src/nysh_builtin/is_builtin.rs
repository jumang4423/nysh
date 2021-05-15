// - check the given command is buildin or not
pub fn is_builtin(command: &String) -> bool {
    let available = [
        "exit".to_string(),
        "la".to_string(),
    ]
    .to_vec();

    if let Some(_) = available.iter().find(|&s| *s == *command) {
        true
    } else {
        false
    }
}
