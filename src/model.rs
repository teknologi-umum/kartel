use rusqlite::Connection;

pub struct PointEntry {
    pub user_id: i64,
    pub user_name: String,
    pub points: i64,
}

pub struct Points {
    pub topic: String,
    pub entries: Vec<PointEntry>,
}

pub struct ModelError {
    pub message: String,
    pub query: String,
}

impl ModelError {
    pub fn new(message: String, query: String) -> Self {
        ModelError { message, query }
    }
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

        return match self.database.execute_batch(query.clone()) {
            Ok(..) => (),
            Err(e) => ModelError {
                message: e,
                query: query.into(),
            },
        };
    }

    pub fn put_point(
        &self,
        topic: String,
        user_id: i64,
        user_name: String,
        point: i64,
    ) -> Result<i64, ModelError> {
        let query = "INSERT INTO points \
            (topic, user_id, user_name, point, created_at, updated_at) \
            VALUES \
            (?, ?, ?, ?, date(), date()) \
            ON CONFLICT (topic, user_id) \
            DO UPDATE SET \
                point = point + ?, \
                updated_at = date()";

        return match self.database.execute::<String, i64, String, i64, i64>(
            query.clone(),
            [topic, user_id, user_name, point, point],
        ) {
            Ok(..) => (),
            Err(e) => ModelError {
                message: e,
                query: query.into(),
            },
        };
    }

    pub async fn get_points_by_topic(&self, topic: String) -> Result<Points, ModelError> {
        let query = "SELECT user_id, user_name, point FROM points WHERE topic = ?";

        let mut stmt = self.database.prepare(query.clone());
        let topics_iter = stmt.query_map([topic], |row| {
            Ok(PointEntry {
                user_id: row.get(0)?,
                user_name: row.get(1)?,
                points: row.get(2)?,
            })
        })?;

        let mut topics_vec: Vec<PointEntry> = Vec::new();
        for topic in topics_iter {
            topics_vec.push(topic);
        }

        Points {
            topic,
            entries: topics_vec,
        }
    }

    pub async fn get_topics(&self) -> Result<Vec<String>, ModelError> {
        let query = "SELECT topic FROM points";

        let mut stmt = self.database.prepare(query.clone());
        let topics_iter = stmt.query_map([], |row| row.get(0))?;

        let mut topics_vec: Vec<String> = Vec::new();
        for topic in topics_iter {
            topics_vec.push(topic);
        }

        topics_vec
    }
}
