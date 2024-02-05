use std::{fmt, str::FromStr};

use crate::options;

#[derive(Debug, PartialEq)]
pub(crate) struct CallbackKey {
  id: String,
  trigger: String,
}

impl CallbackKey {
  pub(crate) fn new(id: String, trigger: String) -> Self {
    Self { id, trigger }
  }

  pub(crate) fn id(&self) -> &str {
    &self.id
  }

  pub(crate) fn trigger(&self) -> &str {
    &self.trigger
  }

  pub(crate) fn is_finish_trigger(&self) -> bool {
    matches!(self.trigger(), options::ACTION | options::ON_DELETE)
  }
}

impl fmt::Display for CallbackKey {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}@{}", self.id, self.trigger)
  }
}

impl FromStr for CallbackKey {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let (id, trigger) =
      s.rsplit_once('@').ok_or_else(|| s.to_string())?;
    Ok(Self::new(id.to_owned(), trigger.to_owned()))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn se_de() {
    let key = CallbackKey::new("complex@id".into(), "trigger".into());
    let se = key.to_string();
    let de = CallbackKey::from_str(&se).unwrap();
    assert_eq!(key, de);
  }
}
