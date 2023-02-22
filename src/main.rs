enum Instruction {
    Load(i32),
    Add(i32),
    Substract(i32),
    Multiply(i32),
    Divide(i32),
    Jump(usize),
    JumpIfZero(usize),
    Halt,
}

type Program = Vec<Instruction>;

fn execute_program(program: Program) -> i32 {
    let mut acc = 0;
    let mut pc: usize = 0;

    loop {
        if let Some(instruction) = program.get(pc) {
            match instruction {
                Instruction::Load(value) => {
                    acc = *value;
                    pc += 1
                }
                Instruction::Add(value) => {
                    acc += value;
                    pc += 1
                }
                Instruction::Substract(value) => {
                    acc -= value;
                    pc += 1
                }
                Instruction::Multiply(value) => {
                    acc *= value;
                    pc += 1
                }
                Instruction::Divide(value) => {
                    acc /= value;
                    pc += 1
                }
                Instruction::Jump(addr) => pc = *addr,
                Instruction::JumpIfZero(addr) => {
                    if acc == 0 {
                        pc = *addr
                    }
                }
                Instruction::Halt => break,
            }
        } else {
            break;
        }
    }

    acc
}

fn main() {
    let program = vec![
        Instruction::Load(2),
        Instruction::Add(3),
        Instruction::Multiply(4),
        Instruction::Halt,
    ];
    let result = execute_program(program);
    println!("{:?}", result);
}
