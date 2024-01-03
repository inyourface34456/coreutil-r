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

pub fn char_to_str(subject: char) -> String {
  let mut temp: [u8; 4] = [0; 4];
  subject.encode_utf8(&mut temp);
  let temp2: &str = std::str::from_utf8(&temp).unwrap();
  temp2.to_owned()
}
