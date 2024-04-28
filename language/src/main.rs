use lang::*;

#[cfg(test)]
mod test;

pub mod lang;

fn main() {
    let path: String = std::env::args().skip(1).next().expect("no path provided");

    let input = std::fs::read_to_string(path).expect("invalid path");

    let instant = std::time::Instant::now();
    let program = Program::default().compile_str(&input).map_err(|e| panic!("{}", e)).unwrap();
    println!("compilation: {:?}", instant.elapsed());
    let instant = std::time::Instant::now();
    program.run();
    println!("execution: {:?}", instant.elapsed());
}