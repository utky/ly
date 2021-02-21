use crate::core::lane;
use crate::core::priority;
use crate::core::task;
use crate::core::todo;
use crate::core::Id;
use std::collections::HashMap;

static UNKNOWN: &str = "UNKNOWN";

pub trait FormatWithLanePriority {
    fn format(
        &self,
        lanes: &HashMap<Id, &lane::Lane>,
        priorities: &HashMap<Id, &priority::Priority>,
    ) -> String;
}

impl FormatWithLanePriority for task::Task {
    fn format(
        &self,
        lanes: &HashMap<Id, &lane::Lane>,
        priorities: &HashMap<Id, &priority::Priority>,
    ) -> String {
        let lane_name = lanes
            .get(&self.lane_id)
            .map(|l| l.name.as_ref())
            .unwrap_or(UNKNOWN);
        let priority_name = priorities
            .get(&self.priority)
            .map(|p| p.name.as_ref())
            .unwrap_or(UNKNOWN);
        format!(
            "{}\t{}\t{}\t{}\t{}",
            self.id, lane_name, priority_name, self.estimate, self.summary
        )
    }
}

impl FormatWithLanePriority for todo::TodoTask {
    fn format(
        &self,
        lanes: &HashMap<Id, &lane::Lane>,
        priorities: &HashMap<Id, &priority::Priority>,
    ) -> String {
        let lane_name = lanes
            .get(&self.lane_id)
            .map(|l| l.name.as_ref())
            .unwrap_or(UNKNOWN);
        let priority_name = priorities
            .get(&self.priority)
            .map(|p| p.name.as_ref())
            .unwrap_or(UNKNOWN);
        format!(
            "{}\t{}\t{}\t{}/{}\t{}",
            self.task_id, lane_name, priority_name, self.actual, self.estimate, self.summary
        )
    }
}

pub struct TaskContext<'l, 'p> {
    lanes: HashMap<Id, &'l lane::Lane>,
    priorities: HashMap<Id, &'p priority::Priority>,
}

impl<'l, 'p> TaskContext<'l, 'p> {
    pub fn new(
        lanes: &'l [lane::Lane],
        priorities: &'p [priority::Priority],
    ) -> TaskContext<'l, 'p> {
        let mut lane_hash = HashMap::with_capacity(lanes.len());
        let mut priority_hash = HashMap::with_capacity(priorities.len());
        for l in lanes {
            lane_hash.insert(l.id, l);
        }
        for p in priorities {
            priority_hash.insert(p.id, p);
        }
        TaskContext {
            lanes: lane_hash,
            priorities: priority_hash,
        }
    }
    pub fn format<T: FormatWithLanePriority>(&self, target: T) -> String {
        target.format(&self.lanes, &self.priorities)
    }
}
