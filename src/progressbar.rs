use indicatif::{ProgressBar, ProgressStyle};
use std::sync::mpsc::{Sender, Receiver, RecvError, SendError, channel};
use std::thread;

pub enum Stage {
    Init(u64),
    Start(String, u64),
    Inc(u64),
    Finish(String),
    Stop,
}

pub trait IncSignal {
    fn inc(&self, delta: u64);
}

pub struct ProgressSignal {
    tx: Sender<Stage>,
}

impl ProgressSignal {
    pub fn init(&self, stages: u64) -> Result<(), SendError<Stage>> {
        self.tx.send(Stage::Init(stages))
    }

    pub fn start(&self, message: &str, max: u64) -> Result<(), SendError<Stage>> {
        self.tx.send(Stage::Start(message.to_string(), max))
    }

    pub fn inc(&self, delta: u64) -> Result<(), SendError<Stage>> {
        self.tx.send(Stage::Inc(delta))
    }

    pub fn finish(&self, message: &str) -> Result<(), SendError<Stage>> {
        self.tx.send(Stage::Finish(message.to_string()))
    }

    pub fn stop(&self) -> Result<(), SendError<Stage>> {
        self.tx.send(Stage::Stop)
    }
}

impl IncSignal for ProgressSignal {
    fn inc(&self, delta: u64) {
        self.inc(delta);
    }
}

impl Drop for ProgressSignal {
    fn drop(&mut self) {
        self.stop();
    }
}

pub struct Progress {
    stages: u64,
    index: u64,
}

impl Progress{
    pub fn start() -> ProgressSignal {
        let (tx, rx): (Sender<Stage>, Receiver<Stage>) = channel();

        let mut progress = Progress {
            stages: 0,
            index: 0,
        };

        thread::spawn(move || {
            if let Err(err) = progress.thread_loop(rx) {
                println!("break with {:?}", err);
            }
        });

        ProgressSignal {
            tx
        }
    }

    fn thread_loop(&mut self, rx: Receiver<Stage>) -> Result<(), RecvError> {
        let mut progress_bar = ProgressBar::new(100);

        loop {
            match rx.recv()? {
                Stage::Init(stages) => {
                    self.stages = stages;
                },
                Stage::Start(message, max) => {
                    self.index += 1;

                    progress_bar = ProgressBar::new(max);
                    progress_bar.set_prefix(&format!("[{}/{}]", self.index, self.stages));
                    progress_bar.set_message(&message);
                    progress_bar.set_style(ProgressStyle::default_bar()
                        .template("{prefix} {msg} {spinner:.green} [{elapsed_precise} / {eta}] [{bar:80.cyan/blue}] {percent}%")
                        .progress_chars("#>-"));
                },
                Stage::Inc(delta) => {
                    progress_bar.inc(delta);
                },
                Stage::Finish(message) => {
                    progress_bar.finish_with_message(&message);
                }
                Stage::Stop => break,
            }
        }

        Ok(())
    }
}
