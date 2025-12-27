use crate::renderer_c::render_c_to_string;
use std::collections::HashMap;
use std::fs;
use std::ops::Not;
use std::time::Instant;

mod renderer_c;

#[derive(Debug)]
enum Opcode {
    Halt,
    Set,
    Push,
    Pop,
    Eq,
    Gt,
    Jmp,
    Jt,
    Jf,
    Add,
    Mult,
    Mod,
    And,
    Or,
    Not,
    Rmem,
    Wmem,
    Call,
    Ret,
    Out,
    In,
    Noop,
}
impl Opcode {
    fn of(code: u16) -> Self {
        match code {
            0 => Opcode::Halt,
            1 => Opcode::Set,
            2 => Opcode::Push,
            3 => Opcode::Pop,
            4 => Opcode::Eq,
            5 => Opcode::Gt,
            6 => Opcode::Jmp,
            7 => Opcode::Jt,
            8 => Opcode::Jf,
            9 => Opcode::Add,
            10 => Opcode::Mult,
            11 => Opcode::Mod,
            12 => Opcode::And,
            13 => Opcode::Or,
            14 => Opcode::Not,
            15 => Opcode::Rmem,
            16 => Opcode::Wmem,
            17 => Opcode::Call,
            18 => Opcode::Ret,
            19 => Opcode::Out,
            20 => Opcode::In,
            21 => Opcode::Noop,
            _ => {
                panic!("Unknown opcode: {}", code)
            }
        }
    }
    fn args(&self) -> usize {
        match self {
            Opcode::Halt => 0,
            Opcode::Set => 2,
            Opcode::Push => 1,
            Opcode::Pop => 1,
            Opcode::Eq => 3,
            Opcode::Gt => 3,
            Opcode::Jmp => 1,
            Opcode::Jt => 2,
            Opcode::Jf => 2,
            Opcode::Add => 3,
            Opcode::Mult => 3,
            Opcode::Mod => 3,
            Opcode::And => 3,
            Opcode::Or => 3,
            Opcode::Not => 2,
            Opcode::Rmem => 2,
            Opcode::Wmem => 2,
            Opcode::Call => 1,
            Opcode::Ret => 0,
            Opcode::Out => 1,
            Opcode::In => 1,
            Opcode::Noop => 0,
        }
    }
}

fn _6049(mut r0: u16, mut r1: u16, mut r7: u16, cache: &mut HashMap<u64, u16>) -> u16 {
    if let Some(r) = cache.get(&((r0 as u64) * 32768 + r1 as u64)) {
        return *r;
    }
    if r0 != 0 {
        if r1 != 0 {
            r1 = (r1 + 32767) & 32767;
            let r = _6049(r0, r1, r7, cache);
            cache.insert((r0 as u64) * 32768 + r1 as u64, r);
            r1 = r;

            r0 = (r0 + 32767) & 32767;
            let r = _6049(r0, r1, r7, cache);
            cache.insert((r0 as u64) * 32768 + r1 as u64, r);
            return r;
        } else {
            r0 = (r0 + 32767) & 32767;
            r1 = r7;
            let r = _6049(r0, r1, r7, cache);
            cache.insert((r0 as u64) * 32768 + r1 as u64, r);
            return r;
        }
    } else {
        let r = (r1 + 1) & 32767;
        cache.insert((r0 as u64) * 32768 + r1 as u64, r);
        return r;
    }
}


fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let data: Vec<u8> = fs::read("challenge.bin")?;

    // {
    //     let mut cache: HashMap<u64, u16> = HashMap::new();
    //     let now = Instant::now();
    //     for i in 1..32768 {
    //         let r = _6049(4, 1, i, &mut cache);
    //         println!("{:?} {} {}s elapsed", i, r, now.elapsed().as_secs());
    //         if r == 6 {
    //             break; //25734
    //         }
    //         cache.clear();
    //     }
    //     return Ok(());
    // }

    // let mut numbers = vec![2, 3, 5, 7, 9];
    // loop {
    //     use rand::seq::SliceRandom;
    //     numbers.shuffle(&mut rand::rng());
    //
    //     if numbers[0] + numbers[1] * (numbers[2] * numbers[2]) + (numbers[3] * numbers[3] * numbers[3]) - numbers[4] == 399 {
    //         println!("{:?}", numbers);
    //         return Ok(());
    //     }
    // }

    let mut code_u16 = Vec::<u16>::with_capacity(data.len() / 2);
    for i in 0..data.len() / 2 {
        code_u16.push(read_u16(&data[i * 2..]));
    }

    // renderer_c::render(&code_u16, "dump.c")?;
    // if true {
    //     return Ok(());
    // }

    let mem = code_u16.as_mut_slice();
    let mut p = 0;
    let mut registers = [0u16; 8];
    let mut stack: Vec<u16> = Vec::new();
    let mut debug = false;
    let mut debug_r7 = false;
    let mut input: Vec<char> = Vec::new();

    let mut input_commands = vec![
        "doorway",
        "north",
        "north",
        "bridge",
        "continue",
        "down",
        "east",
        "take empty lantern",
        "west",
        "west",
        "west",
        "passage",
        "ladder",
        "west",
        "south",
        "north",
        "take can",
        "west",
        "use can",
        "ladder",
        "use lantern",
        "darkness",
        "continue",
        "west",
        "west",
        "west",
        "west",
        "north",
        "take red coin",
        "north",
        "east",
        "take concave coin",
        "down",
        "take corroded coin",
        "up",
        "west",
        "west",
        "take blue coin",
        "up",
        "take shiny coin",
        "down",
        "east",
        "use blue coin",
        "use red coin",
        "use shiny coin",
        "use concave coin",
        "use corroded coin",
        "north",
        "take teleporter",
        "use teleporter",
        "take business card",
        "take strange book",
        "look strange book",
    ];

    input_commands.reverse();

    let mut commands_after_teleport = vec![
        "north",
        "north",
        "north",
        "north",
        "north",
        "north",
        "north",
        "east",
        "take journal",
        "west",
        "north",
        "north",

        "take orb",
    ];
    commands_after_teleport.reverse();

    loop {
        let code = mem[p];
        let opcode = Opcode::of(code);
        // println!("{code} {opcode:?}");

        // if p == 5513 {
        //     println!("stop here 5513");
        // }
        // if p == 6064 {
        //     // println!("");
        //     // println!("stop here 6064");
        //
        //     // r0 = 4;
        //     // r1 = 1;
        //     // call(6049);
        //     // r1 = (r0 == 6) ? 1 : 0;
        //     // if (r1 == 0)  goto _5601;
        //     // println!("{}", render_c_to_string(&mem, p - 50, p + 30).unwrap());
        //     // if !debug_r7 {
        //     //     break;
        //     // }
        //     debug_r7 = false;
        // }

        p += 1;
        match opcode {
            Opcode::Noop => {}
            Opcode::Halt => {
                println!("HALT");
                break;
            }
            Opcode::Out => {
                let code = mem[p];
                let code = to_value(code, &registers);
                p += 1;
                if code > 128 {
                    panic!("char is too big: {code}");
                }
                print!("{}", code as u8 as char);
            }
            Opcode::Jmp => {
                let code = mem[p];
                if debug {
                    println!("Jmp to {} from {}", code, p);
                }
                p = code as usize;
            }
            Opcode::Jt => {
                let a = to_value(mem[p], &registers);
                p += 1;
                let b = mem[p];
                p += 1;
                if debug {
                    println!("Jt {a} to {} from {}", b, p);
                }
                if a != 0 {
                    p = b as usize;
                    // println!("jump")
                }
            }
            Opcode::Jf => {
                let a = to_value(mem[p], &registers);
                p += 1;
                let b = mem[p];
                p += 1;
                if debug {
                    println!("Jf {a} to {} from {}", b, p);
                }
                if a == 0 {
                    p = b as usize;
                    // println!("jump")
                }
            }
            Opcode::Set => {
                let a = to_index(mem[p]);
                p += 1;
                let b = to_value(mem[p], &registers);
                p += 1;
                if debug {
                    println!("Set {a} to {b}");
                }
                registers[a] = b;
            }
            Opcode::Add => {
                let a = to_index(mem[p]);
                p += 1;
                let b = to_value(mem[p], &registers);
                p += 1;
                let c = to_value(mem[p], &registers);
                p += 1;
                if debug {
                    println!("Add {b} + {c} to {a}");
                }
                registers[a] = (b + c) % 32768;
            }
            Opcode::Mult => {
                let a = to_index(mem[p]);
                p += 1;
                let b = to_value(mem[p], &registers);
                p += 1;
                let c = to_value(mem[p], &registers);
                p += 1;
                if debug {
                    println!("Mult {b} * {c} to {a}");
                }
                registers[a] = ((b as u32 * c as u32) % 32768) as u16;
            }
            Opcode::Mod => {
                let a = to_index(mem[p]);
                p += 1;
                let b = to_value(mem[p], &registers);
                p += 1;
                let c = to_value(mem[p], &registers);
                p += 1;
                if debug {
                    println!("Mod {b} % {c} to {a}");
                }
                registers[a] = b % c;
            }
            Opcode::Rmem => {
                let a = to_index(mem[p]);
                p += 1;
                let b = mem[p];
                let b = to_value(b, &registers);
                p += 1;
                let b = mem[b as usize];
                if debug {
                    println!("Rmem {b} to {a}");
                }
                registers[a] = b;
            }
            Opcode::Wmem => {
                let a = to_value(mem[p], &registers);
                p += 1;
                let b = to_value(mem[p], &registers);
                p += 1;
                if debug {
                    println!("Wmem {b} to {}", a);
                }
                mem[a as usize] = b;
            }
            Opcode::Eq => {
                let a = to_index(mem[p]);
                p += 1;
                let b = to_value(mem[p], &registers);
                p += 1;
                let c = to_value(mem[p], &registers);
                p += 1;
                if debug {
                    println!("Eq {b} == {c} to {a}");
                }
                registers[a] = if b == c { 1 } else { 0 };
            }
            Opcode::Gt => {
                let a = to_index(mem[p]);
                p += 1;
                let b = to_value(mem[p], &registers);
                p += 1;
                let c = to_value(mem[p], &registers);
                p += 1;
                if debug {
                    println!("Gt {b} > {c} to {a}");
                }
                registers[a] = if b > c { 1 } else { 0 };
            }
            Opcode::And => {
                let a = to_index(mem[p]);
                p += 1;
                let b = to_value(mem[p], &registers);
                p += 1;
                let c = to_value(mem[p], &registers);
                p += 1;
                if debug {
                    println!("And {b} & {c} to {a}");
                }
                registers[a] = b & c;
            }
            Opcode::Or => {
                let a = to_index(mem[p]);
                p += 1;
                let b = to_value(mem[p], &registers);
                p += 1;
                let c = to_value(mem[p], &registers);
                p += 1;
                if debug {
                    println!("Or {b} | {c} to {a}");
                }
                registers[a] = b | c;
            }
            Opcode::Not => {
                let a = to_index(mem[p]);
                p += 1;
                let b = to_value(mem[p], &registers);
                p += 1;
                if debug {
                    println!("Not !{b} to {a}");
                }
                registers[a] = (!b) & 32767;
            }
            Opcode::Push => {
                let value = to_value(mem[p], &registers);
                p += 1;
                if debug {
                    println!("Push {value}");
                }
                stack.push(value);
            }
            Opcode::Call => {
                let value = to_value(mem[p], &registers);
                p += 1;
                if debug {
                    println!("Call {value} from {p}");
                }
                stack.push(p as u16);
                p = value as usize;
            }
            Opcode::Ret => {
                if stack.is_empty() {
                    println!("Ret HALT");
                    break;
                }
                let address = stack.pop().unwrap();
                if debug {
                    println!("Ret {address}");
                }
                p = address as usize;
            }
            Opcode::Pop => {
                let a = to_index(mem[p]);
                p += 1;
                let value = stack.pop().expect("Pop called on empty stack");
                if debug {
                    println!("Pop {value} to {a}");
                }
                registers[a] = value;
            }
            Opcode::In => {
                let a = to_index(mem[p]);
                p += 1;
                if input.is_empty() {
                    let mut input_string = String::new();
                    if debug {
                        println!("In at {a}; {p}");
                    }
                    println!("waiting for input..");

                    if input_commands.is_empty().not() {
                        let c = input_commands.pop().unwrap();
                        println!("using input: {c}");
                        input = c.chars().collect();
                        input.push('\n');
                    } else {
                        if !debug_r7 {
                            // disable check
                            mem[5508] = 21;
                            mem[5509] = 21;
                            mem[5510] = 1;
                            mem[5511] = 32768;
                            mem[5512] = 6;

                            // renderer_c::render(&mem, "dump2.c")?;
                            debug_r7 = true;
                            registers[7] = 25734;

                            input = "use teleporter\n".chars().collect();
                            input_commands = commands_after_teleport.clone();
                        } else {
                            let _input_length = std::io::stdin()
                                .read_line(&mut input_string)
                                .expect("Failed to read input");
                            // println!("Input: {input_length} - {input} to {a}");
                            if input_string.starts_with("debug=true") {
                                debug = true;
                            }

                            input = input_string.chars().collect();
                        }
                    }

                    input.reverse()
                }

                registers[a] = input.pop().unwrap() as u16;
            }
        }
    }

    Ok(())
}

fn to_value(a: u16, registers: &[u16]) -> u16 {
    if a < 32768 { a } else { registers[to_index(a)] }
}
fn to_index(a: u16) -> usize {
    (a - 32768) as usize
}

fn read_u16(data: &[u8]) -> u16 {
    data[0] as u16 | ((data[1] as u16) << 8)
}
