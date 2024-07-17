use uuid::Uuid;

pub fn generate_uuid(count: i32, upper_case: bool, comma: bool) {
    let mut base_str = Uuid::new_v4().to_string();

    if upper_case {
        base_str = base_str.to_uppercase();
    }

    let mut amount = 2;

    let new_line_str = match comma {
        true => ",\n",
        false => "\n",
    };

    while amount <= count {
        let mut uuid = Uuid::new_v4().to_string();

        if upper_case {
            uuid = uuid.to_uppercase();
        }

        base_str.push_str(format!("{new_line_str}{}", uuid).as_str());

        amount += 1;
    }

    if comma {
        base_str.push(',');
    }

    println!("{}", base_str);

    match cli_clipboard::set_contents(base_str) {
        Ok(_) => {
            println!("Copied to clipboard");
        }
        Err(_) => {
            println!("Clipboard copy failure");
        }
    };
}
