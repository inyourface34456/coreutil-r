use clap::Parser;
use getch::Getch;
use regex::Regex;
use std::fs;

#[derive(Parser)]
#[command(author, version, about, long_about)]
struct Cli {
    name: Vec<String>,

    /// equivalent to -vET
    #[arg(short = 'A', long = "show-all", default_value_t = false)]
    show_all: bool,

    /// number nonempty output lines, overrides -n
    #[arg(short = 'b', long = "number-nonblank", default_value_t = false)]
    number_nonblank: bool,

    /// equivalent to -vE
    #[arg(short = 'e', default_value_t = false)]
    show_nonprinting_and_ends: bool,

    /// display $ at end of each line
    #[arg(short = 'E', long = "show-ends", default_value_t = false)]
    show_ends: bool,

    /// number all output lines
    #[arg(short = 'n', long = "number", default_value_t = false)]
    number: bool,

    /// suppress repeated empty output lines
    #[arg(short = 's', long = "squeeze-blank", default_value_t = false)]
    squeeze_blank: bool,

    /// equivalent to -vT
    #[arg(short = 't', default_value_t = false)]
    show_nonprinting_and_tabs: bool,

    /// display TAB characters as ^I
    #[arg(short = 'T', long = "show-tabs", default_value_t = false)]
    show_tabs: bool,

    /// use ^ and M- notation, except for LFD and TAB
    #[arg(short = 'v', long = "show-nonprinting", default_value_t = false)]
    show_nonprinting: bool,
}

fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

fn input() -> Option<String> {
    let mut buffer = "".to_string();
    let input = Getch::new();
    let mut letter;
    loop {
        letter = input.getch().unwrap() as char;
        if letter == '\n' {
            break;
        } else if letter == '\x04' {
            break;
        } else {
            buffer.insert(buffer.len(), letter);
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

fn forever_loop() {
    loop {
        let usr_txt = input();
        match usr_txt {
            Some(dat) => println!("{}", dat),
            None => break,
        }
    }
}

fn numbered_nonblank(i: &str, line_num: usize) -> String {
    if i == "" {
        return "".to_string();
    } else {
        return format!("\t{}  {}", line_num, i);
    }
}

fn non_printing_char(mut data: String) -> String {
    static REPLACE_LIST: [&str; 256] = [
        "", "^@", "^A", "^B", "^C", "^D", "^E", "^F", "^G", "^H", "", "", "^K", "^L", "^M", "^N",
        "^O", "^P", "^Q", "^R", "^S", "^T", "^U", "^V", "^W", "^X", "^Y", "^Z", "^[", "^\\", "^]",
        "^^", "^_", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
        "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
        "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
        "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "", "",
        "", "", "", "", "", "^?", "M-^@", "M-^A", "M-^B", "M-^C", "M-^D", "M-^E", "M-^F", "M-^G",
        "M-^H", "M-^I", "M-^J", "M-^K", "M-^L", "M-^M", "M-^N", "M-^O", "M-^P", "M-^Q", "M-^R",
        "M-^S", "M-^T", "M-^U", "M-^V", "M-^W", "M-^X", "M-^Y", "M-^Z", "M-^[", "M-^\\", "M-^]",
        "M-^^", "M-^_", "M- ", "M-!", "M-\"", "M-#", "M-$", "M-%", "M-&", "M-'", "M-(", "M-)",
        "M-*", "M-+", "M-,", "M--", "M-.", "M-/", "M-0", "M-1", "M-2", "M-3", "M-4", "M-5", "M-6",
        "M-7", "M-8", "M-9", "M-:", "M-;", "M-<", "M-=", "M->", "M-?", "M-@", "M-A", "M-B", "M-C",
        "M-D", "M-E", "M-F", "M-G", "M-H", "M-I", "M-J", "M-K", "M-L", "M-M", "M-N", "M-O", "M-P",
        "M-Q", "M-R", "M-S", "M-T", "M-U", "M-V", "M-W", "M-X", "M-Y", "M-Z", "M-[", "M-\\", "M-]",
        "M-^", "M-_", "M-`", "M-a", "M-b", "M-c", "M-d", "M-e", "M-f", "M-g", "M-h", "M-i", "M-j",
        "M-k", "M-l", "M-m", "M-n", "M-o", "M-p", "M-q", "M-r", "M-s", "M-t", "M-u", "M-v", "M-w",
        "M-x", "M-y", "M-z", "M-{", "M-|", "M-}", "M-~", "M-^?",
    ];

    for i in 1..=255 as u8 {
        match i {
            9..=10 => continue,
            32..=126 => continue,
            _ => data = data.replace(i as char, REPLACE_LIST[i as usize]),
        }
    }
    data
}

fn main() {
    let mut cli = Cli::parse();
    let remove_pern = Regex::new(r"\(.+\)").unwrap();
    let take_out_newlines = Regex::new("\n{2,}").unwrap();

    if cli.show_all {
        cli.show_nonprinting = true;
        cli.show_ends = true;
        cli.show_tabs = true;
    } else if cli.show_nonprinting_and_ends {
        cli.show_nonprinting = true;
        cli.show_ends = true;
    } else if cli.show_nonprinting_and_tabs {
        cli.show_nonprinting = true;
        cli.show_tabs = true;
    }

    if !cli.name.is_empty() {
        for file in cli.name {
            let file = file.as_ref();

            if path_exists(file) {
                match fs::read_to_string::<&str>(file) {
                    Ok(mut data) => {
                        if cli.squeeze_blank {
                            data = take_out_newlines.replace_all(&data, "\n\n").to_string();
                        }

                        if cli.show_tabs {
                            data = data.replace("\t", "^I")
                        }

                        if cli.show_nonprinting {
                            data = non_printing_char(data);
                        }

                        let data: Vec<String> = data
                            .split("\n")
                            .into_iter()
                            .map(|x| {
                                let mut dat = x.to_string();
                                if cli.show_ends {
                                    dat.insert(dat.len(), '$')
                                }
                                dat
                            })
                            .collect();

                        // data.remove(data.len() - 1);

                        let mut count = 1;
                        for i in &data {
                            if cli.number_nonblank {
                                if count == data.len() {
                                    print!("{}", numbered_nonblank(i.as_ref(), count));
                                } else {
                                    println!("{}", numbered_nonblank(i.as_ref(), count));
                                }
                            } else if cli.number {
                                if count == data.len() {
                                    print!("\t{}  {}", count, i);
                                } else {
                                    println!("\t{}  {}", count, i);
                                }
                            } else {
                                if count == data.len() {
                                    print!("{}", i);
                                } else {
                                    println!("{}", i);
                                }
                            }
                            count += 1;
                        }
                    }
                    Err(error) => {
                        eprintln!(
                            "cat: {}: {}",
                            file,
                            remove_pern.replace_all(error.to_string().as_ref(), "")
                        )
                    }
                }
            } else {
                if file == "-" {
                    forever_loop()
                } else {
                    eprintln!("cat: {}: No such file or directory", file);
                }
            }
        }
    } else {
        forever_loop()
    }
}
