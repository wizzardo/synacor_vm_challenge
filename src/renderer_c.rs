use crate::Opcode;
use std::fs::File;

pub fn render(data: &[u16], name: &str) -> std::io::Result<()> {
    let file = File::create(name)?;
    use std::io::{BufWriter, Write};
    let mut writer = BufWriter::new(file);

    writeln!(writer, "#include <stdlib.h>")?;
    writeln!(writer, "#include <stdio.h>")?;
    writeln!(writer, "#include <string.h>")?;
    writeln!(writer, "")?;
    writeln!(writer, "short r0 = 0;")?;
    writeln!(writer, "short r1 = 0;")?;
    writeln!(writer, "short r2 = 0;")?;
    writeln!(writer, "short r3 = 0;")?;
    writeln!(writer, "short r4 = 0;")?;
    writeln!(writer, "short r5 = 0;")?;
    writeln!(writer, "short r6 = 0;")?;
    writeln!(writer, "short r7 = 0;")?;
    writeln!(writer, "short label_to_go = 0;")?;
    writeln!(writer, "int stack_pointer = 0;")?;
    writeln!(writer, "int stack_capacity = 1024;")?;
    writeln!(writer, "short stack[1024];")?;
    writeln!(writer, "")?;

    writeln!(
        writer,
        "void push_stack(short s) {{
  if (stack_pointer >= stack_capacity) {{ fputs(\"Error! Stack overflow!\\n\", stderr); exit(-1); }}
  stack[stack_pointer++] = s;
}}"
    )?;
    writeln!(writer, "")?;

    writeln!(
        writer,
        "short pop_stack() {{
  if (stack_pointer == 0) exit(-1);
  return stack[--stack_pointer];
}}"
    )?;
    writeln!(writer, "")?;
    writeln!(writer, "int main(void) {{")?;

    let mut pointer = 0;
    let mut labels_to_print = vec![];

    loop {
        if pointer >= data.len() {
            break;
        }

        let code = data[pointer];
        pointer += 1;

        if code > 21 {
            continue;
        }
        let opcode = Opcode::of(code);

        match opcode {
            Opcode::Jmp => {
                let arg = data[pointer];
                pointer += 1;
                labels_to_print.push(arg as usize);
            }
            Opcode::Call => {
                let arg = data[pointer];
                pointer += 1;
                labels_to_print.push(arg as usize);
            }
            Opcode::Jt => {
                pointer += 1;
                let arg = data[pointer];
                pointer += 1;
                labels_to_print.push(arg as usize);
            }
            Opcode::Jf => {
                pointer += 1;
                let arg = data[pointer];
                pointer += 1;
                labels_to_print.push(arg as usize);
            }
            _ => {
                for _ in 0..opcode.args() {
                    pointer += 1;
                }
            }
        }
    }

    let mut all_labels = vec![];
    let mut string_content = String::new();
    render_c_to_string_extended(
        data,
        0,
        data.len(),
        &mut string_content,
        &mut all_labels,
        &mut labels_to_print,
    )
    .unwrap();
    writeln!(writer, "{string_content}")?;

    writeln!(writer, "")?;
    writeln!(writer, "  labels:")?;
    writeln!(writer, "  switch (label_to_go) {{")?;
    for x in &all_labels {
        writeln!(writer, "    case {x}: goto _{x};")?;
    }
    writeln!(writer, "  }}")?;

    writeln!(writer, "}}")?;
    writer.flush()?;
    Ok(())
}

pub fn render_c_to_string(
    data: &[u16],
    from: usize,
    to: usize,
) -> Result<String, std::fmt::Error> {
    let mut all_labels = vec![];
    let mut labels_to_print = vec![];
    let mut string_content = String::new();
    render_c_to_string_extended(data, from, to, &mut string_content, &mut all_labels, &mut labels_to_print)?;
    Ok(string_content)
}

fn render_c_to_string_extended(
    data: &[u16],
    from: usize,
    to: usize,
    writer: &mut String,
    all_labels: &mut Vec<usize>,
    labels_to_print: &mut Vec<usize>,
) -> Result<(), std::fmt::Error> {
    use std::fmt::Write;
    let mut pointer = from;
    let mut string_to_print = String::new();
    loop {
        if pointer >= to {
            break;
        }

        let mut label_printed = false;
        if labels_to_print.contains(&pointer) {
            writeln!(writer, "  _{}:", pointer)?;
            all_labels.push(pointer);
            label_printed = true;
            let (i, _) = labels_to_print
                .iter()
                .enumerate()
                .find(|(_, p)| **p == pointer)
                .unwrap();
            labels_to_print.remove(i);
        }

        // if pointer/2 ==380 {
        //     println!("test");
        // }

        let code = data[pointer];
        pointer += 1;

        if code != 19 && string_to_print.len() > 0 {
            writeln!(writer, "  printf(\"{string_to_print}\");")?;
            string_to_print.clear();
        }

        if code > 21 {
            writeln!(writer, "  // {} // unknown data {code:?}", pointer - 1)?;
            continue;
        }
        // if code != 19 {
        // }
        if !label_printed {
            writeln!(writer, "  _{}:", pointer - 1)?;
            all_labels.push(pointer - 1);
        }
        let opcode = Opcode::of(code);
        // println!("{code} {opcode:?}");

        match opcode {
            Opcode::Halt => {
                writeln!(writer, "  exit(0);")?;
            }
            // Opcode::Noop => {}
            Opcode::Set => {
                let r = data[pointer];
                if r < 32768 {
                    write!(writer, "  //invalid set? {} // {opcode:?}", pointer)?;
                    write_commented_opcode(&data, writer, &mut pointer, &opcode)?;
                    continue;
                }
                let r = crate::to_index(r);
                pointer += 1;

                let a = data[pointer];
                pointer += 1;
                let a = to_dump_var(a);
                writeln!(writer, "  r{r} = {a};")?;
            }
            Opcode::Jmp => {
                // writeln!(writer, "  // {} // {opcode:?}", pointer)?;
                let arg = data[pointer];
                pointer += 1;
                writeln!(writer, "  goto _{};", { arg })?;
                // labels_to_print.push((arg) as usize);
            }
            Opcode::Jt => {
                // writeln!(writer, "  // {} // {opcode:?}", pointer)?;

                let a = data[pointer];
                let a = to_dump_var(a);
                pointer += 1;

                let arg = data[pointer];
                pointer += 1;
                writeln!(writer, "  if ({a} != 0)  goto _{};", { arg })?;
            }
            Opcode::Jf => {
                // writeln!(writer, "  // {} // {opcode:?}", pointer)?;

                let a = data[pointer];
                let a = to_dump_var(a);
                pointer += 1;

                let arg = data[pointer];
                pointer += 1;
                writeln!(writer, "  if ({a} == 0)  goto _{};", { arg })?;
            }
            Opcode::Add => {
                let a = data[pointer];
                if a < 32768 {
                    write!(writer, "  //invalid add? {} // {opcode:?}", pointer)?;
                    write_commented_opcode(&data, writer, &mut pointer, &opcode)?;
                    continue;
                }
                let a = crate::to_index(a);
                pointer += 1;
                let b = to_dump_var(data[pointer]);
                pointer += 1;
                let c = to_dump_var(data[pointer]);
                pointer += 1;
                writeln!(writer, "  r{a} = ({b} + {c}) % 32768;")?;
            }
            Opcode::Mult => {
                let a = data[pointer];
                if a < 32768 {
                    write!(writer, "  //invalid mult? {} // {opcode:?}", pointer)?;
                    write_commented_opcode(&data, writer, &mut pointer, &opcode)?;
                    continue;
                }
                let a = crate::to_index(a);
                pointer += 1;
                let b = to_dump_var(data[pointer]);
                pointer += 1;
                let c = to_dump_var(data[pointer]);
                pointer += 1;
                writeln!(writer, "  r{a} = ({b} * {c}) % 32768;")?;
            }
            Opcode::Mod => {
                let a = data[pointer];
                if a < 32768 {
                    write!(writer, "  //invalid mod? {} // {opcode:?}", pointer)?;
                    write_commented_opcode(&data, writer, &mut pointer, &opcode)?;
                    continue;
                }
                let a = crate::to_index(a);
                pointer += 1;
                let b = to_dump_var(data[pointer]);
                pointer += 1;
                let c = to_dump_var(data[pointer]);
                pointer += 1;
                writeln!(writer, "  r{a} = ({b} % {c});")?;
            }
            Opcode::And => {
                let a = data[pointer];
                if a < 32768 {
                    write!(writer, "  //invalid and? {} // {opcode:?}", pointer)?;
                    write_commented_opcode(&data, writer, &mut pointer, &opcode)?;
                    continue;
                }
                let a = crate::to_index(a);
                pointer += 1;
                let b = to_dump_var(data[pointer]);
                pointer += 1;
                let c = to_dump_var(data[pointer]);
                pointer += 1;
                writeln!(writer, "  r{a} = ({b} & {c});")?;
            }
            Opcode::Or => {
                let a = data[pointer];
                if a < 32768 {
                    write!(writer, "  //invalid or? {} // {opcode:?}", pointer)?;
                    write_commented_opcode(&data, writer, &mut pointer, &opcode)?;
                    continue;
                }
                let a = crate::to_index(a);
                pointer += 1;
                let b = to_dump_var(data[pointer]);
                pointer += 1;
                let c = to_dump_var(data[pointer]);
                pointer += 1;
                writeln!(writer, "  r{a} = ({b} | {c});")?;
            }
            Opcode::Not => {
                let a = data[pointer];
                if a < 32768 {
                    write!(writer, "  //invalid not? {} // {opcode:?}", pointer)?;
                    write_commented_opcode(&data, writer, &mut pointer, &opcode)?;
                    continue;
                }
                let a = crate::to_index(a);
                pointer += 1;
                let b = to_dump_var(data[pointer]);
                pointer += 1;
                writeln!(writer, "  r{a} = (~{b}) & 32767;")?;
            }
            Opcode::Eq => {
                let a = data[pointer];
                if a < 32768 {
                    write!(writer, "  //invalid eq? {} // {opcode:?}", pointer)?;
                    write_commented_opcode(&data, writer, &mut pointer, &opcode)?;
                    continue;
                }
                let a = crate::to_index(a);
                pointer += 1;
                let b = to_dump_var(data[pointer]);
                pointer += 1;
                let c = to_dump_var(data[pointer]);
                pointer += 1;
                writeln!(writer, "  r{a} = ({b} == {c}) ? 1 : 0;")?;
            }
            Opcode::Gt => {
                let a = data[pointer];
                if a < 32768 {
                    write!(writer, "  //invalid gt? {} // {opcode:?}", pointer)?;
                    write_commented_opcode(&data, writer, &mut pointer, &opcode)?;
                    continue;
                }
                let a = crate::to_index(a);
                pointer += 1;
                let b = to_dump_var(data[pointer]);
                pointer += 1;
                let c = to_dump_var(data[pointer]);
                pointer += 1;
                writeln!(writer, "  r{a} = ({b} > {c}) ? 1 : 0;")?;
            }
            Opcode::Push => {
                // writeln!(
                //     writer,
                //     "  if (stack_pointer >= stack_capacity) {{ fputs(\"Error! Stack overflow!\\n\", stderr); exit(-1); }}"
                // )?;
                let value = to_dump_var(data[pointer]);
                pointer += 1;
                // writeln!(writer, "  stack[stack_pointer++] = {value};")?;
                writeln!(writer, "  push_stack({value});")?;
            }
            Opcode::Pop => {
                // writeln!(writer, "  if (stack_pointer == 0) exit(-1);")?;
                let a = data[pointer];
                if a < 32768 {
                    write!(writer, "  //invalid pop? {} // {opcode:?}", pointer)?;
                    write_commented_opcode(&data, writer, &mut pointer, &opcode)?;
                    continue;
                }
                let a = crate::to_index(a);
                pointer += 1;
                // writeln!(writer, "  r{a} = stack[--stack_pointer];")?;
                writeln!(writer, "  r{a} = pop_stack();")?;
            }
            // Opcode::Rmem => {}
            // Opcode::Wmem => {}
            Opcode::Call => {
                let value = data[pointer];
                pointer += 1;
                // writeln!(
                //     writer,
                //     "  if (stack_pointer >= stack_capacity) {{ fputs(\"Error! Stack overflow!\\n\", stderr); exit(-1); }}"
                // )?;
                // writeln!(writer, "  stack[stack_pointer++] = {};", pointer)?;
                writeln!(writer, "  push_stack({pointer});")?;

                if value < 32768 {
                    labels_to_print.push((value) as usize);
                    writeln!(writer, "  goto _{value};")?;
                } else {
                    writeln!(writer, "  label_to_go  = r{};", value - 32768)?;
                    writeln!(writer, "  goto labels;")?;
                }
            }
            // Opcode::Ret => {}
            Opcode::Out => {
                let arg = data[pointer];
                pointer += 1;
                if arg == 10 {
                    string_to_print.push_str("\\n");
                    writeln!(writer, "  printf(\"{string_to_print}\");")?;
                    string_to_print.clear();
                } else {
                    string_to_print.push(arg as u8 as char);
                }
            }
            // Opcode::In => {}
            _ => {
                // write!(writer, "  // {} // {opcode:?}", pointer)?;
                // for _ in 0..opcode.args() {
                //     let arg = read_u16(&data[pointer..]);
                //     pointer += 1;
                //     // println!("arg {arg}");
                //     write!(writer, " {arg}")?;
                // }
                // write!(writer, "\n")?;
                write_commented_opcode(&data, writer, &mut pointer, &opcode)?;
            }
        }
    }
    Ok(())
}

fn to_dump_var(a: u16) -> String {
    if a < 32768 {
        a.to_string()
    } else {
        format!("r{}", a - 32768)
    }
}

fn write_commented_opcode(
    data: &[u16],
    writer: &mut String,
    pointer: &mut usize,
    opcode: &Opcode,
) -> Result<(), std::fmt::Error> {
    use std::fmt::Write;
    write!(writer, "  // {} // {opcode:?}", (*pointer - 1))?;
    for _ in 0..opcode.args() {
        let arg = data[*pointer];
        *pointer += 1;
        // println!("arg {arg}");
        write!(writer, " {arg}")?;
    }
    write!(writer, "\n")?;
    Ok(())
}
