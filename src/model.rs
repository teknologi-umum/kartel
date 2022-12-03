use rusqlite::{params, Connection};

#[derive(Debug)]
pub struct PointEntry {
    pub user_id: i64,
    pub user_name: String,
    pub points: i64,
}

#[derive(Debug)]
pub struct Points {
    pub topic: String,
    pub entries: Vec<PointEntry>,
}

#[derive(Debug)]
pub struct ModelError {
    pub message: String,
    pub query: String,
}

pub struct Model {
    database: Connection,
}

impl Model {
    pub fn new(database: Connection) -> Self {
        Model { database }
    }

    pub fn migrate(&self) -> Result<(), ModelError> {
        let query = "BEGIN;
            CREATE TABLE IF NOT EXISTS points (
                topic TEXT,
                user_id INTEGER,
                user_name TEXT,
                point INTEGER,
                created_at TEXT,
                updated_at TEXT
            );
            CREATE UNIQUE INDEX IF NOT EXISTS unq_idx_topic_userid ON points (topic, user_id);
            CREATE INDEX IF NOT EXISTS idx_topic ON points (topic);
            COMMIT;";

        if let Err(err) = self.database.execute_batch(query) {
            return Err(ModelError {
                message: err.to_string(),
                query: query.into(),
            });
        }

        Ok(())
    }

    pub fn put_point(
        &self,
        topic: String,
        user_id: i64,
        user_name: String,
        point: i64,
    ) -> Result<i64, ModelError> {
        let query = r#"INSERT INTO points 
            (topic, user_id, user_name, point, created_at, updated_at)
            VALUES
            (?, ?, ?, ?, date(), date())
            ON CONFLICT (topic, user_id)
            DO UPDATE SET
                point = point + ?,
                updated_at = date()
            RETURNING point"#;

        match self.database.query_row(
            query,
            params![topic, user_id, user_name, point, point],
            |row| row.get(0),
        ) {
            Ok(updated) => Ok(updated),
            Err(err) => Err(ModelError {
                message: err.to_string(),
                query: query.into(),
            }),
        }
    }

    pub async fn get_points_by_topic(&self, topic: String) -> Result<Points, ModelError> {
        let query = "SELECT user_id, user_name, point FROM points WHERE topic = ?";

        let mut stmt = match self.database.prepare(query) {
            Ok(stmt) => stmt,
            Err(err) => {
                return Err(ModelError {
                    message: err.to_string(),
                    query: query.into(),
                })
            }
        };

        let topics_iter = match stmt.query_map([&topic], |row| {
            Ok(PointEntry {
                user_id: row.get(0)?,
                user_name: row.get(1)?,
                points: row.get(2)?,
            })
        }) {
            Ok(topics) => topics,
            Err(err) => {
                return Err(ModelError {
                    message: err.to_string(),
                    query: query.into(),
                })
            }
        };

        let mut topics_vec: Vec<PointEntry> = Vec::new();
        for topic in topics_iter.flatten() {
            topics_vec.push(topic);
        }

        Ok(Points {
            topic,
            entries: topics_vec,
        })
    }

    pub async fn get_topics(&self) -> Result<Vec<String>, ModelError> {
        let query = "SELECT topic FROM points";

        let mut stmt = match self.database.prepare(&query) {
            Ok(stmt) => stmt,
            Err(err) => {
                return Err(ModelError {
                    message: err.to_string(),
                    query: query.into(),
                })
            }
        };

        let topics_iter = match stmt.query_map([], |row| row.get(0)) {
            Ok(topics) => topics,
            Err(err) => {
                return Err(ModelError {
                    message: err.to_string(),
                    query: query.into(),
                })
            }
        };

        let mut topics_vec: Vec<String> = Vec::new();
        for topic in topics_iter.flatten() {
            topics_vec.push(topic);
        }

        Ok(topics_vec)
    }
}

#[cfg(test)]
mod tests {
    use crate::model::Model;
    use rusqlite::Connection;

    #[test]
    fn test_migrate() {
        let database: Connection = Connection::open_in_memory().unwrap();
        let model = Model::new(database);

        model.migrate().unwrap();
    }

    #[test]
    fn test_put_new_point() {
        let database: Connection = Connection::open_in_memory().unwrap();

        let model = Model::new(database);
        model.migrate().unwrap();

        let res = model
            .put_point("dadjoke".into(), 1, "elianiva".into(), 1)
            .unwrap();

        assert_eq!(res, 1);

        let res2 = model
            .put_point("dadjoke".into(), 1, "elianiva".into(), 1)
            .unwrap();

        assert_eq!(res2, 2);
    }
}
