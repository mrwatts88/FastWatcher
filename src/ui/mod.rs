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
                fetch_sighting_detail(&ui, id);
                ui.set_current_view("sighting-detail".into());
            }
        }
    });

    ui.on_view_taxon_detail({
        let ui_weak = ui.as_weak();
        move |id| {
            if let Some(ui) = ui_weak.upgrade() {
                fetch_taxon_detail(&ui, id);
                ui.set_current_view("taxon-detail".into());
            }
        }
    });

    ui.on_view_trip_detail({
        let ui_weak = ui.as_weak();
        move |id| {
            if let Some(ui) = ui_weak.upgrade() {
                fetch_trip_detail(&ui, id);
                ui.set_current_view("trip-detail".into());
            }
        }
    });

    // Related entity navigation callbacks
    ui.on_view_related_sighting({
        let ui_weak = ui.as_weak();
        move |id| {
            if let Some(ui) = ui_weak.upgrade() {
                fetch_sighting_detail(&ui, id);
                ui.set_current_view("sighting-detail".into());
            }
        }
    });

    ui.on_view_related_taxon({
        let ui_weak = ui.as_weak();
        move |id| {
            if let Some(ui) = ui_weak.upgrade() {
                fetch_taxon_detail(&ui, id);
                ui.set_current_view("taxon-detail".into());
            }
        }
    });

    ui.on_view_related_trip({
        let ui_weak = ui.as_weak();
        move |id| {
            if let Some(ui) = ui_weak.upgrade() {
                fetch_trip_detail(&ui, id);
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

fn fetch_sighting_detail(ui: &AppWindow, id: i32) {
    use crate::core::db::connect;
    use crate::core::sighting::get_sighting_by_id;
    use crate::core::taxon::get_taxon_by_id;
    use crate::core::trip::get_trip_by_id;

    let conn = match connect() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to connect to database: {}", e);
            return;
        }
    };

    // Get the sighting
    let sighting = match get_sighting_by_id(&conn, id as i64) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to fetch sighting: {}", e);
            return;
        }
    };

    // Build taxonomy string
    let mut tax_parts = vec![sighting.kingdom.clone()];
    if let Some(ref p) = sighting.phylum { tax_parts.push(p.clone()); }
    if let Some(ref c) = sighting.class { tax_parts.push(c.clone()); }
    if let Some(ref o) = sighting.order { tax_parts.push(o.clone()); }
    if let Some(ref f) = sighting.family { tax_parts.push(f.clone()); }
    if let Some(ref g) = sighting.genus { tax_parts.push(g.clone()); }
    if let Some(ref s) = sighting.species_epithet { tax_parts.push(s.clone()); }

    // Create SightingDetail struct
    let detail = SightingDetail {
        id: sighting.id as i32,
        common_name: SharedString::from(sighting.common_name.clone()),
        taxonomy: SharedString::from(tax_parts.join(" / ")),
        date: SharedString::from(sighting.date.unwrap_or_default()),
        location: SharedString::from(sighting.location.unwrap_or_default()),
        notes: SharedString::from(sighting.notes.unwrap_or_default()),
        media_path: SharedString::from(sighting.media_path.unwrap_or_default()),
        taxon_id: sighting.taxon_id as i32,
        trip_id: sighting.trip_id.map(|t| t as i32).unwrap_or(0),
        has_trip: sighting.trip_id.is_some(),
    };

    ui.set_current_sighting(detail);

    // Get related taxon (always exists)
    let related_taxa = if let Ok(taxon) = get_taxon_by_id(&conn, sighting.taxon_id) {
        vec![RelatedTaxonItem {
            id: taxon.id as i32,
            common_name: SharedString::from(taxon.common_name),
            rank: SharedString::from(taxon.rank),
        }]
    } else {
        vec![]
    };
    ui.set_related_taxa(ModelRc::new(VecModel::from(related_taxa)));

    // Get related trip (if exists)
    let related_trips = if let Some(trip_id) = sighting.trip_id {
        if let Ok(trip) = get_trip_by_id(&conn, trip_id) {
            vec![RelatedTripItem {
                id: trip.id as i32,
                name: SharedString::from(trip.name),
                date: SharedString::from(trip.date.unwrap_or_default()),
            }]
        } else {
            vec![]
        }
    } else {
        vec![]
    };
    ui.set_related_trips(ModelRc::new(VecModel::from(related_trips)));
}

fn fetch_taxon_detail(ui: &AppWindow, id: i32) {
    use crate::core::db::connect;
    use crate::core::taxon::get_taxon_by_id;
    use crate::core::sighting::get_sightings_by_taxon;
    use crate::core::trip::get_trips_by_taxon;

    let conn = match connect() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to connect to database: {}", e);
            return;
        }
    };

    // Get the taxon
    let taxon = match get_taxon_by_id(&conn, id as i64) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to fetch taxon: {}", e);
            return;
        }
    };

    // Create TaxonDetail struct
    let detail = TaxonDetail {
        id: taxon.id as i32,
        rank: SharedString::from(taxon.rank.clone()),
        common_name: SharedString::from(taxon.common_name.clone()),
        kingdom: SharedString::from(taxon.kingdom.clone()),
        phylum: SharedString::from(taxon.phylum.clone().unwrap_or_default()),
        class: SharedString::from(taxon.class.clone().unwrap_or_default()),
        order: SharedString::from(taxon.order.clone().unwrap_or_default()),
        family: SharedString::from(taxon.family.clone().unwrap_or_default()),
        subfamily: SharedString::from(taxon.subfamily.clone().unwrap_or_default()),
        genus: SharedString::from(taxon.genus.clone().unwrap_or_default()),
        species_epithet: SharedString::from(taxon.species_epithet.clone().unwrap_or_default()),
    };

    ui.set_current_taxon(detail);

    // Get related sightings (using hierarchy-based search)
    let sightings = get_sightings_by_taxon(&conn, &taxon).unwrap_or_default();
    let related_sightings: Vec<RelatedSightingItem> = sightings
        .iter()
        .map(|s| RelatedSightingItem {
            id: s.id as i32,
            common_name: SharedString::from(s.common_name.clone()),
            date: SharedString::from(s.date.as_ref().unwrap_or(&String::new()).clone()),
        })
        .collect();
    ui.set_related_sightings(ModelRc::new(VecModel::from(related_sightings)));

    // Get related trips (using hierarchy-based search)
    let trips = get_trips_by_taxon(&conn, &taxon).unwrap_or_default();
    let related_trips: Vec<RelatedTripItem> = trips
        .iter()
        .map(|t| RelatedTripItem {
            id: t.id as i32,
            name: SharedString::from(t.name.clone()),
            date: SharedString::from(t.date.as_ref().unwrap_or(&String::new()).clone()),
        })
        .collect();
    ui.set_related_trips(ModelRc::new(VecModel::from(related_trips)));
}

fn fetch_trip_detail(ui: &AppWindow, id: i32) {
    use crate::core::db::connect;
    use crate::core::trip::get_trip_by_id;
    use crate::core::sighting::get_sightings_by_trip_id;
    use crate::core::taxon::get_taxon_by_id;
    use std::collections::HashMap;

    let conn = match connect() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to connect to database: {}", e);
            return;
        }
    };

    // Get the trip
    let trip = match get_trip_by_id(&conn, id as i64) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to fetch trip: {}", e);
            return;
        }
    };

    // Create TripDetail struct
    let detail = TripDetail {
        id: trip.id as i32,
        name: SharedString::from(trip.name.clone()),
        date: SharedString::from(trip.date.unwrap_or_default()),
        location: SharedString::from(trip.location.unwrap_or_default()),
        notes: SharedString::from(trip.notes.unwrap_or_default()),
    };

    ui.set_current_trip(detail);

    // Get related sightings
    let sightings = get_sightings_by_trip_id(&conn, id as i64).unwrap_or_default();
    let related_sightings: Vec<RelatedSightingItem> = sightings
        .iter()
        .map(|s| RelatedSightingItem {
            id: s.id as i32,
            common_name: SharedString::from(s.common_name.clone()),
            date: SharedString::from(s.date.as_ref().unwrap_or(&String::new()).clone()),
        })
        .collect();
    ui.set_related_sightings(ModelRc::new(VecModel::from(related_sightings)));

    // Build distinct taxa list from sightings
    let mut taxa_map: HashMap<i64, RelatedTaxonItem> = HashMap::new();
    for sighting in &sightings {
        // Only add each taxon_id once
        if !taxa_map.contains_key(&sighting.taxon_id) {
            // Fetch the taxon to get rank info
            if let Ok(taxon) = get_taxon_by_id(&conn, sighting.taxon_id) {
                taxa_map.insert(
                    sighting.taxon_id,
                    RelatedTaxonItem {
                        id: taxon.id as i32,
                        common_name: SharedString::from(taxon.common_name),
                        rank: SharedString::from(taxon.rank),
                    }
                );
            }
        }
    }

    let mut related_taxa: Vec<RelatedTaxonItem> = taxa_map.into_values().collect();
    // Sort by common name for consistent display
    related_taxa.sort_by(|a, b| a.common_name.to_string().cmp(&b.common_name.to_string()));

    ui.set_related_taxa(ModelRc::new(VecModel::from(related_taxa)));

    // Clear related trips (trips don't have related trips)
    ui.set_related_trips(ModelRc::new(VecModel::from(vec![])));
}
