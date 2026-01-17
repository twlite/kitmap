use crate::stats::calculator::AllStats;
use crossterm::style::{Color, Stylize};
use std::collections::HashMap;

/// QWERTY keyboard layout for heatmap display
const KEYBOARD_LAYOUT: &[&[&str]] = &[
    &[
        "Escape", "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12",
    ],
    &[
        "`",
        "1",
        "2",
        "3",
        "4",
        "5",
        "6",
        "7",
        "8",
        "9",
        "0",
        "-",
        "=",
        "Backspace",
    ],
    &[
        "Tab", "q", "w", "e", "r", "t", "y", "u", "i", "o", "p", "[", "]", "\\",
    ],
    &[
        "CapsLock", "a", "s", "d", "f", "g", "h", "j", "k", "l", ";", "'", "Return",
    ],
    &[
        "ShiftLeft",
        "z",
        "x",
        "c",
        "v",
        "b",
        "n",
        "m",
        ",",
        ".",
        "/",
        "ShiftRight",
    ],
    &[
        "ControlLeft",
        "MetaLeft",
        "Alt",
        "Space",
        "AltGr",
        "MetaRight",
        "ControlRight",
    ],
];

/// Key display names mapping
fn get_display_name(key: &str) -> &str {
    match key {
        "Escape" => "ESC",
        "Backspace" => "⌫",
        "Tab" => "TAB",
        "CapsLock" => "CAPS",
        "Return" | "Enter" => "⏎",
        "ShiftLeft" | "ShiftRight" => "⇧",
        "ControlLeft" | "ControlRight" => "CTRL",
        "MetaLeft" | "MetaRight" => "⌘",
        "Alt" | "AltGr" => "ALT",
        "Space" => "SPACE",
        "UpArrow" => "↑",
        "DownArrow" => "↓",
        "LeftArrow" => "←",
        "RightArrow" => "→",
        _ => key,
    }
}

/// Get width for each key in display characters
fn get_key_width(key: &str) -> usize {
    match key {
        "Backspace" => 8,
        "Tab" => 5,
        "CapsLock" => 6,
        "Return" | "Enter" => 8,
        "ShiftLeft" => 8,
        "ShiftRight" => 10,
        "Space" => 30,
        "ControlLeft" | "ControlRight" => 6,
        "MetaLeft" | "MetaRight" => 5,
        "Alt" | "AltGr" => 5,
        "Escape" => 4,
        _ if key.starts_with('F') && key.len() <= 3 => 3,
        _ => 4,
    }
}

/// Get heat color based on intensity (0.0 to 1.0)
fn get_heat_color(intensity: f64) -> Color {
    if intensity == 0.0 {
        Color::DarkGrey
    } else if intensity < 0.1 {
        Color::Blue
    } else if intensity < 0.25 {
        Color::Cyan
    } else if intensity < 0.4 {
        Color::Green
    } else if intensity < 0.55 {
        Color::Yellow
    } else if intensity < 0.7 {
        Color::Rgb {
            r: 255,
            g: 165,
            b: 0,
        } // Orange
    } else if intensity < 0.85 {
        Color::Red
    } else {
        Color::Rgb {
            r: 255,
            g: 0,
            b: 255,
        } // Magenta/Hot
    }
}

/// Get heat character based on intensity
fn get_heat_char(intensity: f64) -> char {
    if intensity == 0.0 {
        '░'
    } else if intensity < 0.25 {
        '▒'
    } else if intensity < 0.5 {
        '▓'
    } else {
        '█'
    }
}

pub struct AsciiHeatmap {
    key_frequencies: HashMap<String, i64>,
    max_frequency: i64,
}

impl AsciiHeatmap {
    pub fn new(stats: &AllStats) -> Self {
        let max_frequency = stats.key_frequency_map.values().cloned().max().unwrap_or(1);
        Self {
            key_frequencies: stats.key_frequency_map.clone(),
            max_frequency,
        }
    }

    /// Get the intensity (0.0 to 1.0) for a key
    fn get_intensity(&self, key: &str) -> f64 {
        // Try exact match first
        if let Some(&count) = self.key_frequencies.get(key) {
            return count as f64 / self.max_frequency as f64;
        }

        // Try case-insensitive match
        let key_lower = key.to_lowercase();
        let key_upper = key.to_uppercase();

        for (k, &count) in &self.key_frequencies {
            if k.to_lowercase() == key_lower || k.to_uppercase() == key_upper || k == &key_upper {
                return count as f64 / self.max_frequency as f64;
            }
        }

        0.0
    }

    /// Get the count for a key
    fn get_count(&self, key: &str) -> i64 {
        if let Some(&count) = self.key_frequencies.get(key) {
            return count;
        }

        let key_lower = key.to_lowercase();
        let key_upper = key.to_uppercase();

        for (k, &count) in &self.key_frequencies {
            if k.to_lowercase() == key_lower || k.to_uppercase() == key_upper || k == &key_upper {
                return count;
            }
        }

        0
    }

    /// Render a single key with heat color
    fn render_key(&self, key: &str, width: usize) -> String {
        let intensity = self.get_intensity(key);
        let color = get_heat_color(intensity);
        let _heat_char = get_heat_char(intensity);
        let display = get_display_name(key);
        let count = self.get_count(key);

        // Create key display with padding
        let content = if count > 0 {
            format!("{}", display)
        } else {
            display.to_string()
        };

        let padded = format!("{:^width$}", content, width = width);

        // Apply color
        format!("{}", padded.with(color))
    }

    /// Render the full keyboard heatmap
    pub fn render(&self) -> String {
        let mut output = String::new();

        output.push_str("\n");
        output.push_str(
            "┌──────────────────────────────────────────────────────────────────────────────┐\n",
        );
        output.push_str(
            "│                          ⌨️  KEYBOARD HEATMAP                                │\n",
        );
        output.push_str(
            "├──────────────────────────────────────────────────────────────────────────────┤\n",
        );
        output.push_str(
            "│                                                                              │\n",
        );

        for row in KEYBOARD_LAYOUT {
            output.push_str("│  ");
            for key in *row {
                let width = get_key_width(key);
                output.push_str(&self.render_key(key, width));
                output.push(' ');
            }
            output.push_str("\n");
            output.push_str("│                                                                              │\n");
        }

        output.push_str(
            "├──────────────────────────────────────────────────────────────────────────────┤\n",
        );
        output.push_str("│  Heat Legend: ");
        output.push_str(&format!("{} ", "░ Cold".with(Color::DarkGrey)));
        output.push_str(&format!("{} ", "▒ Low".with(Color::Blue)));
        output.push_str(&format!("{} ", "▓ Med".with(Color::Green)));
        output.push_str(&format!("{} ", "█ High".with(Color::Yellow)));
        output.push_str(&format!("{}", "█ Hot".with(Color::Red)));
        output.push_str("                                  │\n");
        output.push_str(
            "└──────────────────────────────────────────────────────────────────────────────┘\n",
        );

        output
    }

    /// Render key statistics summary
    pub fn render_stats(&self, stats: &AllStats) -> String {
        let mut output = String::new();

        output.push_str(
            "\n┌──────────────────────────────────────────────────────────────────────────────┐\n",
        );
        output.push_str(
            "│                           📊 KEYBOARD STATISTICS                             │\n",
        );
        output.push_str(
            "├──────────────────────────────────────────────────────────────────────────────┤\n",
        );

        // General stats
        output.push_str(&format!(
            "│  Total Keys Pressed: {:>55} │\n",
            format!("{}", stats.total_keys).with(Color::Cyan)
        ));
        output.push_str(&format!(
            "│  Total Key Combos: {:>57} │\n",
            format!("{}", stats.total_combos).with(Color::Cyan)
        ));
        output.push_str(&format!(
            "│  Total Sessions: {:>59} │\n",
            format!("{}", stats.total_sessions).with(Color::Cyan)
        ));
        output.push_str(&format!(
            "│  Total Time (minutes): {:>53} │\n",
            format!("{:.1}", stats.total_time_minutes).with(Color::Cyan)
        ));
        output.push_str(&format!(
            "│  Unique Keys Used: {:>57} │\n",
            format!("{}", stats.unique_keys_used).with(Color::Cyan)
        ));

        output.push_str(
            "├──────────────────────────────────────────────────────────────────────────────┤\n",
        );

        // Most pressed key
        if let Some(ref key) = stats.most_pressed_key {
            output.push_str(&format!(
                "│  Most Pressed Key: {:>57} │\n",
                format!("{} ({}x, {:.1}%)", key.key_name, key.count, key.percentage)
                    .with(Color::Green)
            ));
        }

        // Most pressed combo
        if let Some(ref combo) = stats.most_pressed_combo {
            output.push_str(&format!(
                "│  Most Pressed Combo: {:>55} │\n",
                format!("{} ({}x)", combo.combo, combo.count).with(Color::Green)
            ));
        }

        output.push_str(
            "├──────────────────────────────────────────────────────────────────────────────┤\n",
        );

        // Special keys
        output.push_str(&format!(
            "│  Spacebar: {:>65} │\n",
            format!("{}", stats.spacebar_count).with(Color::Yellow)
        ));
        output.push_str(&format!(
            "│  Enter: {:>68} │\n",
            format!("{}", stats.enter_count).with(Color::Yellow)
        ));
        output.push_str(&format!(
            "│  Backspace: {:>64} │\n",
            format!("{}", stats.backspace_count).with(Color::Yellow)
        ));
        output.push_str(&format!(
            "│  Delete: {:>67} │\n",
            format!("{}", stats.delete_count).with(Color::Yellow)
        ));
        output.push_str(&format!(
            "│  Tab: {:>70} │\n",
            format!("{}", stats.tab_count).with(Color::Yellow)
        ));
        output.push_str(&format!(
            "│  Escape: {:>67} │\n",
            format!("{}", stats.escape_count).with(Color::Yellow)
        ));
        output.push_str(&format!(
            "│  Arrow Keys: {:>63} │\n",
            format!("{}", stats.arrow_keys_count).with(Color::Yellow)
        ));

        output.push_str(
            "├──────────────────────────────────────────────────────────────────────────────┤\n",
        );

        // Key categories
        output.push_str(&format!(
            "│  Letter Keys: {:>62} │\n",
            format!("{}", stats.letter_keys_count).with(Color::Magenta)
        ));
        output.push_str(&format!(
            "│  Number Keys: {:>62} │\n",
            format!("{}", stats.number_keys_count).with(Color::Magenta)
        ));
        output.push_str(&format!(
            "│  Modifier Keys: {:>60} │\n",
            format!("{}", stats.modifier_keys_count).with(Color::Magenta)
        ));
        output.push_str(&format!(
            "│  Special Keys: {:>61} │\n",
            format!("{}", stats.special_keys_count).with(Color::Magenta)
        ));

        output.push_str(
            "├──────────────────────────────────────────────────────────────────────────────┤\n",
        );

        // Typing speed
        output.push_str(&format!(
            "│  Avg Typing Speed (CPM): {:>51} │\n",
            format!("{:.1}", stats.average_typing_speed).with(Color::Cyan)
        ));
        output.push_str(&format!(
            "│  Max Typing Speed (CPM): {:>51} │\n",
            format!("{:.1}", stats.max_typing_speed).with(Color::Cyan)
        ));
        output.push_str(&format!(
            "│  Avg Keys Per Minute: {:>54} │\n",
            format!("{:.1}", stats.keys_per_minute_avg).with(Color::Cyan)
        ));
        output.push_str(&format!(
            "│  Avg Keys Per Session: {:>53} │\n",
            format!("{:.1}", stats.average_keys_per_session).with(Color::Cyan)
        ));

        output.push_str(
            "├──────────────────────────────────────────────────────────────────────────────┤\n",
        );

        // Most active times
        if let Some(ref hour) = stats.most_active_hour {
            output.push_str(&format!(
                "│  Most Active Hour: {:>57} │\n",
                format!("{}:00 ({} keys)", hour.hour, hour.count).with(Color::Green)
            ));
        }
        if let Some(ref day) = stats.most_active_day {
            output.push_str(&format!(
                "│  Most Active Day: {:>58} │\n",
                format!("{} ({} keys)", day.day, day.count).with(Color::Green)
            ));
        }

        output.push_str(
            "├──────────────────────────────────────────────────────────────────────────────┤\n",
        );

        // Time range
        if let Some(ref first) = stats.first_recorded {
            output.push_str(&format!(
                "│  First Recorded: {:>59} │\n",
                first[..19].to_string().with(Color::DarkGrey)
            ));
        }
        if let Some(ref last) = stats.last_recorded {
            output.push_str(&format!(
                "│  Last Recorded: {:>60} │\n",
                last[..19].to_string().with(Color::DarkGrey)
            ));
        }

        output.push_str(
            "└──────────────────────────────────────────────────────────────────────────────┘\n",
        );

        // Top keys
        output.push_str(
            "\n┌──────────────────────────────────────────────────────────────────────────────┐\n",
        );
        output.push_str(
            "│                              🔝 TOP 10 KEYS                                  │\n",
        );
        output.push_str(
            "├───────┬──────────────┬───────────────┬────────────────────────────────────────┤\n",
        );
        output.push_str(
            "│ Rank  │     Key      │     Count     │              Bar                       │\n",
        );
        output.push_str(
            "├───────┼──────────────┼───────────────┼────────────────────────────────────────┤\n",
        );

        let max_count = stats.top_keys.first().map(|k| k.count).unwrap_or(1);
        for (i, key) in stats.top_keys.iter().take(10).enumerate() {
            let bar_len = ((key.count as f64 / max_count as f64) * 35.0) as usize;
            let bar = "█".repeat(bar_len);
            let intensity = key.count as f64 / max_count as f64;
            let color = get_heat_color(intensity);

            output.push_str(&format!(
                "│  {:>2}.  │ {:^12} │ {:>13} │ {:<38} │\n",
                i + 1,
                get_display_name(&key.key_name),
                key.count,
                bar.with(color)
            ));
        }

        output.push_str(
            "└───────┴──────────────┴───────────────┴────────────────────────────────────────┘\n",
        );

        // Top combos
        if !stats.top_combos.is_empty() {
            output.push_str("\n┌──────────────────────────────────────────────────────────────────────────────┐\n");
            output.push_str("│                            ⌨️  TOP KEY COMBOS                                │\n");
            output.push_str("├───────┬──────────────────────────┬───────────────┬────────────────────────────┤\n");
            output.push_str("│ Rank  │         Combo            │     Count     │            Bar             │\n");
            output.push_str("├───────┼──────────────────────────┼───────────────┼────────────────────────────┤\n");

            let max_combo = stats.top_combos.first().map(|c| c.count).unwrap_or(1);
            for (i, combo) in stats.top_combos.iter().take(10).enumerate() {
                let bar_len = ((combo.count as f64 / max_combo as f64) * 25.0) as usize;
                let bar = "█".repeat(bar_len);
                let intensity = combo.count as f64 / max_combo as f64;
                let color = get_heat_color(intensity);

                output.push_str(&format!(
                    "│  {:>2}.  │ {:^24} │ {:>13} │ {:<26} │\n",
                    i + 1,
                    &combo.combo[..combo.combo.len().min(24)],
                    combo.count,
                    bar.with(color)
                ));
            }

            output.push_str("└───────┴──────────────────────────┴───────────────┴────────────────────────────┘\n");
        }

        // Hourly distribution
        output.push_str(
            "\n┌──────────────────────────────────────────────────────────────────────────────┐\n",
        );
        output.push_str(
            "│                          ⏰ HOURLY ACTIVITY                                  │\n",
        );
        output.push_str(
            "├──────────────────────────────────────────────────────────────────────────────┤\n",
        );

        let max_hourly = stats
            .hourly_distribution
            .iter()
            .map(|h| h.count)
            .max()
            .unwrap_or(1);
        for h in &stats.hourly_distribution {
            let bar_len = if max_hourly > 0 {
                ((h.count as f64 / max_hourly as f64) * 50.0) as usize
            } else {
                0
            };
            let bar = "█".repeat(bar_len);
            let intensity = if max_hourly > 0 {
                h.count as f64 / max_hourly as f64
            } else {
                0.0
            };
            let color = get_heat_color(intensity);

            output.push_str(&format!(
                "│  {:02}:00 │ {:>8} │ {:<50} │\n",
                h.hour,
                h.count,
                bar.with(color)
            ));
        }

        output.push_str(
            "└──────────────────────────────────────────────────────────────────────────────┘\n",
        );

        output
    }
}
