const LANES: &str = "CREATE TABLE IF NOT EXISTS lanes (
  id INTEGER PRIMARY KEY,
  name VARCHAR NOT NULL UNIQUE,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
 )";

const LANES_INSERT: &str =
    "INSERT INTO lanes(id, name) VALUES (1, 'backlog'), (2, 'todo'), (3, 'done')";

const PRIORITIES: &str = "CREATE TABLE IF NOT EXISTS priorities (
  id INTEGER PRIMARY KEY,
  name VARCHAR NOT NULL UNIQUE,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
 )";

const PRIORITIES_INSERT: &str =
    "INSERT INTO priorities(id, name) VALUES (0, 'n'), (1, 'l'), (2, 'm'), (3, 'h')";

const TASKS: &str = "CREATE TABLE IF NOT EXISTS tasks (
  id INTEGER PRIMARY KEY,
  lane_id TINYINT,
  priority INTEGER NOT NULL,
  summary VARCHAR NOT NULL,
  estimate INTEGER NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (lane_id) REFERENCES lanes (id),
  FOREIGN KEY (priority) REFERENCES priorities (id)
)";

const PLANS: &str = "CREATE TABLE IF NOT EXISTS plans (
  date DATE PRIMARY KEY,
  note VARCHAR,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)";

const PLANNED_TASKS: &str = "CREATE TABLE IF NOT EXISTS planned_tasks (
  date DATE NOT NULL,
  task_id INTEGER NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (date, task_id),
  FOREIGN KEY (date) REFERENCES todo_today (date) ON DELETE CASCADE,
  FOREIGN KEY (task_id) REFERENCES tasks (id)
)";

const CURRENT: &str = "CREATE TABLE IF NOT EXISTS current (
  id INTEGER PRIMARY KEY,
  task_id INTEGER,
  started_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  duration_min INTEGER NOT NULL,
  FOREIGN KEY (task_id) REFERENCES tasks (id)
)";

const ESTIMATES: &str = "CREATE TABLE IF NOT EXISTS estimates (
  id INTEGER PRIMARY KEY,
  task_id INTEGER,
  value INTEGER NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (task_id) REFERENCES tasks (id)
 )";

const POMODOROS: &str = "CREATE TABLE IF NOT EXISTS pomodoros (
  id INTEGER PRIMARY KEY,
  task_id INTEGER,
  started_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  finished_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (task_id) REFERENCES tasks (id)
)";

const INTERRUPTIONS: &str = "CREATE TABLE IF NOT EXISTS interruptions (
  id INTEGER PRIMARY KEY,
  task_id INTEGER,
  external BOOLEAN,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (task_id) REFERENCES tasks (id)
)";

const TAGS: &str = "CREATE TABLE IF NOT EXISTS tags (
  id INTEGER PRIMARY KEY,
  name VARCHAR NOT NULL UNIQUE,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)";

const TAGGED_TASKS: &str = "CREATE TABLE IF NOT EXISTS tagged_tasks (
  tag_id INTEGER NOT NULL,
  task_id INTEGER NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (tag_id, task_id),
  FOREIGN KEY (tag_id) REFERENCES tags (id) ON DELETE CASCADE,
  FOREIGN KEY (task_id) REFERENCES tasks (id)
)";

pub const STATEMENTS: [&str; 13] = [
    LANES,
    LANES_INSERT,
    PRIORITIES,
    PRIORITIES_INSERT,
    TASKS,
    CURRENT,
    PLANS,
    PLANNED_TASKS,
    ESTIMATES,
    POMODOROS,
    INTERRUPTIONS,
    TAGS,
    TAGGED_TASKS,
];
