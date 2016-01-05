#![crate_name = "progress"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

use std::io::{self, Write};

extern crate terminal_size;
use terminal_size::{terminal_size, Width};

/// Struct that used for presenting progress bar with plain texts.
///
/// # Examples
///
/// ```
/// use std::thread;
///
/// extern crate progress;
///
/// let bar = progress::Bar::new();
///
/// bar.set_job_title("Working...");
///
/// for i in 0..11 {
///     thread::sleep_ms(100);
///     bar.reach_percent(i * 10);
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
