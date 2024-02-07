pub mod operation;
use std::fmt;

use crate::controller::operation::Operation;

pub struct Controller {
    current: i64,
    first: Option<i64>,
    second: Option<i64>,
    operation: Option<Operation>,
}

impl fmt::Debug for Controller {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "curr={:?} first={:?} second={:?} op={:?}", self.current, self.first, self.second, self.operation)
    }
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            current: 0,
            first: None,
            second: None,
            operation: None,
        }
    }

    pub fn get_current(&self) -> i64 {
        self.current
    }

    pub fn add_digit(&mut self, digit: u8) {
        if let (Some(_), None) = (self.operation, self.first) {
            self.first = Some(self.current);
            self.current = 0;
        }
        if self.current > 1000000000000i64 {
            return;
        }
        self.current = self.current * 10 + digit as i64;
    }

    pub fn get_operation(&self) -> &Option<Operation> {
        &self.operation
    }

    pub fn set_operation(&mut self, operation: Operation) {
        if let (Some(_), Some(_), Some(_)) = (self.first, self.second, self.operation) {
            self.second = None;
            self.current = 0;
        }
        self.operation = Some(operation);
    }

    pub fn clear_current(&mut self) {
        self.current = 0;
    }

    pub fn reset(&mut self) {
        self.operation = None;
        self.first = None;
        self.second = None;
        self.current = 0;
    }

    pub fn calculate(&mut self) {
        if let None = self.first {
            return;
        }
        if let None = self.second {
            self.second = Some(self.current);
            self.current = 0;
        }
        if let (Some(first), Some(second), Some(op)) = (self.first, self.second, self.operation) {
            let result = op.operate(first, second);
            if result > 1000000000000i64 {
                self.reset();
                return;
            } else {
                self.current = result;
            }
            self.first = Some(result);
        }
    }
}
