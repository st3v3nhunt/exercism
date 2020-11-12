pub fn raindrops(n: u32) -> String {
    let mut sounds = vec![];

    if (n % 3) == 0 {
        sounds.push("Pling");
    }

    if (n % 5) == 0 {
        sounds.push("Plang");
    }

    if (n % 7) == 0 {
        sounds.push("Plong");
    }

    if sounds.len() == 0 {
        // sounds.push(n.to_string())
        return n.to_string();
    }
    let ret = sounds.join("");
    println!("sound: {}", ret);
    return ret;
}
