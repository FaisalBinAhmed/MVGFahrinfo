use anyhow::Result;
use crossterm::event::{self, Event as CrosstermEvent, KeyEvent};
use std::time::{Duration, Instant};

/// reference: https://ratatui.rs/tutorial/counter-app/event.html

#[derive(Clone, Copy, Debug)]
pub enum Event {
    Tick,
    Key(KeyEvent),
}

#[derive(Debug)]
pub struct EventHandler {
    pub sender: tokio::sync::mpsc::UnboundedSender<Event>,
    receiver: tokio::sync::mpsc::UnboundedReceiver<Event>,
}

impl EventHandler {
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
        let _handler = {
            let sender = sender.clone();
            tokio::spawn(async move {
                let mut last_tick = Instant::now();
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);

                    if event::poll(timeout).expect("no events available") {
                        match event::read().expect("unable to read event") {
                            CrosstermEvent::Key(e) => {
                                if e.kind == event::KeyEventKind::Press {
                                    sender.send(Event::Key(e))
                                } else {
                                    Ok(()) // ignore KeyEventKind::Release on windows
                                }
                            }
                            _ => {
                                // ignore other events
                                Ok(())
                            }
                        }
                        .expect("failed to send terminal event")
                    }

                    if last_tick.elapsed() >= tick_rate {
                        sender.send(Event::Tick).expect("failed to send tick event");
                        last_tick = Instant::now();
                    }
                }
            })
        };
        Self { sender, receiver }
    }

    pub async fn next(&mut self) -> Result<Event> {
        self.receiver
            .recv()
            .await
            .ok_or_else(|| anyhow::anyhow!("failed to receive event from event handler"))
    }
}
