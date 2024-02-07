use crate::menu::eval;

pub fn get_word() -> String{
    if eval("lang").as_str() == "pl" {
        return api(generate_polish_url());
    }
    setup_mode();
    match eval("easy").as_str(){
        "0" => api(generate_hard_url()),
        "1" => api(generate_easy_url()),
        _ => panic!("How the fuck")
    }

}

pub fn api(url: String) -> String {
    println!("The executioner is thinking......");
    match reqwest::blocking::get(url).unwrap().text() {
        Ok(value) => {
            match eval("lang").as_str() == "pl" {
                true => polish_cleanup(value),
                false => word_cleanup(value),
            }
        }
        Err(_) => get_word()
    }
}

fn polish_cleanup(s: String) -> String {
    let mut s = s.clone();
    for _ in 0..9 {
        s.remove(0);
    }
    for _ in 0..2 {
        s.pop();
    }
    s

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
fn generate_polish_url() -> String {
    match eval("length").as_str() {
        "0" => String::from("http://16.171.29.102/word"),
        _ => format!("http://16.171.29.102/word/length/{}", eval("length").as_str())
    }
}
