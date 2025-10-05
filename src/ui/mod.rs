use slint::{Timer, VecModel, ModelRc, SharedString};
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
                    ui.set_sightings_model(ModelRc::new(VecModel::from(vec![])));
                    ui.set_taxa_model(ModelRc::new(VecModel::from(vec![])));
                    ui.set_trips_model(ModelRc::new(VecModel::from(vec![])));
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

    // Navigation callbacks
    ui.on_view_sighting_detail({
        let ui_weak = ui.as_weak();
        move |id| {
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_detail_entity_id(id);
                ui.set_current_view("sighting-detail".into());
            }
        }
    });

    ui.on_view_taxon_detail({
        let ui_weak = ui.as_weak();
        move |id| {
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_detail_entity_id(id);
                ui.set_current_view("taxon-detail".into());
            }
        }
    });

    ui.on_view_trip_detail({
        let ui_weak = ui.as_weak();
        move |id| {
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_detail_entity_id(id);
                ui.set_current_view("trip-detail".into());
            }
        }
    });

    ui.on_back_to_search({
        let ui_weak = ui.as_weak();
        move || {
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_current_view("search".into());
            }
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

    // Convert sightings to Slint items
    let sighting_items: Vec<SightingItem> = sightings
        .iter()
        .map(|s| SightingItem {
            id: s.id as i32,
            common_name: SharedString::from(s.common_name.clone()),
            date: SharedString::from(s.date.as_ref().unwrap_or(&String::new()).clone()),
            location: SharedString::from(s.location.as_ref().unwrap_or(&String::new()).clone()),
        })
        .collect();

    // Convert taxa to Slint items
    let taxon_items: Vec<TaxonItem> = taxa
        .iter()
        .map(|t| {
            // Build taxonomy string
            let mut parts = vec![t.kingdom.clone()];
            if let Some(ref p) = t.phylum { parts.push(p.clone()); }
            if let Some(ref c) = t.class { parts.push(c.clone()); }
            if let Some(ref o) = t.order { parts.push(o.clone()); }
            if let Some(ref f) = t.family { parts.push(f.clone()); }
            if let Some(ref g) = t.genus { parts.push(g.clone()); }
            if let Some(ref s) = t.species_epithet { parts.push(s.clone()); }

            TaxonItem {
                id: t.id as i32,
                rank: SharedString::from(t.rank.clone()),
                common_name: SharedString::from(t.common_name.clone()),
                taxonomy: SharedString::from(parts.join(" / ")),
            }
        })
        .collect();

    // Convert trips to Slint items
    let trip_items: Vec<TripItem> = trips
        .iter()
        .map(|t| TripItem {
            id: t.id as i32,
            name: SharedString::from(t.name.clone()),
            date: SharedString::from(t.date.as_ref().unwrap_or(&String::new()).clone()),
            location: SharedString::from(t.location.as_ref().unwrap_or(&String::new()).clone()),
        })
        .collect();

    // Set models on UI
    ui.set_sightings_model(ModelRc::new(VecModel::from(sighting_items)));
    ui.set_taxa_model(ModelRc::new(VecModel::from(taxon_items)));
    ui.set_trips_model(ModelRc::new(VecModel::from(trip_items)));
}
