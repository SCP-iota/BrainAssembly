pub enum BrainfuckInstruction {
    Left,
    Right,
    Plus,
    Minus,
    Begin,
    End,
    Move(i32),
    Change(i32)
}

impl TryFrom<char> for BrainfuckInstruction {
    type Error = String;
    
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            '+' => Ok(Self::Plus),
            '-' => Ok(Self::Minus),
            '[' => Ok(Self::Begin),
            ']' => Ok(Self::End),
            _ => Err(format!("Unrecognized character: {}", c))
        }
    }
}

impl ToString for BrainfuckInstruction {
    fn to_string(&self) -> String {
        match *self {
            Self::Left => "<".to_string(),
            Self::Right => ">".to_string(),
            Self::Plus => "+".to_string(),
            Self::Minus => "-".to_string(),
            Self::Begin => "[".to_string(),
            Self::End => "]".to_string(),
            Self::Move(n) => {
                (if n >= 0 { ">" } else { "<" }).repeat(n as usize)
            },
            Self::Change(n) => {
                (if n >= 0 { "+" } else { "-" }).repeat(n.abs() as usize)
            }
        }
    }
}

pub struct BrainfuckCode (pub Vec<BrainfuckInstruction>);

impl BrainfuckCode {
    pub fn new_from_code(code: &str) -> Result<BrainfuckCode, String> {
        let mut iter = code.chars().map(|c| {
            BrainfuckInstruction::try_from(c)
        });

        if let Some(r) = iter.clone().find(|r| {
            if let Err(_) = r {
                true
            } else {
                false
            }
        }) {
            if let Err(e) = r {
                return Err(e);
            }
        }

        Ok(BrainfuckCode(iter.map(|r| { r.unwrap() }).collect()))
    }
}

impl ToString for BrainfuckCode {
    fn to_string(&self) -> String {
        self.0.iter().map(|i| { i.to_string() }).collect::<Vec<String>>().join("")
    }
}