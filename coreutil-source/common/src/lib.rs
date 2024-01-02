use getch::Getch;

pub fn input() -> Option<String> {
    let mut buffer = "".to_string();
    let input = Getch::new();
    let mut letter;
    loop {
        letter = input.getch().unwrap() as char;
        match letter {
            '\n' => break,
            '\x04' => break,
            _ => buffer.push(letter),
        }
    }
    if letter == '\n' {
        return Some(buffer);
    } else if letter == '\x04' {
        return None;
    } else {
        panic!();
    }
}
