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

    let mut cells = [Wrapping(0i32); 3]; // cell, shadow, ghost
    let mut nose = false;

    // Lookup table for commands
    let effects: HashMap<char, CmdEffect> = [
        ('!', CmdEffect { delta: [3, 1, 0], flip: false, output: false }),
        ('@', CmdEffect { delta: [-7, 2, 1], flip: true, output: false }),
        ('#', CmdEffect { delta: [5, -1, 3], flip: false, output: false }),
        ('$', CmdEffect { delta: [0, 0, 0], flip: false, output: true }),
        ('~', CmdEffect { delta: [0, 0, 0], flip: true, output: false }),
        ('%', CmdEffect { delta: [1, 3, -2], flip: false, output: false }),
        ('^', CmdEffect { delta: [-3, 0, 5], flip: false, output: false }),
        ('&', CmdEffect { delta: [2, -2, 0], flip: true, output: false }),
        ('*', CmdEffect { delta: [4, 1, 1], flip: false, output: false }),
        ('(', CmdEffect { delta: [-2, 0, -1], flip: true, output: false }),
        (')', CmdEffect { delta: [0, 4, 0], flip: false, output: false }),
        ('_', CmdEffect { delta: [0, 0, 1], flip: false, output: false }),
        ('+', CmdEffect { delta: [7, -3, 2], flip: false, output: false }),
        ('\n', CmdEffect { delta: [0, 0, 0], flip: false, output: false }),
].into_iter().copied().collect();


    for cmd in code.chars() {
        if let Some(effect) = effects.get(&cmd) {
            for i in 0..3 {
                cells[i] += Wrapping(effect.delta[i]);
                cells[i] = Wrapping(cells[i].0.rem_euclid(256));
            }

            if effect.flip { nose ^= true; }
            if effect.output {
                let out = ((cells[0].0 + cells[1].0 + cells[2].0) & 0xFF) as u8;
                io::stdout().write_all(&[out])?;
            }
        }
    }

    for i in 0..cells[2].0 {
        cells[0] = Wrapping((cells[0].0 + i*i + cells[1].0 - cells[2].0).rem_euclid(256));
        cells[1] = Wrapping((cells[1].0 + i*2 - nose as i32).rem_euclid(256));
        cells[2] = Wrapping((cells[2].0 + i*3 + nose as i32).rem_euclid(256));
    }

    let _checksum = (cells[0].0 + cells[1].0 + cells[2].0 + nose as i32) & 0xFF;

    Ok(())
}
