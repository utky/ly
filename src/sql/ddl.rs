const LANES: &'static str = "CREATE TABLE IF NOT EXISTS lanes (
  id tinyint,
  name varchar NOT NULL UNIQUE,
  created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (id)
 )";

const LANES_INSERT: &'static str = "INSERT INTO lanes(id, name) VALUES (1, 'backlog'), (2, 'todo')";

const TASKS: &'static str = "CREATE TABLE IF NOT EXISTS tasks (
  id integer AUTO_INCREMENT,
  uuid varchar(36) NOT NULL UNIQUE,
  lane_id tinyint,
  summary varchar NOT NULL,
  created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (id),
  FOREIGN KEY (lane_id) REFERENCES lanes (id)
)";

const ARCHIVES: &'static str = "CREATE TABLE IF NOT EXISTS archives (
  id integer,
  uuid varchar(36) NOT NULL UNIQUE,
  lane_id integer,
  summary varchar NOT NULL,
  created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (id),
  FOREIGN KEY (lane_id) REFERENCES lanes (id)
)";

const ESTIMATES: &'static str = "CREATE TABLE IF NOT EXISTS estimates (
  id integer AUTO_INCREMENT,
  task_id integer,
  value integer NOT NULL,
  created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (id),
  FOREIGN KEY (task_id) REFERENCES tasks (id)
 )";

const POMODOROS: &'static str = "CREATE TABLE IF NOT EXISTS pomodoros (
  id integer AUTO_INCREMENT,
  task_id integer,
  started_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  finished_at timestamp,
  PRIMARY KEY (id),
  FOREIGN KEY (task_id) REFERENCES tasks (id)
)";

const INTERRUPTIONS: &'static str = "CREATE TABLE IF NOT EXISTS interruptions (
  id integer AUTO_INCREMENT,
  task_id integer,
  external boolean,
  created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (id),
  FOREIGN KEY (task_id) REFERENCES tasks (id)
);";

pub const STATEMENTS: [&str; 7] = [
    LANES,
    LANES_INSERT,
    TASKS,
    ARCHIVES,
    ESTIMATES,
    POMODOROS,
    INTERRUPTIONS,
];
