use std::collections::VecDeque;

fn main() {
    use Operation::*;
    let mut data = Stack(
        //
        [
            DataType::Float(10.0),
            DataType::String(String::from("ARGGHH")),
        ]
        .into(),
    );
    let program = [
        //
        //
        Print,
        Duplicate,
        Print,
        Duplicate,
        Push(DataType::Float(1.0)),
        Add,
        Duplicate,
        Jnz(1),
    ];
    data.execute(&program);
}
#[derive(Clone)]
enum Operation {
    Add,
    Multiply,
    Subtraction,
    Division,
    Print,
    // Jump if not zero
    Jnz(usize),
    Duplicate,
    Push(DataType),
}

#[derive(Clone)]
enum DataType {
    Float(f64),
    String(String),
}

impl From<f64> for DataType {
    fn from(f: f64) -> Self {
        DataType::Float(f)
    }
}
impl From<String> for DataType {
    fn from(f: String) -> Self {
        DataType::String(f)
    }
}
struct Stack(VecDeque<DataType>);

impl Stack {
    pub fn push(&mut self, val: DataType) {
        self.0.push_back(val);
    }
    pub fn pop(&mut self) -> DataType {
        match self.0.pop_back() {
            Some(v) => v,
            None => panic!("Stack Empty!"),
        }
    }

    pub fn execute(&mut self, ops: &[Operation]) {
        let mut ip = 0;

        while ops.len() != ip {
            let op = &ops[ip];

            match op {
                Operation::Add => match (self.pop(), self.pop()) {
                    (DataType::Float(a), DataType::Float(b)) => self.push(DataType::Float(a + b)),
                    _ => panic!("ERROR!"),
                },
                Operation::Subtraction => match (self.pop(), self.pop()) {
                    (DataType::Float(a), DataType::Float(b)) => self.push(DataType::Float(a - b)),
                    _ => panic!("ERROR!"),
                },
                Operation::Multiply => match (self.pop(), self.pop()) {
                    (DataType::Float(a), DataType::Float(b)) => self.push(DataType::Float(a * b)),
                    _ => panic!("ERROR!"),
                },
                Operation::Division => match (self.pop(), self.pop()) {
                    (DataType::Float(a), DataType::Float(b)) => self.push(DataType::Float(a / b)),
                    _ => panic!("ERROR!"),
                },
                Operation::Print => match self.pop() {
                    DataType::Float(a) => {
                        println!("{}", a)
                    }
                    DataType::String(a) => {
                        println!("{}", a)
                    }
                },
                Operation::Jnz(jmp_addr) => match self.pop() {
                    DataType::Float(val) => {
                        if val != 0.0 {
                            ip = jmp_addr.clone();
                            continue;
                        }
                    }
                    _ => panic!("STRING!"),
                },
                Operation::Duplicate => {
                    let val = self.pop();
                    self.push(val.clone());
                    self.push(val.clone());
                }
                Operation::Push(value) => {
                    self.push(value.clone());
                }
            }
            ip += 1;
        }
    }
}

#[test]
fn test_stack() {}
