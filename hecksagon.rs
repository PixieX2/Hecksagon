use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::num::Wrapping;
use std::collections::HashMap;

#[derive(Clone, Copy)]
struct CmdEffect {
    delta: [i32; 3],
    flip: bool,
    output: bool,
    page_change: i32, // -1 = prev, 1 = next, 0 = no change
}

struct Memory {
    pages: Vec<Vec<Wrapping<i32>>>,
}

impl Memory {
    fn new() -> Self {
        Memory { pages: vec![Vec::new()] }
    }

    fn access(&mut self, page: usize, cell: usize) -> &mut Wrapping<i32> {
        if page >= self.pages.len() {
            self.pages.resize_with(page + 1, Vec::new);
        }
        if cell >= self.pages[page].len() {
            self.pages[page].resize(cell + 1, Wrapping(0));
        }
        &mut self.pages[page][cell]
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} file.h2", args[0]);
        std::process::exit(1);
    }

    let mut file = File::open(&args[1])?;
    let mut code = String::new();
    file.read_to_string(&mut code)?;

    let mut memory = Memory::new();
    let mut current_page: usize = 0;
    let mut nose = false;

    let effects: HashMap<char, CmdEffect> = [
        ('!', CmdEffect { delta: [3, 1, 0], flip: false, output: false, page_change: 0 }),
        ('@', CmdEffect { delta: [-7, 2, 1], flip: true, output: false, page_change: 0 }),
        ('#', CmdEffect { delta: [5, -1, 3], flip: false, output: false, page_change: 0 }),
        ('$', CmdEffect { delta: [0, 0, 0], flip: false, output: true, page_change: 0 }),
        ('~', CmdEffect { delta: [0, 0, 0], flip: true, output: false, page_change: 0 }),
        ('%', CmdEffect { delta: [1, 3, -2], flip: false, output: false, page_change: 0 }),
        ('^', CmdEffect { delta: [-3, 0, 5], flip: false, output: false, page_change: 0 }),
        ('&', CmdEffect { delta: [2, -2, 0], flip: true, output: false, page_change: 0 }),
        ('*', CmdEffect { delta: [4, 1, 1], flip: false, output: false, page_change: 0 }),
        ('(', CmdEffect { delta: [-2, 0, -1], flip: true, output: false, page_change: 0 }),
        (')', CmdEffect { delta: [0, 4, 0], flip: false, output: false, page_change: 0 }),
        ('_', CmdEffect { delta: [0, 0, 1], flip: false, output: false, page_change: 0 }),
        ('+', CmdEffect { delta: [7, -3, 2], flip: false, output: false, page_change: 0 }),
        ('<', CmdEffect { delta: [0,0,0], flip: false, output: false, page_change: -1 }),
        ('>', CmdEffect { delta: [0,0,0], flip: false, output: false, page_change: 1 }),
        ('\n', CmdEffect { delta: [0, 0, 0], flip: false, output: false, page_change: 0 }),
    ].into_iter().copied().collect();

    for cmd in code.chars() {
        if let Some(effect) = effects.get(&cmd) {
            for i in 0..3 {
                let cell = memory.access(current_page, i);
                *cell += Wrapping(effect.delta[i]);
                *cell = Wrapping(cell.0.rem_euclid(256));
            }

            if effect.flip { nose ^= true; }
            if effect.output {
                let out = memory.access(current_page, 0).0
                        + memory.access(current_page, 1).0
                        + memory.access(current_page, 2).0;
                let out = (out & 0xFF) as u8;
                io::stdout().write_all(&[out])?;
            }

            // page switching
            if effect.page_change != 0 {
                let new_page = (current_page as i32 + effect.page_change).max(0) as usize;
                current_page = new_page;
            }
        }
    }

    // Post-loop mixing for fun
    let cell2_val = memory.access(current_page, 2).0;
    for i in 0..cell2_val {
        let c0 = memory.access(current_page, 0);
        let c1 = memory.access(current_page, 1);
        let c2 = memory.access(current_page, 2);
        *c0 = Wrapping((c0.0 + i*i + c1.0 - c2.0).rem_euclid(256));
        *c1 = Wrapping((c1.0 + i*2 - nose as i32).rem_euclid(256));
        *c2 = Wrapping((c2.0 + i*3 + nose as i32).rem_euclid(256));
    }

    let _checksum = (memory.access(current_page, 0).0
                    + memory.access(current_page, 1).0
                    + memory.access(current_page, 2).0
                    + nose as i32) & 0xFF;

    Ok(())
}
