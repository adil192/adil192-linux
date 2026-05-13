use anyhow::Result;

use crate::my_css::MyCss;

impl MyCss {
  pub fn theme_firefox() -> Result<()> {
    assert!(Self::enabled()?);
    todo!()
  }
}
