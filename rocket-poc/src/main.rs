use std::collections::HashMap;

#[rocket::get("/shuffle_word?<word>")]
fn shuffle_word(word: String) -> String {
    println!("received: {}", word);
    let letters = word.chars().enumerate().collect::<HashMap<usize, char>>();
    let shuffled_Result = letters.iter().map(|(_, value)| value).collect();
    println!("returning: {}", shuffled_Result);
    shuffled_Result
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![shuffle_word])
        .launch()
        .await;
}
