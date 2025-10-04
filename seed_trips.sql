-- ==========================
-- FastWatcher Trips Seed Data
-- ==========================
INSERT
    OR IGNORE INTO trips (id, name, date, location, notes)
VALUES
    (
        1,
        'Ozark Trail Exploration',
        '2025-10-04',
        'Missouri, USA',
        'First autumn trip on the Ozark Trail. Clear skies, mild temperature. Excellent bird activity.'
    );

INSERT
    OR IGNORE INTO trips (id, name, date, location, notes)
VALUES
    (
        2,
        'Lake Michigan Shoreline Walk',
        '2025-08-14',
        'Milwaukee, WI, USA',
        'Morning hike along the lakefront. Notable gulls, hawks, and several migrating songbirds.'
    );

INSERT
    OR IGNORE INTO trips (id, name, date, location, notes)
VALUES
    (
        3,
        'Shawnee National Forest',
        '2025-05-22',
        'Illinois, USA',
        'Weekend trip through mixed deciduous forest. Dense canopy, good diversity of species.'
    );