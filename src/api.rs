use crate::menu::eval;

pub fn get_word() -> String{
    setup_mode();

    let url = match eval("easy").as_str(){
        "0" => generate_hard_url(),
        "1" => generate_easy_url(),
        _ => panic!("How the fuck")
    };

    match reqwest::blocking::get(url).unwrap().text() {
        Ok(value) => word_cleanup(value),
        Err(_) => get_word()
    }

}

fn word_cleanup(s: String) -> String{
    let mut s = s.clone();
    for _ in 0..2 {
        s.remove(0);
        s.pop();
    }
    s
}


fn setup_mode() {
    let db_connection = sled::open("data").unwrap();
    if db_connection.get("easy").unwrap().is_none() {
        let _ = db_connection.insert("easy", "0");
    }
    drop(db_connection);
}


fn generate_easy_url() -> String {
    match eval("length").as_str() {
        "0" => String::from("https://random-word-api.vercel.app/api?words=1"),
        _ => format!("https://random-word-api.vercel.app/api?words=1&length={}",  eval("length").as_str()),
    }
}

fn generate_hard_url() -> String {
    match eval("length").as_str() {
        "0" => format!("https://random-word-api.herokuapp.com/word?lang={}", eval("lang").as_str()),
        _ => format!("https://random-word-api.herokuapp.com/word?lang={}&length={}", eval("lang").as_str(), eval("length").as_str()),

    }
}
