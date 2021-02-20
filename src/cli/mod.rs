use crate::core::lane;
use crate::core::priority;
use crate::core::task;
use crate::core::Id;
use std::collections::HashMap;

pub struct TaskList<'l, 'p, 't> {
    lanes: HashMap<Id, &'l lane::Lane>,
    priorities: HashMap<Id, &'p priority::Priority>,
    tasks: &'t [task::Task],
}

static UNKNOWN: &str = "UNKNOWN";

impl<'l, 'p, 't> TaskList<'l, 'p, 't> {
    pub fn new(
        lanes: &'l [lane::Lane],
        priorities: &'p [priority::Priority],
        tasks: &'t [task::Task],
    ) -> TaskList<'l, 'p, 't> {
        let mut lane_hash = HashMap::with_capacity(lanes.len());
        let mut priority_hash = HashMap::with_capacity(priorities.len());
        for l in lanes {
            lane_hash.insert(l.id, l);
        }
        for p in priorities {
            priority_hash.insert(p.id, p);
        }
        TaskList {
            lanes: lane_hash,
            priorities: priority_hash,
            tasks,
        }
    }
    pub fn output(&self) -> String {
        let mut buffer = String::new();
        for t in self.tasks {
            let lane_name = self
                .lanes
                .get(&t.lane_id)
                .map(|l| l.name.as_ref())
                .unwrap_or(UNKNOWN);
            let priority_name = self
                .priorities
                .get(&t.priority)
                .map(|p| p.name.as_ref())
                .unwrap_or(UNKNOWN);
            buffer.push_str(
                format!(
                    "{}\t{}\t{}\t{}\t{}\n",
                    t.id, lane_name, priority_name, t.estimate, t.summary
                )
                .as_ref(),
            );
        }
        buffer
    }
}
