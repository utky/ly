use crate::core::lane;
use crate::core::task;

pub trait Row {
  fn as_row(&self) -> String;
}

impl Row for lane::Lane {
  fn as_row(&self) -> String {
    format!("{}\t{}", self.id, self.name)
  }
}

impl Row for task::Task {
  fn as_row(&self) -> String {
    format!("{}\t{}\t{}", self.id, self.lane_id, self.summary)
  }
}
