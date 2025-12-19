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
        Memory { pages: vec![Vec::with_capacity(8)] }
    }

    fn access(&mut self, page: usize, cell: usize) -> &mut Wrapping<i32> {
        if page >= self.pages.len() {
            self.pages.resize_with(page + 1, || Vec::with_capacity(8));
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
    else if &args [1] == "-v" {
        println!("v3.x (Impossible to determine exact version because this was not created using cargo.)"); 
        
    }
    else if &args [1] == "-h" {
        println!("Usage: {} file.h2", args[0]);
    }
    else if &args [1] == "--help" {
        println!("Usage: {} file.h2", args[0]);

    }
    else if &args [1] == "--version" {
        println!("v3.x (Impossible to determine exact version because this was not created using cargo.)");

    }
    else if &args [1] == "--stdin" {
        println!("Not yet implemented, and probably never will be. if you REALLY want to do that, just do PR please.");
        
    }
    else if &args [1] == "--fhelp" {
        println!("do a PR if you know how it works and want to add full help. It's not that hard unless you only use python or something like that. I planned to make this help page, but I don't have the time to do it.");
    }


    let mut file = File::open(&args[1])?;
    let mut code = String::new();
    file.read_to_string(&mut code)?;

    let mut memory = Memory::new();
    let mut current_page: usize = 0;
    let mut nose = false;

    let mut effects: HashMap<char, CmdEffect> = HashMap::new();
    for &(c, d0, d1, d2, flip, output, page_change) in &[
        ('!', 3, 1, 0, false, false, 0),
        ('@', -7, 2, 1, true, false, 0),
        ('#', 5, -1, 3, false, false, 0),
        ('$', 0, 0, 0, false, true, 0),
        ('~', 0, 0, 0, true, false, 0),
        ('%', 1, 3, -2, false, false, 0),
        ('^', -3, 0, 5, false, false, 0),
        ('&', 2, -2, 0, true, false, 0),
        ('*', 4, 1, 1, false, false, 0),
        ('(', -2, 0, -1, true, false, 0),
        (')', 0, 4, 0, false, false, 0),
        ('_', 0, 0, 1, false, false, 0),
        ('+', 7, -3, 2, false, false, 0),
        ('<', 0, 0, 0, false, false, -1),
        ('>', 0, 0, 0, false, false, 1),
        ('\n', 0, 0, 0, false, false, 0),
    ] {
        effects.insert(c, CmdEffect { delta: [d0, d1, d2], flip, output, page_change });
    }

    for cmd in code.chars() {
        if let Some(effect) = effects.get(&cmd) {
            for i in 0..3 {
                let cell = memory.access(current_page, i);
                *cell += Wrapping(effect.delta[i]);
                cell.0 &= 0xFF; // faster than rem_euclid
            }

            if effect.flip { nose ^= true; }
            if effect.output {
                let sum = memory.access(current_page, 0).0
                        + memory.access(current_page, 1).0
                        + memory.access(current_page, 2).0;
                io::stdout().write_all(&[(sum & 0xFF) as u8])?;
            }

            if effect.page_change != 0 {
                current_page = (current_page as i32 + effect.page_change).max(0) as usize;
            }
        }
    }

    // Post-loop mixing
    let cell2_val = memory.access(current_page, 2).0.max(0) as usize;
     for i in 0..cell2_val {
     let i = i as i32; // cast once

     // borrow c1 and c2 separately
     let c1_val;
     let c2_val;
     {
         let c1 = memory.access(current_page, 1);
         c1_val = c1.0;
     }
     {
         let c2 = memory.access(current_page, 2);
         c2_val = c2.0;
     }

     {
         let c0 = memory.access(current_page, 0);
         *c0 = Wrapping((c0.0 + i*i + c1_val - c2_val) & 0xFF);
     }

     {
         let c1 = memory.access(current_page, 1);
         *c1 = Wrapping((c1.0 + i*2 - nose as i32) & 0xFF);
     }

     {
         let c2 = memory.access(current_page, 2);
         *c2 = Wrapping((c2.0 + i*3 + nose as i32) & 0xFF);
     }
 }

    Ok(())
}