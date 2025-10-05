use slint::Timer;
use std::time::Duration;
use std::cell::RefCell;
use std::rc::Rc;

slint::include_modules!();

pub fn run_ui() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    // Debounce timer setup
    let debounce_timer = Rc::new(RefCell::new(None::<Timer>));
    let debounce_delay = Duration::from_millis(0);

    // Handle search text changes with debounce
    ui.on_search_changed({
        let ui_weak = ui.as_weak();
        let timer_ref = debounce_timer.clone();

        move |text| {
            let text = text.to_string();

            // Cancel existing timer
            if let Some(timer) = timer_ref.borrow_mut().take() {
                timer.stop();
            }

            // Only search if 3+ characters
            if text.len() < 3 {
                // Clear results if less than 3 chars
                if let Some(ui) = ui_weak.upgrade() {
                    ui.set_sightings_count(0);
                    ui.set_taxa_count(0);
                    ui.set_trips_count(0);
                }
                return;
            }

            // Create new debounced timer
            let ui_weak_clone = ui_weak.clone();
            let new_timer = Timer::default();
            new_timer.start(slint::TimerMode::SingleShot, debounce_delay, move || {
                if let Some(ui) = ui_weak_clone.upgrade() {
                    // Perform search
                    perform_search(&ui, &text);
                }
            });
            *timer_ref.borrow_mut() = Some(new_timer);
        }
    });

    ui.run()
}

fn perform_search(ui: &AppWindow, query: &str) {
    use crate::core::db::connect;
    use crate::core::search::*;

    let conn = match connect() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to connect to database: {}", e);
            return;
        }
    };

    // Run all 3 searches
    let sightings = run_search_sightings(&conn, query).unwrap_or_default();
    let taxa = run_search_taxa(&conn, query).unwrap_or_default();
    let trips = run_search_trips(&conn, query).unwrap_or_default();

    // Update counts
    ui.set_sightings_count(sightings.len() as i32);
    ui.set_taxa_count(taxa.len() as i32);
    ui.set_trips_count(trips.len() as i32);

    // TODO: Convert to Slint models and display actual results
}
