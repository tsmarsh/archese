pub fn tokenize(program: &str) -> Vec<String>{
    program.replace("(", " ( ")
        .replace(")", " ) ")
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_owned())
        .collect()
}
