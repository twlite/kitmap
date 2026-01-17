use crate::db::get_db_path;
use anyhow::Result;

pub async fn run() -> Result<()> {
    let db_path = get_db_path()?;
    println!("Database path: {}", db_path.display());
    Ok(())
}
