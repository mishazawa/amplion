extern crate cursive;
use cursive::Cursive;
use cursive::views::{
  LinearLayout,
  TextView,
  Button
};
use std::sync::mpsc::{self, Receiver, Sender};

#[derive(Debug)]
pub enum UiMessage {
  QUIT
}


#[derive(Debug)]
pub struct UiThread {
  channel: (Sender<UiMessage>, Receiver<UiMessage>)
}

impl UiThread {
  pub fn new () -> Self {
    UiThread {
      channel: mpsc::channel()
    }
  }

  pub fn receiver (&self) -> &Receiver<UiMessage> {
    &self.channel.1
  }

  pub fn sender (&self) -> &Sender<UiMessage> {
    &self.channel.0
  }

  pub fn run (sender: Sender<UiMessage>) {
    // Creates the cursive root - required for every application.
    let mut siv = Cursive::default();

    let quit_callback = move |s: &mut Cursive| {
      sender.send(UiMessage::QUIT).unwrap();
      s.quit()
    };

    siv.add_global_callback('q', quit_callback);
    let oscillators = LinearLayout::vertical()
      .child(TextView::new("Hello cursive! Press <q> to quit."))
      .child(TextView::new("Hello cursive! Press <q> to quit."))
      .child(TextView::new("Hello cursive! Press <q> to quit."));

    // Creates a dialog with a single "Quit" b
    siv.add_layer(oscillators);
    // Starts the event loop.
    siv.run()
  }
}
