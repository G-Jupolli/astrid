use uuid::Uuid;

pub fn generate_uuid(upper_case: bool) {
    let mut uuid = Uuid::new_v4().to_string();

    if upper_case {
        uuid = uuid.to_uppercase();
    }

    println!("{}", uuid);

    match cli_clipboard::set_contents(uuid) {
        Ok(_) => {
            println!("Copied to clipboard");
        }
        Err(_) => {
            println!("Clipboard copy failure");
        }
    };
}
