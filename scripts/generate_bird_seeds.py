#!/usr/bin/env python3
"""
Generate SQL seed files from NACC bird species list.

Outputs:
- seed_taxa_full.sql: All 2,220 species + parent taxa
- seed_taxa_test.sql: First 100 species + their parent taxa
- seed_sightings.sql: Sample sightings using birds from first 100

Usage:
    cd scripts
    python generate_bird_seeds.py
"""

import pandas as pd
import os

def extract_species_epithet(binomial_str):
    """Extract species epithet from 'Genus species' format"""
    if pd.isna(binomial_str) or not binomial_str:
        return None
    parts = str(binomial_str).strip().split()
    return parts[-1] if len(parts) >= 2 else None

def sql_value(val):
    """Convert Python value to SQL literal (NULL or 'string')"""
    if pd.isna(val) or val == '' or val is None:
        return 'NULL'
    # Escape single quotes in strings by doubling them
    escaped = str(val).replace("'", "''")
    return f"'{escaped}'"

def generate_parent_taxa(df_subset):
    """
    Generate unique parent taxa from species subset.
    Returns: (orders_df, families_df, subfamilies_df, genera_df)
    """
    # Orders (with common name = order name)
    orders = df_subset[['kingdom', 'phylum', 'class', 'order']].drop_duplicates()
    orders['rank'] = 'order'
    orders['family'] = None
    orders['subfamily'] = None
    orders['genus'] = None
    orders['species_epithet'] = None
    orders['common_name'] = orders['order']

    # Families (with common name = family name)
    families = df_subset[['kingdom', 'phylum', 'class', 'order', 'family']].drop_duplicates()
    families['rank'] = 'family'
    families['subfamily'] = None
    families['genus'] = None
    families['species_epithet'] = None
    families['common_name'] = families['family']

    # Subfamilies (only where subfamily exists)
    subfamilies = df_subset[df_subset['subfamily'].notna() & (df_subset['subfamily'] != '')][
        ['kingdom', 'phylum', 'class', 'order', 'family', 'subfamily']
    ].drop_duplicates()
    subfamilies['rank'] = 'subfamily'
    subfamilies['genus'] = None
    subfamilies['species_epithet'] = None
    subfamilies['common_name'] = subfamilies['subfamily']

    # Genera (with common name = genus name)
    genera = df_subset[['kingdom', 'phylum', 'class', 'order', 'family', 'subfamily', 'genus']].drop_duplicates()
    genera['rank'] = 'genus'
    genera['species_epithet'] = None
    genera['common_name'] = genera['genus']

    return orders, families, subfamilies, genera

def generate_taxon_insert(row):
    """Generate SQL INSERT statement for a single taxon"""
    return f"""INSERT OR IGNORE INTO taxa (
    rank, kingdom, phylum, class, "order", family, subfamily, genus, species_epithet, common_name
) VALUES (
    '{row['rank']}',
    '{row['kingdom']}',
    {sql_value(row.get('phylum'))},
    {sql_value(row.get('class'))},
    {sql_value(row.get('order'))},
    {sql_value(row.get('family'))},
    {sql_value(row.get('subfamily'))},
    {sql_value(row.get('genus'))},
    {sql_value(row.get('species_epithet'))},
    {sql_value(row['common_name'])}
);"""

def write_taxa_sql(df_subset, output_path, description):
    """Write taxa SQL file with parent taxa + species"""
    orders, families, subfamilies, genera = generate_parent_taxa(df_subset)

    with open(output_path, 'w', encoding='utf-8') as f:
        f.write(f"-- {description}\n")
        f.write("-- Generated from NACC bird species list\n")
        f.write("-- DO NOT EDIT MANUALLY - regenerate using scripts/generate_bird_seeds.py\n\n")

        # Insert orders
        f.write("-- Orders\n")
        for _, row in orders.iterrows():
            f.write(generate_taxon_insert(row) + "\n")

        # Insert families
        f.write("\n-- Families\n")
        for _, row in families.iterrows():
            f.write(generate_taxon_insert(row) + "\n")

        # Insert subfamilies (if any)
        if not subfamilies.empty:
            f.write("\n-- Subfamilies\n")
            for _, row in subfamilies.iterrows():
                f.write(generate_taxon_insert(row) + "\n")

        # Insert genera
        f.write("\n-- Genera\n")
        for _, row in genera.iterrows():
            f.write(generate_taxon_insert(row) + "\n")

        # Insert species
        f.write("\n-- Species\n")
        for _, row in df_subset.iterrows():
            f.write(generate_taxon_insert(row) + "\n")

    print(f"  Wrote {output_path}")
    print(f"    - {len(orders)} orders")
    print(f"    - {len(families)} families")
    print(f"    - {len(subfamilies)} subfamilies")
    print(f"    - {len(genera)} genera")
    print(f"    - {len(df_subset)} species")
    print(f"    Total: {len(orders) + len(families) + len(subfamilies) + len(genera) + len(df_subset)} taxa")

def generate_sightings_sql(df_first_100, output_path):
    """Generate sample sightings using birds from first 100 species"""
    # Pick diverse species for sighting examples
    # Get 2 from each of first 8 different families
    sample_birds = df_first_100.groupby('family').head(2).head(16)

    with open(output_path, 'w', encoding='utf-8') as f:
        f.write("-- Generated sample sightings from NACC bird data\n")
        f.write("-- Uses birds from first 100 species for test compatibility\n")
        f.write("-- DO NOT EDIT MANUALLY - regenerate using scripts/generate_bird_seeds.py\n\n")

        # Generate sightings with trips (3 sightings per trip)
        f.write("-- Sightings with trips\n")
        trip_sightings = sample_birds.head(9)
        for i, (_, bird) in enumerate(trip_sightings.iterrows(), 1):
            trip_id = ((i - 1) // 3) + 1  # 3 sightings per trip (trips 1-3)
            epithet = bird['species_epithet']
            f.write(f"""INSERT OR IGNORE INTO sightings (
    trip_id, taxon_id, kingdom, phylum, class, "order", family, subfamily, genus, species_epithet, common_name,
    notes, media_path, date, location
) VALUES (
    {trip_id},
    (SELECT id FROM taxa WHERE rank='species' AND genus='{bird['genus']}' AND species_epithet='{epithet}'),
    '{bird['kingdom']}', '{bird['phylum']}', '{bird['class']}', '{bird['order']}', '{bird['family']}',
    {sql_value(bird['subfamily'])}, '{bird['genus']}', '{epithet}', {sql_value(bird['common_name'])},
    'Observed during field trip', NULL, '2025-10-0{((i - 1) % 9) + 1}', 'Field location {i}'
);
""")

        # Generate casual sightings (no trip)
        f.write("-- Casual sightings (no trip)\n")
        casual_sightings = sample_birds.tail(7)
        for i, (_, bird) in enumerate(casual_sightings.iterrows(), 1):
            epithet = bird['species_epithet']
            f.write(f"""INSERT OR IGNORE INTO sightings (
    trip_id, taxon_id, kingdom, phylum, class, "order", family, subfamily, genus, species_epithet, common_name,
    notes, media_path, date, location
) VALUES (
    NULL,
    (SELECT id FROM taxa WHERE rank='species' AND genus='{bird['genus']}' AND species_epithet='{epithet}'),
    '{bird['kingdom']}', '{bird['phylum']}', '{bird['class']}', '{bird['order']}', '{bird['family']}',
    {sql_value(bird['subfamily'])}, '{bird['genus']}', '{epithet}', {sql_value(bird['common_name'])},
    'Backyard observation', NULL, '2025-09-{10 + i}', 'Home backyard'
);
""")

    print(f"  Wrote {output_path}")
    print(f"    - {len(trip_sightings)} sightings with trips")
    print(f"    - {len(casual_sightings)} casual sightings")
    print(f"    Total: {len(trip_sightings) + len(casual_sightings)} sightings")

def main():
    # Change to project root directory
    script_dir = os.path.dirname(os.path.abspath(__file__))
    project_root = os.path.dirname(script_dir)
    os.chdir(project_root)

    print("Reading NACC species list...")
    csv_path = 'NACC_list_species.csv'
    if not os.path.exists(csv_path):
        print(f"ERROR: {csv_path} not found in {os.getcwd()}")
        print("Expected file location: /Users/mattwatts/code/fast-watcher/NACC_list_species.csv")
        return 1

    df = pd.read_csv(csv_path)

    # Add bird constants
    df['kingdom'] = 'Animalia'
    df['phylum'] = 'Chordata'
    df['class'] = 'Aves'

    # Extract species epithet from binomial
    df['species_epithet'] = df['species'].apply(extract_species_epithet)

    # Ensure rank is 'species'
    df['rank'] = 'species'

    print(f"Loaded {len(df)} species\n")

    # Generate full dataset
    print("Generating seed_taxa_full.sql (all species)...")
    write_taxa_sql(df, 'seed_taxa_full.sql',
                   "NACC North American Birds - Full Dataset (All 2,220 species)")

    # Generate test dataset (first 100)
    print("\nGenerating seed_taxa_test.sql (first 100 species)...")
    df_test = df.head(100)
    write_taxa_sql(df_test, 'seed_taxa_test.sql',
                   "NACC North American Birds - Test Subset (First 100 species)")

    # Generate sightings using first 100
    print("\nGenerating seed_sightings.sql...")
    generate_sightings_sql(df_test, 'seed_sightings.sql')

    print("\n✅ Done! Generated files:")
    print("  - seed_taxa_full.sql")
    print("  - seed_taxa_test.sql")
    print("  - seed_sightings.sql")
    print("\nNext steps:")
    print("  1. cargo test  (uses test subset)")
    print("  2. cargo run -- drop-db && cargo run -- init-db  (uses full dataset)")

    return 0

if __name__ == '__main__':
    exit(main())
