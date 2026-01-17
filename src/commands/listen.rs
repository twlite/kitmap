use crate::db::{
    init_db,
    models::{KeyCombo, KeyEvent, Session, TypingSample},
};
use anyhow::Result;
use crossterm::style::Stylize;
use rdev::{listen, Event, EventType, Key};
use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Modifier keys that can be part of key combinations
const MODIFIER_KEYS: &[Key] = &[
    Key::ShiftLeft,
    Key::ShiftRight,
    Key::ControlLeft,
    Key::ControlRight,
    Key::Alt,
    Key::AltGr,
    Key::MetaLeft,
    Key::MetaRight,
];

/// Check if a key is a modifier
fn is_modifier(key: &Key) -> bool {
    MODIFIER_KEYS.contains(key)
}

/// Get a human-readable name for a key
fn key_to_name(key: &Key) -> String {
    format!("{:?}", key)
}

/// Get a simplified key code
fn key_to_code(key: &Key) -> String {
    format!("{:?}", key)
}

struct ListenState {
    db: crate::db::DbConnection,
    session: Session,
    pressed_modifiers: HashSet<String>,
    last_key_time: Option<Instant>,
    keys_in_interval: u32,
    interval_start: Instant,
    total_keys: u64,
}

impl ListenState {
    fn new(db: crate::db::DbConnection) -> Self {
        Self {
            db,
            session: Session::new(),
            pressed_modifiers: HashSet::new(),
            last_key_time: None,
            keys_in_interval: 0,
            interval_start: Instant::now(),
            total_keys: 0,
        }
    }

    fn record_key_event(&mut self, key: Key) {
        let key_name = key_to_name(&key);
        let key_code = key_to_code(&key);
        let is_mod = is_modifier(&key);

        // Record the key event
        let event = KeyEvent::new(key_code, key_name.clone(), is_mod);
        if let Err(e) = event.save(&self.db) {
            eprintln!("Failed to save key event: {}", e);
        }

        // If this is a non-modifier key and there are modifiers held, record a combo
        if !is_mod && !self.pressed_modifiers.is_empty() {
            let mut mods: Vec<_> = self.pressed_modifiers.iter().cloned().collect();
            mods.sort();
            mods.push(key_name.clone());
            let combo_str = mods.join("+");

            let combo = KeyCombo::new(combo_str);
            if let Err(e) = combo.save(&self.db) {
                eprintln!("Failed to save key combo: {}", e);
            }
        }

        // Track typing speed
        self.keys_in_interval += 1;
        self.total_keys += 1;
        self.session.increment_keys();

        // Calculate typing speed every 10 seconds
        let elapsed = self.interval_start.elapsed();
        if elapsed >= Duration::from_secs(10) {
            let chars_per_minute = (self.keys_in_interval as f64 / elapsed.as_secs_f64()) * 60.0;
            let sample = TypingSample::new(chars_per_minute);
            if let Err(e) = sample.save(&self.db) {
                eprintln!("Failed to save typing sample: {}", e);
            }

            self.keys_in_interval = 0;
            self.interval_start = Instant::now();
        }

        self.last_key_time = Some(Instant::now());
    }

    fn modifier_pressed(&mut self, key: Key) {
        self.pressed_modifiers.insert(key_to_name(&key));
    }

    fn modifier_released(&mut self, key: Key) {
        self.pressed_modifiers.remove(&key_to_name(&key));
    }
}

pub async fn run() -> Result<()> {
    println!("{}", "🎹 KitMap - Keyboard Activity Tracker".cyan().bold());
    println!("{}", "━".repeat(40).dark_grey());
    println!();
    println!("{} Initializing database...", "→".dark_grey());

    let db = init_db()?;

    println!("{} Database ready!", "✓".green());
    println!();
    println!("{}", "Starting keyboard listener...".yellow());
    println!("{}", "Press Ctrl+C to stop recording.".dark_grey());
    println!();

    let state = Arc::new(Mutex::new(ListenState::new(db.clone())));

    // Start session
    {
        let mut s = state.lock().unwrap();
        s.session.start(&db)?;
    }

    // Set up Ctrl+C handler with atomic flag
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let state_clone = state.clone();
    let db_clone = db.clone();

    ctrlc::set_handler(move || {
        println!();
        println!("{}", "Stopping listener...".yellow());

        // End session
        {
            let mut s = state_clone.lock().unwrap();
            if let Err(e) = s.session.end(&db_clone) {
                eprintln!("Failed to end session: {}", e);
            }

            println!();
            println!("{}", "━".repeat(40).dark_grey());
            println!("{} Session ended!", "✓".green());
            println!(
                "   Total keys recorded: {}",
                s.total_keys.to_string().cyan()
            );
            println!();
        }

        r.store(false, Ordering::SeqCst);
        std::process::exit(0);
    })
    .expect("Failed to set Ctrl+C handler");

    // Start listening
    let state_for_callback = state.clone();

    let callback = move |event: Event| {
        match event.event_type {
            EventType::KeyPress(key) => {
                let mut s = state_for_callback.lock().unwrap();

                if is_modifier(&key) {
                    s.modifier_pressed(key);
                }

                s.record_key_event(key);

                // Print feedback
                let key_name = key_to_name(&key);
                print!(
                    "\r{} {} recorded (total: {})",
                    "⌨".cyan(),
                    key_name.green(),
                    s.total_keys.to_string().yellow()
                );
                print!("                    "); // Clear any remaining chars
                use std::io::Write;
                let _ = std::io::stdout().flush();
            }
            EventType::KeyRelease(key) => {
                if is_modifier(&key) {
                    let mut s = state_for_callback.lock().unwrap();
                    s.modifier_released(key);
                }
            }
            _ => {}
        }
    };

    // This blocks until the program is terminated
    if let Err(error) = listen(callback) {
        eprintln!("Error: {:?}", error);

        // End session on error
        let mut s = state.lock().unwrap();
        s.session.end(&db)?;
    }

    Ok(())
}
