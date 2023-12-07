use std::io::BufRead;

use crate::program::{Identifier, Program};

enum Mode {
    Normal,
    Insert,
}

pub fn start(program: &mut Program) {
    let mut mode = Mode::Normal;
    let mut line_idx = 0;
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        handle(&mut mode, &mut line_idx, program, line);
    }
}
fn handle(mode: &mut Mode, line_idx: &mut isize, program: &mut Program, line: String) {
    let words = line.split_whitespace().collect::<Vec<&str>>();

    match mode {
        Mode::Normal => {
            if words.is_empty() {
                eprintln!("\x1b[31mInvalid command\x1b[0m");
                return;
            }
            match words[0] {
                "clear" | "cls" => {
                    println!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                }
                "help" | "h" | "?" => {
                    println!("{}", crate::HELP);
                }
                "open" | "o" => {
                    if words.len() != 2 {
                        eprintln!("\x1b[31mInvalid command\x1b[0m");
                        eprintln!("\x1b[31mUsage: open <filepath>\x1b[0m");
                        return;
                    }
                    match program.open(words[1]) {
                        Ok(_) => {}
                        Err(err) => {
                            eprintln!("\x1b[31m{}\x1b[0m", err);
                        }
                    }
                }
                "close" | "c" => {
                    if words.len() < 2 {
                        eprintln!("\x1b[31mInvalid command\x1b[0m");
                        eprintln!("\x1b[31mUsage: close <id|filepath>\x1b[0m");
                        return;
                    }
                    match words[1].parse::<u16>() {
                        Ok(id) => {
                            program.close(Identifier::Id(id));
                        }
                        Err(_) => {
                            program.close(Identifier::Path(words[1].to_string()));
                        }
                    }
                }
                "save" | "s" => {
                    program.save();
                }
                "save-all" | "sa" => {
                    program.save_all();
                }
                "focus" | "f" => {
                    if words.len() != 2 {
                        eprintln!("\x1b[31mInvalid command\x1b[0m");
                        eprintln!("\x1b[31mUsage: focus <id|filepath>\x1b[0m");
                        return;
                    }
                    match words[1].parse::<u16>() {
                        Ok(id) => {
                            program.focus(Identifier::Id(id));
                        }
                        Err(_) => {
                            program.focus(Identifier::Path(words[1].to_string()));
                        }
                    }
                }
                "insert" | "i" => {
                    if program.current_buffer_id.is_none() {
                        eprintln!("\x1b[31mNo buffer focused\x1b[0m");
                        return;
                    }
                    if words.len() < 2 {
                        *line_idx = program
                            .get_buffer(Identifier::Id(program.current_buffer_id.unwrap()))
                            .unwrap()
                            .data
                            .lines()
                            .count() as isize;
                        *mode = Mode::Insert;
                        return;
                    }
                    match words[1].parse::<isize>() {
                        Ok(idx) => {
                            *line_idx = idx;
                        }
                        Err(_) => {
                            eprintln!("\x1b[31mInvalid command\x1b[0m");
                            eprintln!("\x1b[31mUsage: insert <line_idx>\x1b[0m");
                            return;
                        }
                    }
                    *mode = Mode::Insert;
                }
                "list-dir" | "ld" => {
                    let mut path = ".";
                    if words.len() > 1 {
                        path = words[1];
                    }
                    let mut files = vec![];
                    let mut dirs = vec![];
                    let all_files = match std::fs::read_dir(path) {
                        Ok(files) => files,
                        Err(_) => {
                            eprintln!("\x1b[31mFile not found\x1b[0m");
                            return;
                        }
                    };
                    for file in all_files {
                        let file = file.unwrap();
                        if file.file_type().unwrap().is_file() {
                            files.push(file.file_name().to_string_lossy().to_string());
                        } else if file.file_type().unwrap().is_dir() {
                            dirs.push(file.file_name().to_string_lossy().to_string());
                        }
                    }
                    for file in files {
                        println!("file │ {}", file);
                    }
                    for dir in dirs {
                        println!("dir  │ {}", dir);
                    }
                }
                "list" | "l" => {
                    let buffers = program.list_buffers();
                    println!("ID \t│ PATH");
                    for buffer in buffers {
                        if program.current_buffer_id == Some(buffer.id) {
                            print!("\x1b[32m");
                        }
                        print!("{} \t│ {}", buffer.id, buffer.filepath);
                        if buffer.modified {
                            print!("\x1b[31m*");
                        }
                        println!("\x1b[0m");
                    }
                }
                "print" | "p" => {
                    let current_buffer_id = match program.current_buffer_id {
                        Some(id) => id,
                        None => {
                            eprintln!("\x1b[31mNo buffer focused\x1b[0m");
                            return;
                        }
                    };
                    let buffer = match program.get_buffer(Identifier::Id(current_buffer_id)) {
                        Some(b) => b,
                        None => {
                            eprintln!("\x1b[31mNo buffer focused\x1b[0m");
                            return;
                        }
                    };
                    println!("\nID: {}\nPATH: {}\n", buffer.id, buffer.filepath);
                    program.print();
                }
                "exit" | "q" => {
                    std::process::exit(0);
                }
                _ => {
                    eprintln!("\x1b[31mInvalid command\x1b[0m");
                }
            }
        }
        Mode::Insert => {
            let text = line.trim();
            program.insert(text, *line_idx);
            *mode = Mode::Normal;
        }
    }
}
