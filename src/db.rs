use rusqlite::{ params, Connection, Result };
use crate::Task;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("data/tasks.db")?;
        // Initializing db schema
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            status INTEGER NOT NULL DEFAULT 0
        )",
            []
        )?;
        Ok(Database { conn })
    }

    // Adding tasks to the db
    pub fn add_task(&self, task: &Task) -> Result<()> {
        self.conn.execute(
            "INSERT INTO tasks (name, status) VALUES (?1, ?2)",
            params![task.name, task.status]
        )?;
        Ok(())
    }

    // Get tasks from the db
    pub fn get_tasks(&self) -> Result<Vec<Task>> {
        let mut stmt = self.conn.prepare("SELECT name, status FROM tasks")?;
        let task_iter = stmt.query_map([], |row| {
            let name: String = row.get(0)?;
            let status: bool = row.get(1)?;
            Ok((name, status))
        })?;
        let mut task_collection = Vec::new();
        for task in task_iter {
            let (name, status) = task?;
            let task = Task { name, status };

            task_collection.push(task);
        }
        Ok(task_collection)
    }

    // Remove a task from db
    pub fn remove_task(&self, task_name: &str) -> Result<()> {
        self.conn.execute("DELETE FROM tasks WHERE name = ?1", params![task_name])?;
        Ok(())
    }

    // Mark a task as complete
    pub fn mark_complete(&self, task_name: &str) -> Result<()> {
        self.conn.execute("UPDATE tasks SET STATUS = 1 WHERE name = ?1", params![task_name])?;
        Ok(())
    }
}
