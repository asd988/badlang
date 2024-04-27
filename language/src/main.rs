use lang::*;

#[cfg(test)]
mod test;

pub mod lang;

fn main() {
    // read "test" file
    let input = std::fs::read_to_string("test.txt").unwrap();

    //create_program("a = -10\na+= 15\n< a\n").run();

    let instant = std::time::Instant::now();
    let program = Program::default().compile_str(&input).unwrap();
    println!("compilation: {:?}", instant.elapsed());
    let instant = std::time::Instant::now();
    program.run();
    println!("execution: {:?}", instant.elapsed());
}