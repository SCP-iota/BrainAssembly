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
