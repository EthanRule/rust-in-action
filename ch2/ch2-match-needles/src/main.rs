fn main() {
    let _needle = 42;
    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

    for item in &haystack {
        let result = match item {
            42 | 132 => "hit!",
            _ => "miss",
        };
    
        if result == "hit!" {
            println!("{}: {}", item, result);
        }
    }
}
