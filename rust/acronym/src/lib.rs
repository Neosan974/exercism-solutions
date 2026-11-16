pub fn abbreviate(phrase: &str) -> String {
    let acronym = phrase.split(&[' ', '-']).map(|word| word.replace("_", "").get(..1).unwrap().to_string().to_uppercase()).collect();
    println!("{:?}", acronym);
    acronym
}
