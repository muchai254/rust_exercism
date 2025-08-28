pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut result = String::new();
    
    for i in (start_bottles - take_down + 1..=start_bottles).rev() {
        if i > 0 {
            // Add the verse for the current number of bottles
            let bottle_word = if i == 1 { "bottle" } else { "bottles" };
            let next_i = i - 1;
            let next_bottle_word = if next_i == 1 { "bottle" } else { "bottles" };
            
            result.push_str(&format!(
                "{} green {} hanging on the wall,\n",
                capitalize_number(i),
                bottle_word
            ));
            result.push_str(&format!(
                "{} green {} hanging on the wall,\n",
                capitalize_number(i),
                bottle_word
            ));
            
            if next_i > 0 {
                result.push_str(&format!(
                    "And if one green bottle should accidentally fall,\nThere'll be {} green {} hanging on the wall.",
                    number_to_words(next_i),
                    next_bottle_word
                ));
            } else {
                result.push_str("And if one green bottle should accidentally fall,\nThere'll be no green bottles hanging on the wall.");
            }
            
            // Add a double newline between verses, but not after the last verse
            if i > start_bottles - take_down + 1 {
                result.push_str("\n\n");
            }
        }
    }
    
    result
}

// Helper function to capitalize the first letter of number words
fn capitalize_number(n: u32) -> String {
    let word = number_to_words(n);
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

// Helper function to convert numbers to words
fn number_to_words(n: u32) -> String {
    match n {
        0 => "no".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        10 => "ten".to_string(),
        _ => n.to_string(), // Fallback for numbers outside 0-10
    }
}