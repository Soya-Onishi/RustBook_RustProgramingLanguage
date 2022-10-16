pub trait Messenger {
  fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
  messenger: &'a T,
  value: usize,
  max: usize,
}

impl<'a, T> LimitTracker<'a, T> 
  where T: Messenger
{
  pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
    LimitTracker { messenger, value: 0, max }
  }

  pub fn set_value(&mut self, value: usize) {
    self.value = value;

    let percentage_of_max = self.value as f64 / self.max as f64;

    if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
      self.messenger.send("Warning");
    } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
      self.messenger.send("Urgent Warning");
    } else if percentage_of_max >= 1.0 {
      self.messenger.send("Error");
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::cell::RefCell;

  struct MockMessenger {
    sent_messages: RefCell<Vec<String>>
  }

  impl MockMessenger {
    fn new() -> MockMessenger {
      MockMessenger { sent_messages: RefCell::new(Vec::new()) }
    }
  }

  impl Messenger for MockMessenger {
    fn send(&self, msg: &str) {                
        // 以下コメントアウトを外すと
        // 借用規則を違反したと判断されpanicになる
        // let _borrow_mut1 = self.sent_messages.borrow_mut();        
        // let _borrow_mut2 = self.sent_messages.borrow();
        self.sent_messages.borrow_mut().push(String::from(msg));
    }
  }

  #[test]
  fn it_sends_and_warning() {
    let mock = MockMessenger::new();
    let mut tracker = LimitTracker::new(&mock, 100);

    tracker.set_value(80);

    assert_eq!(mock.sent_messages.borrow().len(), 1);
  }
}