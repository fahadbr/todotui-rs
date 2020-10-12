use crate::runner::Action;

use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use termion::event::Key;
use termion::input::TermRead;

pub trait Handler<I> {
    fn handle(&mut self, event: Event<I>) -> Option<Action>;
}

pub enum Event<I> {
    Input(I),
    _Tick,
}

pub struct Generator {
    rx: mpsc::Receiver<Event<Key>>,
    //input_handle: thread::JoinHandle<()>,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub exit_key: Key,
    pub tick_rate: Duration,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            exit_key: Key::Char('q'),
            tick_rate: Duration::from_millis(250),
        }
    }
}

impl Generator {
    pub fn new() -> Generator {
        Generator::with_config(Config::default())
    }

    pub fn with_config(config: Config) -> Generator {
        let (sender, receiver) = mpsc::channel();
        //let input_handle = {
        //let sender = sender.clone();
        thread::spawn(move || {
            let stdin = io::stdin();
            for evt in stdin.keys() {
                if let Ok(key) = evt {
                    sender.send(Event::Input(key)).unwrap();
                    if key == config.exit_key {
                        return;
                    }
                }
            }
        });

        //};
        Self {
            //input_handle,
            rx: receiver,
        }
    }

    pub fn next(&self) -> Result<Event<Key>, mpsc::RecvError> {
        self.rx.recv()
    }
}

