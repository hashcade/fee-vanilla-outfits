use std::cell::Cell;

std::thread_local! {
  static UNIT: Cell<usize> = const { Cell::new(0) };
}

/// A synchronous combat appearance request; never a cached game object.
pub struct CombatScope(usize);

impl CombatScope {
  pub fn enter(unit: usize) -> Self {
    Self(UNIT.with(|current| current.replace(unit)))
  }
}

impl Drop for CombatScope {
  fn drop(&mut self) {
    UNIT.with(|current| current.set(self.0));
  }
}

pub fn contains(unit: usize) -> bool {
  unit != 0 && UNIT.with(|current| current.get() == unit)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn scope_is_nested_and_cleared() {
    assert!(!contains(1));
    {
      let _outer = CombatScope::enter(1);
      assert!(contains(1));
      {
        let _inner = CombatScope::enter(2);
        assert!(!contains(1));
        assert!(contains(2));
      }
      assert!(contains(1));
      assert!(!contains(2));
    }
    assert!(!contains(1));
    assert!(!contains(0));
  }

  #[test]
  fn scope_does_not_leak_to_another_thread() {
    let _scope = CombatScope::enter(1);
    assert!(!std::thread::spawn(|| contains(1)).join().unwrap());
  }
}
