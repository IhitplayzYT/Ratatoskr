pub mod Database{
    use mysql::{Pool, PooledConn, TxOpts, prelude::Queryable};
    use crate::model::journal::Journal::Journal_task;
pub struct Database {
    pool: Pool,
}

impl Database {
    pub fn new(url: &str) -> mysql::Result<Self> {
        Ok(Self {
            pool: Pool::new(url)?,
        })
    }

    fn conn(&self) -> mysql::Result<PooledConn> {
        self.pool.get_conn()
    }
pub fn save(&self,task: &Journal_task) -> mysql::Result<()>{
    let mut conn = self.conn()?;
    let mut tx = conn.start_transaction(TxOpts::default())?;

    Ok(())
}


}


}