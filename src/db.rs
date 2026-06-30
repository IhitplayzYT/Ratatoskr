pub mod Database{
use std::collections::HashSet;

use chrono::{DateTime, NaiveDateTime, Utc};
use mysql::{
    params,
    prelude::*,
    Pool,
    PooledConn,
    TxOpts,
};

use uuid::Uuid;

    use crate::model::journal::Journal::Journal_task;
    use crate::model::meta::Meta::{Mood, MyColor, Tag};
pub struct Database {
    pool: Pool,
}

impl Database {

 //-------------------------------------------------Init APIs----------------------------------------------------
    pub fn new(url: &str) -> mysql::Result<Self> {
        Ok(Self {
            pool: Pool::new(url)?,
        })
    }

    fn conn(&self) -> mysql::Result<PooledConn> {
        self.pool.get_conn()
    }

 //-----------------------------------------------------------------------------------------------------


 //-------------------------------------------------Journal APIs----------------------------------------------------

pub fn save_journal_task(&self,task: &Journal_task) -> mysql::Result<()>{
    let mut conn = self.conn()?;
    let mut tx = conn.start_transaction(TxOpts::default())?;
   tx.exec_drop(
            r"
            INSERT INTO Journal_tasks(
                id,
                title,
                content,
                mood,
                created_at,
                updated_at,
                member,
                topic
            )
            VALUES(
                :id,
                :title,
                :content,
                :mood,
                :created,
                :updated,
                :member,
                :topic
            )
            ",
            params! {
                "id" => task.id.to_string(),
                "title" => &task.title,
                "content" => &task.content,
                "mood" => task.mood.as_ref().map(|m| format!("{:?}",m)),
                "created" => task.created_at.naive_utc(),
                "updated" => task.updated_at.naive_utc(),
                "member" => &task.member,
                "topic" => &task.topic, 
            },
        )?;

        for tag in &task.tags {
            self.add_tag(&mut tx, tag)?;
            let tag_id: u64 = tx.exec_first(
                "SELECT id FROM tags WHERE name=?",
                (&tag.name,tag.color.rgb_str()),
            )?
            .unwrap();

            tx.exec_drop(
                r"
                INSERT IGNORE INTO journal_task_tags(
                    task_id,
                    tag_id
                )
                VALUES(
                    ?,
                    ?
                )
                ",
                (
                    task.id.to_string(),
                    tag_id,
                ),
            )?;
        }
        tx.commit()?;
    Ok(())
}

fn load_journal_task(&self,jt_id:Uuid) -> mysql::Result<Option<Journal_task>>{
    let mut conn = self.conn()?;
    let mut tx = conn.start_transaction(TxOpts::default())?;
    
    let mut ret = tx.exec_map(r"
    SELECT title,content,mood,created_at,updated_at,member,topic FROM Journal_tasks jt WHERE jt.id = ?;
    ",(jt_id,),|(t,c,m,ca,ua,me,to):(String,String,String,NaiveDateTime,NaiveDateTime,String,String)|{
        let mut k = Journal_task::new(Some(t), Some(c), Some(Mood::from(m)), None, Some(me), Some(to));
        k.created_at = DateTime::<Utc>::from_naive_utc_and_offset(ca, Utc);
        k.updated_at = DateTime::<Utc>::from_naive_utc_and_offset(ua, Utc);
        k
    })?;
    if ret.is_empty(){
        tx.commit()?;
        return Ok(None);
    } 

    if ret.len() != 1 {
        tx.rollback()?;
        eprintln!("Not Possible all Journal Ids have to be UNIQUE!");
        std::process::exit(2);
    }

    ret[0].tags = self.load_tags(&mut tx, "Journal",jt_id)?.into_iter().collect::<HashSet<Tag>>();
tx.commit()?;
Ok(Some(ret[0].clone()))
}

fn load_all_journal_task(&self) -> mysql::Result<Vec<Journal_task>>{
    let mut conn = self.conn()?;
    let mut tx = conn.start_transaction(TxOpts::default())?;
    
    let mut ret = tx.exec_map(r"
    SELECT id,title,content,mood,created_at,updated_at,member,topic FROM Journal_tasks;
    ",(),|(id,t,c,m,ca,ua,me,to):(String,String,String,String,NaiveDateTime,NaiveDateTime,String,String)|{
        let mut k = Journal_task::new(Some(t), Some(c), Some(Mood::from(m)), None, Some(me), Some(to));
        k.id = Uuid::parse_str(&id).unwrap();
        k.created_at = DateTime::<Utc>::from_naive_utc_and_offset(ca, Utc);
        k.updated_at = DateTime::<Utc>::from_naive_utc_and_offset(ua, Utc);
        k
    })?;
    for k in &mut ret{
        k.tags = self.load_tags(&mut tx,"Journal",k.id).unwrap().into_iter().collect::<HashSet<Tag>>();
    }


tx.commit()?;
Ok(ret)
}

 //-----------------------------------------------------------------------------------------------------

 //-------------------------------------------------Generic APIs----------------------------------------------------
fn delete_task(&self,table:&str,id:Uuid) -> mysql::Result<()>{
let mut conn = self.conn()?;
let mut tx = conn.start_transaction(TxOpts::default())?; 
let stmt = format!("DELETE FROM {}_task WHERE id = ?",table);
tx.exec_drop(&stmt,(id,))?;
Ok(())
} 


fn load_tags(&self,conn: &mut impl Queryable,table:&str,id: Uuid) -> mysql::Result<Vec<Tag>> {
    let stmt = format!(r"
        SELECT t.name.t.color
        FROM tags t
        INNER JOIN {}_task_tags tt
            ON t.id = tt.tag_id
        WHERE jtt.task_id = ?
        ",table
 );
    conn.exec_map(stmt,
        (id.to_string(),),
        |(name,color):(String,String)| {
            let r = u8::from_str_radix(&color[0..2], 16).unwrap();
            let g = u8::from_str_radix(&color[2..4], 16).unwrap();
            let b = u8::from_str_radix(&color[4..6], 16).unwrap();
            Tag::new(name, Some(MyColor::RGB(r, g, b)))},
    )
}

fn add_tag(&self,conn:&mut impl Queryable,tag: &Tag) -> mysql::Result<()>{
    conn.exec_drop(r"
    INSERT INTO tags(name,color) VALUES(?.?);
    ", (&tag.name,tag.color.rgb_str(),))?;
    Ok(())
}

//-----------------------------------------------------------------------------------------------------









}


}