-- Species-level taxa
INSERT
    OR IGNORE INTO taxa (
        rank,
        kingdom,
        phylum,
        class,
        "order",
        family,
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
        'Crow Family'
    );