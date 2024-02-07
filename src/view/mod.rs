use crate::controller::{operation::Operation, Controller};
use std::io::{stdin, stdout};
use termion::color::White;
use termion::{
    clear,
    color::{Bg, Reset},
    cursor,
    event::Key,
    input::TermRead,
    raw::IntoRawMode,
};

pub struct View<'a> {
    ctrl: &'a mut Controller,
    debug_mode: bool,
}

impl<'a> View<'a> {
    pub fn new(ctrl: &'a mut Controller) -> View {
        View {
            ctrl,
            debug_mode: false,
        }
    }

    fn fmt_op(&self, op: Operation) -> String {
        if self.ctrl.get_operation().is_some_and(|x| x == op) {
            format!("{}{}{}", Bg(White), op, Bg(Reset))
        } else {
            format!("{}", op)
        }
    }

    fn render(&self) {
        print!("{}{}", clear::All, cursor::Goto(1, 1));
        if self.debug_mode { print!("{:?}\r\n", self.ctrl); }
        print!("+---------------+\r\n");
        print!("| {: >13} |\r\n", self.ctrl.get_current());
        print!("+---------------+\r\n");
        print!("|  ON     C  CE |\r\n");
        print!("|  1   2   3  {} |\r\n", self.fmt_op(Operation::Multiply));
        print!("|  4   5   6  {} |\r\n", self.fmt_op(Operation::Divide));
        print!("|  7   8   9    |\r\n");
        print!("|  0   {}   {}  = |\r\n", self.fmt_op(Operation::Plus), self.fmt_op(Operation::Minus));
        print!("+---------------+\r\n");
        print!("> q to exit     <\r\n");
        print!("> d for debug   <\r\n");
    }

    pub fn keypress_loop(&mut self) {
        let stdin = stdin();
        let _stdout = stdout().into_raw_mode().unwrap();
        self.render();
        for key in stdin.keys() {
            match key.unwrap() {
                Key::Char(chr) => match chr {
                    chr @ '0'..='9' => {
                        let digit = chr.to_digit(10).unwrap();
                        self.ctrl.add_digit(digit as u8);
                    }
                    '+' => self.ctrl.set_operation(Operation::Plus),
                    '-' => self.ctrl.set_operation(Operation::Minus),
                    '*' => self.ctrl.set_operation(Operation::Multiply),
                    '/' => self.ctrl.set_operation(Operation::Divide),
                    '\n' => self.ctrl.calculate(),
                    'q' => break,
                    'd' => self.debug_mode = !self.debug_mode,
                    _ => {}
                },
                Key::Backspace => self.ctrl.clear_current(),
                Key::Delete => self.ctrl.reset(),
                _ => {}
            }
            self.render();
        }
    }
}
