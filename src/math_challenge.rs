use ::rand::{thread_rng, Rng};

#[derive(Clone, Debug)]
pub enum MathOperation {
    Addition,
    Subtraction,
    Multiplication,
}

#[derive(Clone, Debug)]
pub struct MathChallenge {
    pub num1: i32,
    pub num2: i32,
    pub operation: MathOperation,
    pub correct_answer: i32,
    pub user_input: String,
    pub is_completed: bool,
    pub is_correct: bool,
}

impl MathChallenge {
    pub fn new_random() -> Self {
        let mut rng = thread_rng();
        let operation = match rng.gen_range(0..3) {
            0 => MathOperation::Addition,
            1 => MathOperation::Subtraction,
            _ => MathOperation::Multiplication,
        };
        
        let (num1, num2, correct_answer) = match operation {
            MathOperation::Addition => {
                let a = rng.gen_range(10..100);
                let b = rng.gen_range(10..100);
                (a, b, a + b)
            }
            MathOperation::Subtraction => {
                let a = rng.gen_range(20..100);
                let b = rng.gen_range(10..a);
                (a, b, a - b)
            }
            MathOperation::Multiplication => {
                let a = rng.gen_range(2..10);
                let b = rng.gen_range(2..10);
                (a, b, a * b)
            }
        };
        
        Self {
            num1,
            num2,
            operation,
            correct_answer,
            user_input: String::new(),
            is_completed: false,
            is_correct: false,
        }
    }
    
    pub fn get_question_text(&self) -> String {
        let op_symbol = match self.operation {
            MathOperation::Addition => "+",
            MathOperation::Subtraction => "-",
            MathOperation::Multiplication => "×",
        };
        format!("{} {} {} = ?", self.num1, op_symbol, self.num2)
    }
    
    pub fn add_digit(&mut self, digit: char) {
        if self.user_input.len() < 4 && digit.is_ascii_digit() {
            self.user_input.push(digit);
        }
    }
    
    pub fn remove_digit(&mut self) {
        self.user_input.pop();
    }
    
    pub fn submit_answer(&mut self) -> bool {
        if let Ok(answer) = self.user_input.parse::<i32>() {
            self.is_correct = answer == self.correct_answer;
            self.is_completed = true;
            self.is_correct
        } else {
            false
        }
    }
    
    pub fn get_user_answer(&self) -> &str {
        &self.user_input
    }
}