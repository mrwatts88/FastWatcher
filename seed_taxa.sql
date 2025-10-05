-- Species-level taxa
INSERT
    OR IGNORE INTO taxa (
        rank,
        kingdom,
        phylum,
        class,
        "order",
        family,
        subfamily,
        genus,
        species_epithet,
        common_name
    )
VALUES
    (
        'species',
        'Animalia',
        'Chordata',
        'Aves',
        'Passeriformes',
        'Corvidae',
        NULL,
        'Cyanocitta',
        'cristata',
        'Blue Jay'
    );

INSERT
    OR IGNORE INTO taxa (
        rank,
        kingdom,
        phylum,
        class,
        "order",
        family,
        subfamily,
        genus,
        species_epithet,
        common_name
    )
VALUES
    (
        'species',
        'Animalia',
        'Chordata',
        'Aves',
        'Accipitriformes',
        'Accipitridae',
        NULL,
        'Buteo',
        'jamaicensis',
        'Red-tailed Hawk'
    );

INSERT
    OR IGNORE INTO taxa (
        rank,
        kingdom,
        phylum,
        class,
        "order",
        family,
        subfamily,
        genus,
        species_epithet,
        common_name
    )
VALUES
    (
        'species',
        'Animalia',
        'Chordata',
        'Mammalia',
        'Primates',
        'Hominidae',
        NULL,
        'Homo',
        'sapiens',
        'Human'
    );

-- Family-level taxon (example of partial identification)
INSERT
    OR IGNORE INTO taxa (
        rank,
        kingdom,
        phylum,
        class,
        "order",
        family,
        subfamily,
        genus,
        species_epithet,
        common_name
    )
VALUES
    (
        'family',
        'Animalia',
        'Chordata',
        'Aves',
        'Passeriformes',
        'Corvidae',
        NULL,
        NULL,
        NULL,
        'Crow Family'
    );