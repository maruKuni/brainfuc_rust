mod brainfuck;
use brainfuck::interpreter::BFInterpreter;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let tape = args[1].as_str();
    let mut instance = BFInterpreter::new(tape);
    instance.run();
}
