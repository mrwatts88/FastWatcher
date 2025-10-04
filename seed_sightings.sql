-- ==========================
-- FastWatcher Sightings Seed Data
-- ==========================
-- Trip 1: Ozark Trail Exploration
INSERT
    OR IGNORE INTO sightings (
        trip_id,
        taxon_id,
        kingdom,
        phylum,
        class,
        "order",
        family,
        genus,
        species_epithet,
        common_name,
        notes,
        media_path,
        date,
        location
    )
VALUES
    (
        1,
        1,
        'Animalia',
        'Chordata',
        'Aves',
        'Passeriformes',
        'Corvidae',
        'Cyanocitta',
        'cristata',
        'Blue Jay',
        'Pair of blue jays near trailhead, very vocal.',
        'photos/ozark_bluejay1.jpg',
        '2025-10-04',
        'Trailhead Oak Grove'
    ),
    (
        1,
        2,
        'Animalia',
        'Chordata',
        'Aves',
        'Accipitriformes',
        'Accipitridae',
        'Buteo',
        'jamaicensis',
        'Red-tailed Hawk',
        'Soaring above the valley around midday.',
        'photos/ozark_hawk1.jpg',
        '2025-10-04',
        'Overlook Ridge'
    ),
    (
        1,
        1,
        'Animalia',
        'Chordata',
        'Aves',
        'Passeriformes',
        'Corvidae',
        'Cyanocitta',
        'cristata',
        'Blue Jay',
        'Single jay feeding on acorns.',
        'photos/ozark_bluejay2.jpg',
        '2025-10-04',
        'Creek Bend'
    );

-- Trip 2: Lake Michigan Shoreline Walk
INSERT
    OR IGNORE INTO sightings (
        trip_id,
        taxon_id,
        kingdom,
        phylum,
        class,
        "order",
        family,
        genus,
        species_epithet,
        common_name,
        notes,
        media_path,
        date,
        location
    )
VALUES
    (
        2,
        2,
        'Animalia',
        'Chordata',
        'Aves',
        'Accipitriformes',
        'Accipitridae',
        'Buteo',
        'jamaicensis',
        'Red-tailed Hawk',
        'Circling above the dunes — bright red tail visible.',
        'photos/lake_hawk1.jpg',
        '2025-08-14',
        'North Dunes'
    ),
    (
        2,
        1,
        'Animalia',
        'Chordata',
        'Aves',
        'Passeriformes',
        'Corvidae',
        'Cyanocitta',
        'cristata',
        'Blue Jay',
        'Three jays in shoreline pine grove.',
        'photos/lake_bluejay1.jpg',
        '2025-08-14',
        'Pine Grove'
    ),
    (
        2,
        3,
        'Animalia',
        'Chordata',
        'Mammalia',
        'Primates',
        'Hominidae',
        'Homo',
        'sapiens',
        'Human',
        'Observer taking notes at the shoreline.',
        NULL,
        '2025-08-14',
        'Observation Point'
    );

-- Trip 3: Shawnee National Forest
INSERT
    OR IGNORE INTO sightings (
        trip_id,
        taxon_id,
        kingdom,
        phylum,
        class,
        "order",
        family,
        genus,
        species_epithet,
        common_name,
        notes,
        media_path,
        date,
        location
    )
VALUES
    (
        3,
        2,
        'Animalia',
        'Chordata',
        'Aves',
        'Accipitriformes',
        'Accipitridae',
        'Buteo',
        'jamaicensis',
        'Red-tailed Hawk',
        'Perched silently on tall oak — possibly juvenile.',
        'photos/shawnee_hawk1.jpg',
        '2025-05-22',
        'South Ridge'
    ),
    (
        3,
        1,
        'Animalia',
        'Chordata',
        'Aves',
        'Passeriformes',
        'Corvidae',
        'Cyanocitta',
        'cristata',
        'Blue Jay',
        'Blue jay mobbing a hawk near clearing.',
        'photos/shawnee_bluejay1.jpg',
        '2025-05-22',
        'Forest Edge'
    ),
    (
        3,
        3,
        'Animalia',
        'Chordata',
        'Mammalia',
        'Primates',
        'Hominidae',
        'Homo',
        'sapiens',
        'Human',
        'Researcher documenting local bird species.',
        NULL,
        '2025-05-22',
        'Campsite HQ'
    );