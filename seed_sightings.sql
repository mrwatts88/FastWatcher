-- Generated sample sightings from NACC bird data
-- Uses birds from first 100 species for test compatibility
-- DO NOT EDIT MANUALLY - regenerate using scripts/generate_bird_seeds.py

-- Sightings with trips
INSERT OR IGNORE INTO sightings (
    trip_id, taxon_id, kingdom, phylum, class, "order", family, subfamily, genus, species_epithet, common_name,
    notes, media_path, date, location
) VALUES (
    1,
    (SELECT id FROM taxa WHERE rank='species' AND genus='Nothocercus' AND species_epithet='bonapartei'),
    'Animalia', 'Chordata', 'Aves', 'Tinamiformes', 'Tinamidae',
    NULL, 'Nothocercus', 'bonapartei', 'Highland Tinamou',
    'Observed during field trip', NULL, '2025-10-01', 'Field location 1'
);
INSERT OR IGNORE INTO sightings (
    trip_id, taxon_id, kingdom, phylum, class, "order", family, subfamily, genus, species_epithet, common_name,
    notes, media_path, date, location
) VALUES (
    1,
    (SELECT id FROM taxa WHERE rank='species' AND genus='Tinamus' AND species_epithet='major'),
    'Animalia', 'Chordata', 'Aves', 'Tinamiformes', 'Tinamidae',
    NULL, 'Tinamus', 'major', 'Great Tinamou',
    'Observed during field trip', NULL, '2025-10-02', 'Field location 2'
);
INSERT OR IGNORE INTO sightings (
    trip_id, taxon_id, kingdom, phylum, class, "order", family, subfamily, genus, species_epithet, common_name,
    notes, media_path, date, location
) VALUES (
    1,
    (SELECT id FROM taxa WHERE rank='species' AND genus='Dendrocygna' AND species_epithet='viduata'),
    'Animalia', 'Chordata', 'Aves', 'Anseriformes', 'Anatidae',
    'Dendrocygninae', 'Dendrocygna', 'viduata', 'White-faced Whistling-Duck',
    'Observed during field trip', NULL, '2025-10-03', 'Field location 3'
);
INSERT OR IGNORE INTO sightings (
    trip_id, taxon_id, kingdom, phylum, class, "order", family, subfamily, genus, species_epithet, common_name,
    notes, media_path, date, location
) VALUES (
    2,
    (SELECT id FROM taxa WHERE rank='species' AND genus='Dendrocygna' AND species_epithet='autumnalis'),
    'Animalia', 'Chordata', 'Aves', 'Anseriformes', 'Anatidae',
    'Dendrocygninae', 'Dendrocygna', 'autumnalis', 'Black-bellied Whistling-Duck',
    'Observed during field trip', NULL, '2025-10-04', 'Field location 4'
);
INSERT OR IGNORE INTO sightings (
    trip_id, taxon_id, kingdom, phylum, class, "order", family, subfamily, genus, species_epithet, common_name,
    notes, media_path, date, location
) VALUES (
    2,
    (SELECT id FROM taxa WHERE rank='species' AND genus='Ortalis' AND species_epithet='vetula'),
    'Animalia', 'Chordata', 'Aves', 'Galliformes', 'Cracidae',
    NULL, 'Ortalis', 'vetula', 'Plain Chachalaca',
    'Observed during field trip', NULL, '2025-10-05', 'Field location 5'
);
INSERT OR IGNORE INTO sightings (
    trip_id, taxon_id, kingdom, phylum, class, "order", family, subfamily, genus, species_epithet, common_name,
    notes, media_path, date, location
) VALUES (
    2,
    (SELECT id FROM taxa WHERE rank='species' AND genus='Ortalis' AND species_epithet='cinereiceps'),
    'Animalia', 'Chordata', 'Aves', 'Galliformes', 'Cracidae',
    NULL, 'Ortalis', 'cinereiceps', 'Gray-headed Chachalaca',
    'Observed during field trip', NULL, '2025-10-06', 'Field location 6'
);
INSERT OR IGNORE INTO sightings (
    trip_id, taxon_id, kingdom, phylum, class, "order", family, subfamily, genus, species_epithet, common_name,
    notes, media_path, date, location
) VALUES (
    3,
    (SELECT id FROM taxa WHERE rank='species' AND genus='Numida' AND species_epithet='meleagris'),
    'Animalia', 'Chordata', 'Aves', 'Galliformes', 'Numididae',
    NULL, 'Numida', 'meleagris', 'Helmeted Guineafowl',
    'Observed during field trip', NULL, '2025-10-07', 'Field location 7'
);
INSERT OR IGNORE INTO sightings (
    trip_id, taxon_id, kingdom, phylum, class, "order", family, subfamily, genus, species_epithet, common_name,
    notes, media_path, date, location
) VALUES (
    3,
    (SELECT id FROM taxa WHERE rank='species' AND genus='Rhynchortyx' AND species_epithet='cinctus'),
    'Animalia', 'Chordata', 'Aves', 'Galliformes', 'Odontophoridae',
    NULL, 'Rhynchortyx', 'cinctus', 'Tawny-faced Quail',
    'Observed during field trip', NULL, '2025-10-08', 'Field location 8'
);
INSERT OR IGNORE INTO sightings (
    trip_id, taxon_id, kingdom, phylum, class, "order", family, subfamily, genus, species_epithet, common_name,
    notes, media_path, date, location
) VALUES (
    3,
    (SELECT id FROM taxa WHERE rank='species' AND genus='Oreortyx' AND species_epithet='pictus'),
    'Animalia', 'Chordata', 'Aves', 'Galliformes', 'Odontophoridae',
    NULL, 'Oreortyx', 'pictus', 'Mountain Quail',
    'Observed during field trip', NULL, '2025-10-09', 'Field location 9'
);
-- Casual sightings (no trip)
INSERT OR IGNORE INTO sightings (
    trip_id, taxon_id, kingdom, phylum, class, "order", family, subfamily, genus, species_epithet, common_name,
    notes, media_path, date, location
) VALUES (
    NULL,
    (SELECT id FROM taxa WHERE rank='species' AND genus='Dendrocygna' AND species_epithet='viduata'),
    'Animalia', 'Chordata', 'Aves', 'Anseriformes', 'Anatidae',
    'Dendrocygninae', 'Dendrocygna', 'viduata', 'White-faced Whistling-Duck',
    'Backyard observation', NULL, '2025-09-11', 'Home backyard'
);
INSERT OR IGNORE INTO sightings (
    trip_id, taxon_id, kingdom, phylum, class, "order", family, subfamily, genus, species_epithet, common_name,
    notes, media_path, date, location
) VALUES (
    NULL,
    (SELECT id FROM taxa WHERE rank='species' AND genus='Dendrocygna' AND species_epithet='autumnalis'),
    'Animalia', 'Chordata', 'Aves', 'Anseriformes', 'Anatidae',
    'Dendrocygninae', 'Dendrocygna', 'autumnalis', 'Black-bellied Whistling-Duck',
    'Backyard observation', NULL, '2025-09-12', 'Home backyard'
);
INSERT OR IGNORE INTO sightings (
    trip_id, taxon_id, kingdom, phylum, class, "order", family, subfamily, genus, species_epithet, common_name,
    notes, media_path, date, location
) VALUES (
    NULL,
    (SELECT id FROM taxa WHERE rank='species' AND genus='Ortalis' AND species_epithet='vetula'),
    'Animalia', 'Chordata', 'Aves', 'Galliformes', 'Cracidae',
    NULL, 'Ortalis', 'vetula', 'Plain Chachalaca',
    'Backyard observation', NULL, '2025-09-13', 'Home backyard'
);
INSERT OR IGNORE INTO sightings (
    trip_id, taxon_id, kingdom, phylum, class, "order", family, subfamily, genus, species_epithet, common_name,
    notes, media_path, date, location
) VALUES (
    NULL,
    (SELECT id FROM taxa WHERE rank='species' AND genus='Ortalis' AND species_epithet='cinereiceps'),
    'Animalia', 'Chordata', 'Aves', 'Galliformes', 'Cracidae',
    NULL, 'Ortalis', 'cinereiceps', 'Gray-headed Chachalaca',
    'Backyard observation', NULL, '2025-09-14', 'Home backyard'
);
INSERT OR IGNORE INTO sightings (
    trip_id, taxon_id, kingdom, phylum, class, "order", family, subfamily, genus, species_epithet, common_name,
    notes, media_path, date, location
) VALUES (
    NULL,
    (SELECT id FROM taxa WHERE rank='species' AND genus='Numida' AND species_epithet='meleagris'),
    'Animalia', 'Chordata', 'Aves', 'Galliformes', 'Numididae',
    NULL, 'Numida', 'meleagris', 'Helmeted Guineafowl',
    'Backyard observation', NULL, '2025-09-15', 'Home backyard'
);
INSERT OR IGNORE INTO sightings (
    trip_id, taxon_id, kingdom, phylum, class, "order", family, subfamily, genus, species_epithet, common_name,
    notes, media_path, date, location
) VALUES (
    NULL,
    (SELECT id FROM taxa WHERE rank='species' AND genus='Rhynchortyx' AND species_epithet='cinctus'),
    'Animalia', 'Chordata', 'Aves', 'Galliformes', 'Odontophoridae',
    NULL, 'Rhynchortyx', 'cinctus', 'Tawny-faced Quail',
    'Backyard observation', NULL, '2025-09-16', 'Home backyard'
);
INSERT OR IGNORE INTO sightings (
    trip_id, taxon_id, kingdom, phylum, class, "order", family, subfamily, genus, species_epithet, common_name,
    notes, media_path, date, location
) VALUES (
    NULL,
    (SELECT id FROM taxa WHERE rank='species' AND genus='Oreortyx' AND species_epithet='pictus'),
    'Animalia', 'Chordata', 'Aves', 'Galliformes', 'Odontophoridae',
    NULL, 'Oreortyx', 'pictus', 'Mountain Quail',
    'Backyard observation', NULL, '2025-09-17', 'Home backyard'
);
