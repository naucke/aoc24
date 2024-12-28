// TODO input parsing
const PROGRAM: &[u8] = &[2, 4, 1, 3, 7, 5, 0, 3, 1, 5, 4, 1, 5, 5, 3, 0];

struct VM {
    a: i64,
    b: i64,
    c: i64,
}

impl VM {
    fn combo(&self, combo: u8) -> i64 {
        match combo {
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => combo as i64,
        }
    }

    fn dv(&self, arg: u8) -> i64 {
        self.a / (1 << self.combo(arg))
    }

    fn run(&mut self, prog: &[u8]) -> Vec<u8> {
        let mut ip = 0;
        let mut out = Vec::new();
        while ip + 1 < prog.len() {
            let arg = prog[ip + 1];
            match prog[ip] {
                0 => self.a = self.dv(arg),
                1 => self.b = self.b ^ arg as i64,
                2 => self.b = self.combo(arg) % 8,
                3 => {
                    if self.a != 0 {
                        ip = arg as usize;
                        continue;
                    }
                }
                4 => self.b = self.b ^ self.c,
                5 => out.push((self.combo(arg) % 8) as u8),
                6 => self.b = self.dv(arg),
                7 => self.c = self.dv(arg),
                _ => panic!("invalid opcode {}", prog[ip]),
            }
            ip += 2;
        }
        out
    }
}

fn task_a() {
    let mut vm = VM {
        a: 63687530,
        b: 0,
        c: 0,
    };
    let sth: Vec<_>= vm.run(PROGRAM).into_iter().map(|i| i.to_string()).collect();
    println!("{}", sth.join(","));
}

fn main() {
    task_a();
}
