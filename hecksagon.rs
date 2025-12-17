use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::num::Wrapping;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} file.h2", args[0]);
        std::process::exit(1);
    }

    let mut file = File::open(&args[1])?;
    let mut code = String::new();
    file.read_to_string(&mut code)?;

    let mut cells = [Wrapping(0i32); 3]; // cell=0, shadow=1, ghost=2
    let mut nose = false;
    let debug = false;

    for cmd in code.chars() {
        let mut d = [Wrapping(0i32); 3];
        let mut flip = false;

        match cmd {
            '!' => { d[0] += Wrapping(3); d[1] += Wrapping(1); }
            '@' => { d[0] += Wrapping(-7); flip = true; d[1] += Wrapping(2); d[2] += Wrapping(1); }
            '#' => { d[0] += Wrapping(5); d[1] += Wrapping(-1); d[2] += Wrapping(3); }
            '$' => {}
            '~' => { flip = true; }
            '%' => { d[0] += Wrapping(1); d[1] += Wrapping(3); d[2] += Wrapping(-2); }
            '^' => { d[0] += Wrapping(-3); d[2] += Wrapping(5); }
            '&' => { d[0] += Wrapping(2); flip = true; d[1] += Wrapping(-2); }
            '*' => { d[0] += Wrapping(4); d[1] += Wrapping(1); d[2] += Wrapping(1); }
            '(' => { d[0] += Wrapping(-2); flip = true; d[2] += Wrapping(-1); }
            ')' => { d[1] += Wrapping(4); }
            '_' => { d[2] += Wrapping(1); }
            '+' => { d[0] += Wrapping(7); d[1] += Wrapping(-3); d[2] += Wrapping(2); }
            _ => continue,
        }

        for i in 0..3 {
            cells[i] = Wrapping((cells[i].0 + d[i].0).rem_euclid(256));
        }
        if flip { nose ^= true; }

        if cmd == '$' {
            let out = ((cells[0].0 + cells[1].0 + cells[2].0) & 0xFF) as u8;
            io::stdout().write_all(&[out])?;
        }

        if debug {
            eprintln!("[DEBUG] cmd={} cell={} shadow={} ghost={} nose={}", 
                      cmd, cells[0].0, cells[1].0, cells[2].0, nose as u8);
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
