use std::collections::HashMap;
use std::io::BufRead;
use std::path::Path;

#[derive(Clone, Copy)]
enum Reg {
    X = 0,
    Y = 1,
    Z = 2,
    W = 3,
}

#[derive(Clone, Copy)]
enum Arg {
    Register(Reg),
    Literal(i64)
}

#[derive(Clone, Copy)]
enum Instr {
    SetReg(Reg),
    Add(Arg, Arg),
    Mul(Arg, Arg),
    Div(Arg, Arg),
    Mod(Arg, Arg),
    Eql(Arg, Arg)
}

type Registers = [i64;4];

fn parse_reg(tok: &str) -> Reg {
    match tok {
        "x" => Reg::X,
        "y" => Reg::Y,
        "z" => Reg::Z,
        "w" => Reg::W,
        _   => panic!("Unexpected token {}", tok)
    }
}

fn parse_arg(tok: &str) -> Arg {
    match tok.parse::<i64>() {
        Ok(n) => Arg::Literal(n),
        _ => Arg::Register(parse_reg(tok))
    }
}

fn run_monad_block(instrs: &[Instr], input_val: i64, z_state: i64) -> Registers {
    fn get_arg_vals(arg_0: Arg, arg_1: Arg, reg: &Registers) -> (i64, i64) {
        match (arg_0, arg_1) {
            (Arg::Register(r0), Arg::Register(r1)) => (reg[r0 as usize], reg[r1 as usize]),
            (Arg::Register(r), Arg::Literal(l)) => (reg[r as usize], l),
            (Arg::Literal(_), _) => panic!(),
        }
    }

    fn set_reg(arg: Arg, val: i64, reg: &mut Registers) {
        match arg {
            Arg::Register(r) => reg[r as usize] = val,
            _ => panic!()
        }
    }

    let mut reg = Registers::default();
    reg[Reg::Z as usize] = z_state;
    reg[Reg::W as usize] = input_val;

    for ins in instrs {
        match *ins {
            Instr::SetReg(_) => panic!(),
            Instr::Add(r0, r1) => {
                let (arg_0, arg_1) = get_arg_vals(r0, r1, &reg); 
                set_reg(r0, arg_0 + arg_1, &mut reg);
            },
            Instr::Mul(r0, r1) => {
                let (arg_0, arg_1) = get_arg_vals(r0, r1, &reg); 
                set_reg(r0, arg_0 * arg_1, &mut reg);
            },
            Instr::Div(r0, r1) => {
                let (arg_0, arg_1) = get_arg_vals(r0, r1, &reg); 
                set_reg(r0, arg_0 / arg_1, &mut reg);
            },
            Instr::Mod(r0, r1) => {
                let (arg_0, arg_1) = get_arg_vals(r0, r1, &reg); 
                set_reg(r0, arg_0 % arg_1, &mut reg);
            },
            Instr::Eql(r0, r1) => {
                let (arg_0, arg_1) = get_arg_vals(r0, r1, &reg); 
                let val = if arg_0 == arg_1 { 1 } else { 0 };
                set_reg(r0, val, &mut reg);
            }
        }
    }
    reg
}

fn transpile(instrs: &[Instr]) {
    println!("let mut x = 0;");
    println!("let mut y = 0;");
    println!("let mut z = 0;");
    println!("let mut w = 0;");

    fn reg_to_str(r: Reg) -> String {
        match r {
            Reg::X => "x".to_string(),
            Reg::Y => "y".to_string(),
            Reg::Z => "z".to_string(),
            Reg::W => "w".to_string(),
        }
    }

    fn arg_to_str(arg: Arg) -> String {
        match arg {
            Arg::Register(r) => reg_to_str(r),
            Arg::Literal(l) => l.to_string(),
        }
    }

    for ins in instrs {
        match *ins {
            Instr::SetReg(r) => {
                println!("{} = *yield_input.next().unwrap();", reg_to_str(r));
            }
            Instr::Add(r0, r1) => {
                println!("{} = {} + {};", arg_to_str(r0), arg_to_str(r0), arg_to_str(r1))
            },
            Instr::Mul(r0, r1) => {
                println!("{} = {} * {};", arg_to_str(r0), arg_to_str(r0), arg_to_str(r1))
            },
            Instr::Div(r0, r1) => {
                println!("{} = {} / {};", arg_to_str(r0), arg_to_str(r0), arg_to_str(r1))
            },
            Instr::Mod(r0, r1) => {
                println!("{} = {} % {};", arg_to_str(r0), arg_to_str(r0), arg_to_str(r1))
            },
            Instr::Eql(r0, r1) => {
                println!("{} = if {} == {} {{1}} else {{0}};", arg_to_str(r0), arg_to_str(r0), arg_to_str(r1))
            }
        }
    }
}

// Map input z and code block to out z
type Cache = HashMap<(i64, usize), Option<i64>>;

fn solve(code_blocks: &[Vec<Instr>], digits: &[i64], cache: &mut Cache, block_idx: usize, last_z: i64) -> Option<i64> {
    if block_idx == code_blocks.len() {
        return if last_z == 0 {
            Some(0)
        } else {
            None
        }
    }

    if let Some(&res) = cache.get(&(last_z, block_idx)) {
        return res;
    }

    for &digit in digits {
        let regs = run_monad_block(&code_blocks[block_idx], digit, last_z);
        let out_z = regs[Reg::Z as usize];

        match solve(code_blocks, digits, cache, block_idx + 1, out_z) {
            Some(val) => {
                let res = Some(val * 10 + digit);
                cache.insert((out_z, block_idx), res);
                return res;
            },
            None => (),
        }
    }

    cache.insert((last_z, block_idx), None);

    None
}

fn run_instructions(instrs: &[Instr], inputs: &[i64]) -> Registers {
    let mut yield_input = inputs.iter();
    let mut reg = Registers::default();

    fn get_arg_vals(arg_0: Arg, arg_1: Arg, reg: &Registers) -> (i64, i64) {
        match (arg_0, arg_1) {
            (Arg::Register(r0), Arg::Register(r1)) => (reg[r0 as usize], reg[r1 as usize]),
            (Arg::Register(r), Arg::Literal(l)) => (reg[r as usize], l),
            (Arg::Literal(_), _) => panic!(),
        }
    }

    fn set_reg(arg: Arg, val: i64, reg: &mut Registers) {
        match arg {
            Arg::Register(r) => reg[r as usize] = val,
            _ => panic!()
        }
    }

    for ins in instrs {
        match *ins {
            Instr::SetReg(r) => {
                let val = yield_input.next().unwrap();
                reg[r as usize] = *val;
            }
            Instr::Add(r0, r1) => {
                let (arg_0, arg_1) = get_arg_vals(r0, r1, &reg); 
                set_reg(r0, arg_0 + arg_1, &mut reg);
            },
            Instr::Mul(r0, r1) => {
                let (arg_0, arg_1) = get_arg_vals(r0, r1, &reg); 
                set_reg(r0, arg_0 * arg_1, &mut reg);
            },
            Instr::Div(r0, r1) => {
                let (arg_0, arg_1) = get_arg_vals(r0, r1, &reg); 
                set_reg(r0, arg_0 / arg_1, &mut reg);
            },
            Instr::Mod(r0, r1) => {
                let (arg_0, arg_1) = get_arg_vals(r0, r1, &reg); 
                set_reg(r0, arg_0 % arg_1, &mut reg);
            },
            Instr::Eql(r0, r1) => {
                let (arg_0, arg_1) = get_arg_vals(r0, r1, &reg); 
                let val = if arg_0 == arg_1 { 1 } else { 0 };
                set_reg(r0, val, &mut reg);
            }
        }
    }
    reg
}

pub fn run(root_dir: &Path) {
    let input_path = root_dir.join("day24_input.txt");
    
    let file = std::fs::File::open(input_path).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut instrs = Vec::new();

    for res in reader.lines() {
        let line = match res {
            Ok(l) => l,
            Err(_) => continue,
        };

        let toks = line.split(" ").collect::<Vec<_>>();
        assert!(toks.len() == 2 || toks.len() == 3);

        match toks[0] {
            "inp" => {
                let reg = parse_reg(toks[1]);
                instrs.push(Instr::SetReg(reg));
            },
            "add" => instrs.push(Instr::Add(parse_arg(toks[1]), parse_arg(toks[2]))),
            "mul" => instrs.push(Instr::Mul(parse_arg(toks[1]), parse_arg(toks[2]))),
            "div" => instrs.push(Instr::Div(parse_arg(toks[1]), parse_arg(toks[2]))),
            "mod" => instrs.push(Instr::Mod(parse_arg(toks[1]), parse_arg(toks[2]))),
            "eql" => instrs.push(Instr::Eql(parse_arg(toks[1]), parse_arg(toks[2]))),
            _   => panic!("Unexpected token {}", toks[0])
        }
    }

    // MONAD has 14 blocks that operate on each input digit,
    // carrying over state from the last block in the z register.
    // We can solve this individually, if we pass along the state.
    let mut code_blocks = instrs.split(|i| {
        match i {
            Instr::SetReg(_) => true,
            _ => false
        }
    }).map(|i| i.to_vec()).collect::<Vec<_>>();

    code_blocks.remove(0);

    let mut cache = HashMap::new(); 

    let biggest = solve(&code_blocks, &(1..=9).rev().collect::<Vec<_>>(), &mut cache, 0, 0).unwrap();
    for d in biggest.to_string().chars().rev() {
        print!("{}", d);
    }
    println!();
    

    let smallest = solve(&code_blocks, &(1..=9).collect::<Vec<_>>(), &mut cache, 0, 0).unwrap();
    for d in smallest.to_string().chars().rev() {
        print!("{}", d);
    }
    println!();
}
