use std::fmt::Display;

#[derive(Copy, Clone)]
pub enum BrainfuckInstruction {
    Begin,
    End,
    Move(i32),
    Change(i32),
    Output,
    Input
}

impl TryFrom<char> for BrainfuckInstruction {
    type Error = String;
    
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '<' => Ok(Self::Move(-1)),
            '>' => Ok(Self::Move(1)),
            '+' => Ok(Self::Change(1)),
            '-' => Ok(Self::Change(-1)),
            '[' => Ok(Self::Begin),
            ']' => Ok(Self::End),
            '.' => Ok(Self::Output),
            ',' => Ok(Self::Input),
            _ => Err(format!("Unrecognized character: {}", c))
        }
    }
}

impl Display for BrainfuckInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match *self {
            Self::Move(n) => (if n >= 0 { ">" } else { "<" }).repeat(n.abs() as usize),
            Self::Change(n) => (if n >= 0 { "+" } else { "-" }).repeat(n.abs() as usize),
            Self::Begin => "[".to_string(),
            Self::End => "]".to_string(),
            Self::Output => ".".to_string(),
            Self::Input => ",".to_string()
        })
    }
}

pub struct BrainfuckCode (pub Vec<BrainfuckInstruction>);

enum OptimizerState {
    Translating,
    AccumulatingChange(i32),
    AccumulatingMove(i32)
}

impl BrainfuckCode {
    pub fn new_from_code(code: &str) -> Result<BrainfuckCode, String> {
        let iter = code.chars().map(|c| {
            BrainfuckInstruction::try_from(c)
        });

        if let Some(r) = iter.clone().find(|r| {
            r.is_err()
        }) {
            r?;
        }

        Ok(BrainfuckCode(iter.map(|r| { r.unwrap() }).collect()))
    }

    fn optimizer_handle_instruction(build: &mut Vec<BrainfuckInstruction>, instr: &BrainfuckInstruction, state: OptimizerState) -> OptimizerState {
        match state {
            OptimizerState::Translating => match instr {
                BrainfuckInstruction::Move(n) => OptimizerState::AccumulatingMove(*n),
                BrainfuckInstruction::Change(n) => OptimizerState::AccumulatingChange(*n),
                i => {
                    build.push(*i);
                    state
                }
            },
            OptimizerState::AccumulatingMove(curr) => match instr {
                BrainfuckInstruction::Move(n) => OptimizerState::AccumulatingMove(curr + n),
                i => {
                    build.push(BrainfuckInstruction::Move(curr));
                    BrainfuckCode::optimizer_handle_instruction(build, i, OptimizerState::Translating)
                }
            },
            OptimizerState::AccumulatingChange(curr) => match instr {
                BrainfuckInstruction::Change(n) => OptimizerState::AccumulatingChange(curr + n),
                i => {
                    build.push(BrainfuckInstruction::Change(curr));
                    BrainfuckCode::optimizer_handle_instruction(build, i, OptimizerState::Translating)
                }
            }
        }
    }

    pub fn optimize(&self) -> Self {
        let mut optimized: Vec<BrainfuckInstruction> = vec!();
        let mut state = OptimizerState::Translating;

        for instr in &self.0 {
            /*
            // START josh
            match instr {
                BrainfuckInstruction::Move(_) => {
                    
                },
                BrainfuckInstruction::Change(_) => todo!(),
                _ => optimized.push(*instr)
            }

            for (int i = 0; i <= len(arr); i++) {
                let instr = arr[i];

                if (it's not a move or change)                impl BrainfuckCode {
                    
                }
                    optimized.push(instr)
                else
                    start_i = i
                    while arr[i] is move
                        if arr[i] is moveLeft
                            i++
                        else
                            i--
                    optimized.push(Move(i - start_i))
            }



            // END josh
            */

            state = BrainfuckCode::optimizer_handle_instruction(&mut optimized, instr, state);
        }

        match state {
            OptimizerState::AccumulatingMove(n) => optimized.push(BrainfuckInstruction::Move(n)),
            OptimizerState::AccumulatingChange(n) => optimized.push(BrainfuckInstruction::Change(n)),
            _ => ()
        }

        Self (optimized)
    }

    pub fn optimize_better(&self) -> BrainfuckCode {
        let mut optimized = Vec::new();
        let mut move_accumulator = 0;
        let mut change_accumulator = 0;

        for instr in &self.0 {
            match instr {
                BrainfuckInstruction::Move(n) => {
                    if change_accumulator != 0 {
                        optimized.push(BrainfuckInstruction::Change(change_accumulator));
                        change_accumulator = 0;
                    }
                    move_accumulator += n;
                }
                BrainfuckInstruction::Change(n) => {
                    if move_accumulator != 0 {
                        optimized.push(BrainfuckInstruction::Move(move_accumulator));
                        move_accumulator = 0;
                    }
                    change_accumulator += n;
                }
                _ => {
                    if move_accumulator != 0 {
                        optimized.push(BrainfuckInstruction::Move(move_accumulator));
                        move_accumulator = 0;
                    }
                    if change_accumulator != 0 {
                        optimized.push(BrainfuckInstruction::Change(change_accumulator));
                        change_accumulator = 0;
                    }
                    optimized.push(*instr);
                }
            }
        }

        // Push any remaining accumulated instructions
        if move_accumulator != 0 {
            optimized.push(BrainfuckInstruction::Move(move_accumulator));
        }
        if change_accumulator != 0 {
            optimized.push(BrainfuckInstruction::Change(change_accumulator));
        }

        Self (optimized)
    }
}

impl Display for BrainfuckCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(|i| { i.to_string() }).collect::<Vec<String>>().join(""))
    }
}

// TODO: make enum for "codegen backends" (aka choosing between outputting to C, NASM, MASM, etc)
// and make a Trait for "this object can generate code"
// use std::fs::File;
// use std::io::prelude::*;
// use std::path::Path;

pub fn codegen_c(code: BrainfuckCode) -> String {
    let mut c_code = String::new();
    c_code.push_str("#include <stdio.h>\n#include <stdlib.h>\n\nint main() {\n");
    c_code.push_str("    char array[30000] = {0};\n");
    c_code.push_str("    char *ptr = array;\n\n");

    for instruction in code.0 {
        match instruction {
            BrainfuckInstruction::Move(n) => {
                if n > 0 {
                    c_code.push_str(&format!("    ptr += {};\n", n));
                } else {
                    c_code.push_str(&format!("    ptr -= {};\n", -n));
                }
            },
            BrainfuckInstruction::Change(n) => {
                if n > 0 {
                    c_code.push_str(&format!("    (*ptr) += {};\n", n));
                } else {
                    c_code.push_str(&format!("    (*ptr) -= {};\n", -n));
                }
            },
            BrainfuckInstruction::Begin => c_code.push_str("    while (*ptr) {\n"),
            BrainfuckInstruction::End => c_code.push_str("    }\n"),
            BrainfuckInstruction::Output => c_code.push_str("    putchar(*ptr);\n"),
            BrainfuckInstruction::Input => c_code.push_str("    *ptr = getchar();\n"),
        }
    }

    c_code.push_str("\n    return 0;\n}");
    c_code
    
    // Output to file
    
    // let path = Path::new("output.c");
    // let display = path.display();

    // // Open a file in write-only mode, returns `io::Result<File>`
    // let mut file = match File::create(&path) {
    //     Err(why) => panic!("couldn't create {}: {}", display, why),
    //     Ok(file) => file,
    // };

    // // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    // match file.write_all(c_code.as_bytes()) {
    //     Err(why) => panic!("couldn't write to {}: {}", display, why),
    //     Ok(_) => println!("successfully wrote to {}", display),
    // }
}

pub enum AssemblyStyle {
    NASM,
    MASM,
}

pub fn codegen_assembly(code: BrainfuckCode, style: AssemblyStyle) -> String {
    let mut assembly_code = String::new();
    
    match style {
        AssemblyStyle::NASM => {
            assembly_code.push_str("section .data\n");
            assembly_code.push_str("    fmt db \"%c\", 0\n");
            assembly_code.push_str("    fmt_in db \"%c\", 0\n");
            
            assembly_code.push_str("section .text\n");
            assembly_code.push_str("    extern printf, scanf, malloc\n");
            assembly_code.push_str("    global main\n");
            
            assembly_code.push_str("main:\n");
            assembly_code.push_str("    ; Allocate memory for the Brainfuck array\n");
            assembly_code.push_str("    mov rcx, 30000\n");
            assembly_code.push_str("    call malloc\n");
            assembly_code.push_str("    mov rbx, rax\n");
        },
        AssemblyStyle::MASM => {
            assembly_code.push_str(".data\n");
            assembly_code.push_str("fmt db \"%c\", 0\n");
            assembly_code.push_str("fmt_in db \"%c\", 0\n");
            
            assembly_code.push_str(".code\n");
            assembly_code.push_str("extern printf : PROC\n");
            assembly_code.push_str("extern scanf : PROC\n");
            assembly_code.push_str("extern malloc : PROC\n");
            assembly_code.push_str("main PROC\n");
            
            assembly_code.push_str("    ; Allocate memory for the Brainfuck array\n");
            assembly_code.push_str("    push 30000\n");
            assembly_code.push_str("    call malloc\n");
            assembly_code.push_str("    mov ebx, eax\n");
        },
    }
    
    let mut loop_label_stack: Vec<usize> = vec!();

    for instruction in &code.0 {
        match instruction {
            BrainfuckInstruction::Move(n) => {
                let (reg, op) = if *n > 0 { ("add", *n) } else { ("sub", -n) };
                match style {
                    AssemblyStyle::NASM => assembly_code.push_str(&format!("    {} rbx, {}\n", reg, op)),
                    AssemblyStyle::MASM => assembly_code.push_str(&format!("    {} ebx, {}\n", reg, op)),
                }
            },
            BrainfuckInstruction::Change(n) => {
                let (reg, op) = if *n > 0 { ("add", *n) } else { ("sub", -n) };
                match style {
                    AssemblyStyle::NASM => assembly_code.push_str(&format!("    {} byte [rbx], {}\n", reg, op)),
                    AssemblyStyle::MASM => assembly_code.push_str(&format!("    {} BYTE PTR [ebx], {}\n", reg, op)),
                }
            },
            BrainfuckInstruction::Begin => match style {
                AssemblyStyle::NASM => assembly_code.push_str("    cmp byte [rbx], 0\n    je end_loop\nstart_loop:\n"),
                AssemblyStyle::MASM => {
                    let loop_label_number = assembly_code.len();
                    loop_label_stack.push(loop_label_number);
                    let loop_label = format!("loop_start_{}", loop_label_number);
                    let end_label = format!("loop_end_{}", loop_label_number);
                    assembly_code.push_str(&format!("{}:\n", loop_label));
                    assembly_code.push_str("    cmp BYTE PTR [ebx], 0\n");
                    assembly_code.push_str(&format!("    je {}\n", end_label));
                },
            },
            BrainfuckInstruction::End => match style {
                AssemblyStyle::NASM => assembly_code.push_str("    cmp byte [rbx], 0\n    jne start_loop\nend_loop:\n"),
                AssemblyStyle::MASM => {
                    let loop_label_number = loop_label_stack.pop().unwrap();
                    let loop_label = format!("loop_start_{}", loop_label_number);
                    let end_label = format!("loop_end_{}", loop_label_number);
                    assembly_code.push_str("    cmp BYTE PTR [ebx], 0\n");
                    assembly_code.push_str(&format!("    jne {}\n", loop_label));
                    assembly_code.push_str(&format!("{}:\n", end_label));
                },
            },
            BrainfuckInstruction::Output => match style {
                AssemblyStyle::NASM => {
                    assembly_code.push_str("    movzx rdx, byte [rbx]\n");
                    assembly_code.push_str("    mov rcx, [rel fmt]\n");
                    assembly_code.push_str("    sub esp, 8\n");
                    assembly_code.push_str("    call printf\n");
                    assembly_code.push_str("    add esp, 8\n");
                },
                AssemblyStyle::MASM => {
                    assembly_code.push_str("    movzx eax, BYTE PTR [ebx]\n");
                    assembly_code.push_str("    push eax\n");
                    assembly_code.push_str("    push OFFSET fmt\n");
                    assembly_code.push_str("    call printf\n");
                    assembly_code.push_str("    add esp, 8\n");
                },
            },
            BrainfuckInstruction::Input => match style {
                AssemblyStyle::NASM => {
                    assembly_code.push_str("    mov rdx, rbx\n");
                    assembly_code.push_str("    mov rcx, [rel fmt_in]\n");
                    assembly_code.push_str("    sub esp, 8\n");
                    assembly_code.push_str("    call scanf\n");
                    assembly_code.push_str("    add esp, 8\n");
                },
                AssemblyStyle::MASM => {
                    assembly_code.push_str("    push ebx\n");
                    assembly_code.push_str("    push OFFSET fmt_in\n");
                    assembly_code.push_str("    call scanf\n");
                    assembly_code.push_str("    add esp, 8\n");
                },
            },
        }
    }
    
    match style {
        AssemblyStyle::NASM => {
            assembly_code.push_str("ret    ; Exit the program\n");
        },
        AssemblyStyle::MASM => {
            assembly_code.push_str("    ; Exit the program\n");
            assembly_code.push_str("    push 0\n");
            assembly_code.push_str("    call ExitProcess\n");
            assembly_code.push_str("main ENDP\n");
            assembly_code.push_str("END main\n");
        },
    }
    
    assembly_code
}

