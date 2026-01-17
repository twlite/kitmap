use crate::db::{init_db, schema};
use anyhow::Result;
use crossterm::style::Stylize;
use std::io::{self, Write};

pub async fn run(force: bool) -> Result<()> {
    println!("{}", "🗑️  KitMap - Reset Data".cyan().bold());
    println!("{}", "━".repeat(40).dark_grey());
    println!();

    if !force {
        println!(
            "{}",
            "⚠️  Warning: This will delete ALL recorded keyboard data!".yellow()
        );
        println!();
        print!("Are you sure you want to continue? [y/N]: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim().to_lowercase();
        if input != "y" && input != "yes" {
            println!();
            println!("{}", "Operation cancelled.".dark_grey());
            return Ok(());
        }
    }

    println!();
    println!("{} Clearing database...", "→".dark_grey());

    let db = init_db()?;
    {
        let conn = db.lock().unwrap();
        schema::clear_all_data(&conn)?;
    }

    println!("{} All keyboard data has been cleared!", "✓".green());
    println!();

    Ok(())
}
