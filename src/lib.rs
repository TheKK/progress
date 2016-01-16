#![crate_name = "progress"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

//! **progress** is meant to be a set of useful tools for showing program running
//! progress (as its name) and steps.
//!
//! Installation
//! ============
//!
//! Add the following lines to your `Cargo.toml` dependencies section, if you
//! use [Cargo](https://crates.io):
//!
//! ```
//! [dependencies]
//! progress = "0.1.0"
//! ```
//!
//! Usage
//! =====
//!
//! Please check documentations for each structs. Life is easy here :)
//!
//! Who create this
//! ===============
//!
//! - [Ying-Ruei Liang (KK)](https://github.com/TheKK)
//!
//! Contribution
//! ============
//!
//! I can't believe you would say that, but if you have any great idea or any
//! bug report, don't be hesitate! It would be more wonderful if someone wants
//! to write some code for this project!
//!
//! TODO list
//! =========
//!
//! - BarBuilder, so we can do some customization, e.g. change the symbols used
//! - Add more type of indicators, e.g. spinning symbol or nayn cat :3
//! - Color/styled text support (print!("{:<50}") will count unprintable text as
//! well, I have to solve it first)
//! - Make output format customizable, despite I have no idea how to achieve this
//! for now.
//!
//! License
//! =======
//!
//! MIT

use std::io::{self, Write};

extern crate terminal_size;
use terminal_size::{terminal_size, Width};

/// A builder that used for creating customize progress bar.
///
/// # Examples
///
/// ```
/// use std::thread;
///
/// extern crate progress;
///
/// fn main() {
///     let mut bar = progress::BarBuilder::new()
///         .left_cap("<")
///         .right_cap(">")
///         .empty_symbol("-")
///         .filled_symbol("/")
///         .build();
///
///     bar.set_job_title("Meow...");
///
///     for i in 0..11 {
///         thread::sleep_ms(500);
///         bar.reach_percent(i * 10);
///     }
/// }
pub struct BarBuilder {
    _left_cap: Option<String>,
    _right_cap: Option<String>,
    _filled_symbol: Option<String>,
    _empty_symbol: Option<String>,
}

impl BarBuilder {
    /// Create a new progress bar builder.
    pub fn new() -> BarBuilder {
        BarBuilder {
            _left_cap: None,
            _right_cap: None,
            _filled_symbol: None,
            _empty_symbol: None,
        }
    }

    /// Set desired symbol used as left cap
    ///
    /// ```shell
    /// [=========-] 90%
    /// ^
    pub fn left_cap(&mut self, symbol: &str) -> &mut BarBuilder {
        self._left_cap = Some(symbol.to_string());

        self
    }

    /// Set desired symbol used as right cap
    ///
    /// ```shell
    /// [=========-] 90%
    ///            ^
    pub fn right_cap(&mut self, symbol: &str) -> &mut BarBuilder {
        self._right_cap = Some(symbol.to_string());

        self
    }

    /// Set desired symbol used as filled bar
    ///
    /// ```shell
    /// [=========-] 90%
    ///  ^^^^^^^^^
    pub fn filled_symbol(&mut self, symbol: &str) -> &mut BarBuilder {
        self._filled_symbol = Some(symbol.to_string());

        self
    }

    /// Set desired symbol used as empty bar
    ///
    /// ```shell
    /// [=========-] 90%
    ///           ^
    ///  ```
    pub fn empty_symbol(&mut self, symbol: &str) -> &mut BarBuilder {
        self._empty_symbol = Some(symbol.to_string());

        self
    }

    /// Build progress bar according to previous configurations.
    pub fn build(&mut self) -> Bar {
        // XXX Does `take()` appropriate way?
        Bar {
            _job_title: String::new(),
            _progress_percentage: 0,
            _left_cap: self._left_cap.take().unwrap_or(String::from("[")),
            _right_cap: self._right_cap.take().unwrap_or(String::from("]")),
            _filled_symbol: self._filled_symbol.take().unwrap_or(String::from("=")),
            _empty_symbol: self._empty_symbol.take().unwrap_or(String::from("-")),
        }
    }
}

/// Struct that used for presenting progress bar with plain texts.
///
/// # Examples
///
/// ```
/// use std::thread;
///
/// extern crate progress;
///
/// fn main() {
///     let bar = progress::Bar::new();
///
///     bar.set_job_title("Working...");
///
///     for i in 0..11 {
///         thread::sleep_ms(100);
///         bar.reach_percent(i * 10);
///     }
/// }
pub struct Bar {
    _job_title: String,
    _progress_percentage: i32,
    _left_cap: String,
    _right_cap: String,
    _filled_symbol: String,
    _empty_symbol: String,
}

impl Bar {
    /// Create a new progress bar.
    pub fn new() -> Bar {
        Bar {
            _job_title: String::new(),
            _progress_percentage: 0,
            _left_cap: String::from("["),
            _right_cap: String::from("]"),
            _filled_symbol: String::from("="),
            _empty_symbol: String::from("-"),
        }
    }

    /// Reset progress percentage to zero and job title to empty string. Also
    /// prints "\n".
    pub fn jobs_done(&mut self) {
        self._job_title.clear();
        self._progress_percentage = 0;

        print!("\n");
    }

    /// Set text shown in progress bar.
    pub fn set_job_title(&mut self, new_title: &str) {
        self._job_title.clear();
        self._job_title.push_str(new_title);
        self._show_progress();
    }

    /// Put progress to given percentage.
    pub fn reach_percent(&mut self, percent: i32) {
        self._progress_percentage = percent;
        self._show_progress();
    }

    /// Increase progress with given percentage.
    pub fn add_percent(&mut self, progress: i32) {
        self._progress_percentage += progress;
        self._show_progress();
    }
}

impl Bar {
    fn _show_progress(&self) {
        let width = if let Some((Width(w), _)) = terminal_size() {
            w as i32
        } else {
            81 as i32
        };
        let overhead = self._progress_percentage / 100;
        let left_percentage = self._progress_percentage - overhead * 100;
        let bar_len = width - (50 + 5) - 2;
        let bar_finished_len = ((bar_len as f32) *
                                (left_percentage as f32 / 100.0)) as i32;
        let filled_symbol = if overhead & 0b1 == 0 {
            &self._filled_symbol
        } else {
            &self._empty_symbol
        };
        let empty_symbol = if overhead & 0b1 == 0 {
            &self._empty_symbol
        } else {
            &self._filled_symbol
        };

        io::stdout().flush().unwrap();
        print!("\r");

        print!("{:<50}", self._job_title);
        print!("{}", self._left_cap);
        for _ in 0..bar_finished_len {
            print!("{}", filled_symbol);
        }
        for _ in bar_finished_len..bar_len {
            print!("{}", empty_symbol);
        }
        print!("{}", self._right_cap);
        print!("{:>4}%", self._progress_percentage);
    }
}

/// Struct that used for presenting progress with plain texts.
///
/// # Examples
///
/// ```
/// use std::thread;
///
/// extern crate progress;
///
/// fn main() {
///     let mut text = progress::Text::new();
///
///     text.set_job_title("Drawing...");
///     thread::sleep_ms(1000);
///
///     text.set_job_title("Painting...");
///     thread::sleep_ms(1000);
///
///     text.set_job_title("Sleeping zzz");
///     thread::sleep_ms(1000);
///
///     text.set_job_title("Wait! Is that a nyan cat?");
///     thread::sleep_ms(1000);
///
///     text.set_job_title("This must be my dream zzzzzz");
///     thread::sleep_ms(1000);
///
///     text.jobs_done();
/// }
pub struct Text {
    _job_title: String,
}

impl Text {
    /// Create a new progress text.
    pub fn new() -> Text {
        Text {
            _job_title: String::new(),
        }
    }

    /// Set text shown in progress text.
    pub fn set_job_title(&mut self, new_title: &str) {
        self._job_title.clear();
        self._job_title.push_str(new_title);
        self._show_progress();
    }

    /// Tell progress::Text everything has been done. Also prints "\n".
    pub fn jobs_done(&self) {
        print!("\n");
    }
}

impl Text {
    fn _show_progress(& self) {
        io::stdout().flush().unwrap();
        print!("\r");
        // TODO How to handle extra text?
        print!("{:<81}", self._job_title);
    }
}
