use time::Duration;
use diesel::{SqliteConnection, QueryResult};
use diesel::connection::SimpleConnection;

#[derive(Debug)]
pub struct ConnectionOptions {
    pub busy_timeout: Option<Duration>,
}

impl ConnectionOptions {
    pub fn apply(&self, conn: &SqliteConnection) -> QueryResult<()> {
        conn.batch_execute(
            &"PRAGMA foreign_keys = ON;".to_string()
        )?;
        match self.busy_timeout {
            None => {},
            Some(t) => {
                conn.batch_execute(
                    &format!("PRAGMA busy_timeout = {};", t.whole_milliseconds())
                )?;
            },
        }
        Ok(())
    }
}

impl Default for ConnectionOptions {
    fn default() -> Self {
        Self {
            busy_timeout: Some(Duration::seconds(1))
        }
    }
}

impl diesel::r2d2::CustomizeConnection<SqliteConnection, diesel::r2d2::Error> for ConnectionOptions {
    fn on_acquire(&self, conn: &mut SqliteConnection) -> Result<(), diesel::r2d2::Error> {
        self.apply(conn).map_err(diesel::r2d2::Error::QueryError)
    }
}