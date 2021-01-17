const LANES: &'static str = "CREATE TABLE IF NOT EXISTS lanes (
  id INTEGER PRIMARY KEY,
  name VARCHAR NOT NULL UNIQUE,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
 )";

const LANES_INSERT: &'static str = "INSERT INTO lanes(id, name) VALUES (1, 'backlog'), (2, 'todo')";

const PRIORITIES: &'static str = "CREATE TABLE IF NOT EXISTS priorities (
  id INTEGER PRIMARY KEY,
  name VARCHAR NOT NULL UNIQUE,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
 )";

const PRIORITIES_INSERT: &'static str = "INSERT INTO priorities(id, name) VALUES (0, 'n'), (1, 'l'), (2, 'm'), (3, 'h')";

const TASKS: &'static str = "CREATE TABLE IF NOT EXISTS tasks (
  id INTEGER PRIMARY KEY,
  lane_id TINYINT,
  priority INTEGER NOT NULL,
  summary VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (lane_id) REFERENCES lanes (id),
  FOREIGN KEY (priority) REFERENCES priorities (id)
)";

const CURRENT: &'static str = "CREATE TABLE IF NOT EXISTS current (
  id INTEGER PRIMARY KEY,
  task_id INTEGER,
  started_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (task_id) REFERENCES tasks (id)
)";

const ARCHIVES: &'static str = "CREATE TABLE IF NOT EXISTS archives (
  id INTEGER PRIMARY KEY,
  lane_id INTEGER,
  summary VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (lane_id) REFERENCES lanes (id)
)";

const ESTIMATES: &'static str = "CREATE TABLE IF NOT EXISTS estimates (
  id INTEGER PRIMARY KEY,
  task_id INTEGER,
  value INTEGER NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (task_id) REFERENCES tasks (id)
 )";

const POMODOROS: &'static str = "CREATE TABLE IF NOT EXISTS pomodoros (
  id INTEGER PRIMARY KEY,
  task_id INTEGER,
  started_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  finished_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (task_id) REFERENCES tasks (id)
)";

const INTERRUPTIONS: &'static str = "CREATE TABLE IF NOT EXISTS interruptions (
  id INTEGER PRIMARY KEY,
  task_id INTEGER,
  external BOOLEAN,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (task_id) REFERENCES tasks (id)
)";

pub const STATEMENTS: [&str; 10] = [
  LANES,
  LANES_INSERT,
  PRIORITIES,
  PRIORITIES_INSERT,
  TASKS,
  CURRENT,
  ARCHIVES,
  ESTIMATES,
  POMODOROS,
  INTERRUPTIONS,
];
