/* ----------------------------------------------------------------------------

    MIT License

    Copyright (c) 2024 MW

    Permission is hereby granted, free of charge, to any person obtaining a
    copy of this software and associated documentation files (the "Software"),
    to deal in the Software without restriction, including without limitation
    the rights to use, copy, modify, merge, publish, distribute, sublicense,
    and/or sell copies of the Software, and to permit persons to whom the
    Software is furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in
    all copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
    FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
    DEALINGS IN THE SOFTWARE.

---------------------------------------------------------------------------- */

use std::fmt::{Debug, Display};
use std::ops::{Index};
use std::{env};
use std::vec::{Vec};


pub struct ArgsHelper {
    program_name: String,
    args: Vec<String>,
    options: Vec<String>
}

impl ArgsHelper {
    pub fn expect(arg_count: usize, message: &str) -> Self {
        let new = Self::init();

        if new.len() < arg_count {
            exit_with_error(format!("{}: {}", new.program_name, message));
            std::process::exit(0);
        }

        new
    }

    pub fn init() -> Self {
        let mut args = Vec::new();
        let mut options = Vec::new();

        let mut command_line = env::args();
        let program_name = command_line.next().unwrap_or_default();

        for arg in command_line {
            if ['-', '+'].contains(&arg.chars().nth(0).unwrap_or_default()) {
                options.push(arg);
            }
            else {
                args.push(arg);
            }
        }

        Self {
            program_name,
            args,
            options
        }
    }

    pub fn len(&self) -> usize {
        self.args.len()
    }

    pub fn has_option(&self, option: &str) -> bool {
        self.options.contains(&String::from(option))
    }
}

impl Index<usize> for ArgsHelper {
    type Output = String;

    fn index(&self, index: usize) -> &Self::Output {
        &self.args[index]
    }
}


//---------------------------------------------------------------------------//


pub fn exit_with_error(message: String) {
    eprintln!("{}", message);
    std::process::exit(1);
}

/// Extension trait for `Option'.
/// 
/// Simplifies error-handling where a `None` means that the program should
/// not continue.
pub trait OptionUnwrapExit<T> {
    fn unwrap_or_exit(self, message: String) -> T;    
}

impl<T> OptionUnwrapExit<T> for Option<T> {
    /// Attempts to unwrap an `Option' - prints a message to std-err and exits
    /// the program if the unwrap is not successful.
    fn unwrap_or_exit(self, message: String) -> T {
        match self {
            Some(value) => value,
            _ => {
                exit_with_error(message);
                self.unwrap()               // This line never reached - only here to satisfy return value!
            }
        }
    }
}

/// Extension trait for `Result`.
/// 
/// Simplifies error-handling where an `Err` means that the program should
/// not continue.
pub trait ResultUnwrapExit<T> {
    fn unwrap_or_exit(self, message: String) -> T;    
}

impl<T, E> ResultUnwrapExit<T> for Result<T, E> {
    /// Attempts to unwrap a `Result' - prints a message to std-err and exits
    /// the program if the unwrap is not successful.
    fn unwrap_or_exit(self, message: String) -> T {
        match self {
            Ok(value) => value,
            _ => {
                exit_with_error(message);
                panic!("");                 // This line never reached - only here to satisfy return value!
            }
        }
    }
}

/// Extension trait for `Option`.
/// 
/// Simplifies display of optional values.
pub trait OptionUnwrapDisplay<T> where T: Display {
    fn unwrap_display(&self) -> String;
    fn unwrap_display_or(&self, message: &str) -> String;
}

impl<T> OptionUnwrapDisplay<T> for Option<T> where T: Display {
    fn unwrap_display(&self) -> String {
        self.unwrap_display_or("None")
    }

    fn unwrap_display_or(&self, message: &str) -> String {
        match self {
            Some(x) => format!("{}", x),
            _ => String::from(message)
        }
    }
}