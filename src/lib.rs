use std::thread;
use std::io::{self, Write};
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;

#[test]
fn it_works() {
}

const STOP_MAGIC: &'static str = "cAtStOp";

pub struct Pbar {
    thread_handler: Option<thread::JoinHandle<()>>,
    tx: Sender<String>,
    current_progress: i32,
}

impl Pbar {
    pub fn new(goal: i32) -> Pbar {
        let (tx, rx) = channel::<String>();
        let handler = thread::spawn(move || {
            let mut job_title = String::new();
            let mut percent: i32 = 0;
            let rx = rx;
            let goal = goal;

            fn show_progress(percent: i32, job_title: &String, bar_len: i32) {
                let sym_to_draw = (bar_len as f32) * (percent as f32 / 100.0);
                let sym_to_draw = sym_to_draw as i32;

                io::stdout().flush().unwrap();
                print!("\r");

                print!("{:<30}", job_title);
                print!("[");
                for _ in 0..sym_to_draw {
                    print!("=");
                }
                for _ in sym_to_draw..bar_len {
                    print!("-");
                }
                print!("]");
            };

            while let Ok(packet) = rx.recv() {
                match packet.as_ref() {
                    STOP_MAGIC => {
                        print!("\n");
                        return;
                    },
                    _ if packet.trim().parse::<f32>().is_ok() => {
                        percent = packet.trim().parse::<i32>().unwrap();
                        show_progress(percent, &job_title, goal);
                    },
                    _ => {
                        job_title.clone_from(&packet);
                        show_progress(percent, &job_title, goal);
                    },
                }
            }
        });

        Pbar {
            thread_handler: Some(handler),
            tx: tx,
            current_progress: 0,
        }
    }

    pub fn set_job_title(&mut self, title: &String) {
        self.tx.send(title.clone()).unwrap();
    }

    pub fn reach_percent(&mut self, percent: i32) {
        self.current_progress = percent;
        self.tx.send(format!("{}", percent)).unwrap();
    }

    pub fn add_percent(&mut self, progress: i32) {
        self.current_progress += progress;
        self.tx.send(format!("{}", self.current_progress)).unwrap();
    }
}

impl Drop for Pbar {
    fn drop(&mut self) {
        let join_handler = self.thread_handler.take().unwrap();

        self.tx.send(STOP_MAGIC.to_string()).unwrap();
        join_handler.join().unwrap();
    }
}
