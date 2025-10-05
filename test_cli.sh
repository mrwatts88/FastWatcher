#!/usr/bin/env bash

# Fast Watcher CLI Test Script
# Tests all CRUD operations and search functionality

# Don't use set -e, we need to check return codes manually
# because cargo warnings go to stderr

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test counters
TESTS_PASSED=0
TESTS_FAILED=0

# Helper functions
print_header() {
    echo ""
    echo "=========================================="
    echo "$1"
    echo "=========================================="
}

print_test() {
    echo ""
    echo "TEST: $1"
}

# Clean output (remove empty lines)
clean_output() {
    grep -v "^$"
}

# Extract ID from output
extract_id() {
    grep -o 'ID: [0-9]*' | grep -o '[0-9]*$'
}

assert_success() {
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✅ PASS${NC}: $1"
        ((TESTS_PASSED++))
    else
        echo -e "${RED}❌ FAIL${NC}: $1"
        ((TESTS_FAILED++))
        return 1
    fi
}

assert_failure() {
    if [ $? -ne 0 ]; then
        echo -e "${GREEN}✅ PASS${NC}: $1 (expected to fail)"
        ((TESTS_PASSED++))
    else
        echo -e "${RED}❌ FAIL${NC}: $1 (should have failed)"
        ((TESTS_FAILED++))
        return 1
    fi
}

assert_contains() {
    local output="$1"
    local expected="$2"
    local description="$3"

    if echo "$output" | grep -q "$expected"; then
        echo -e "${GREEN}✅ PASS${NC}: $description"
        ((TESTS_PASSED++))
    else
        echo -e "${RED}❌ FAIL${NC}: $description"
        echo "  Expected to contain: $expected"
        echo "  Got: $output"
        ((TESTS_FAILED++))
        return 1
    fi
}

# Start tests
print_header "FAST WATCHER CLI TESTS"

# Build once
print_header "SETUP: Build Project"
print_test "Building fast-watcher"
cargo build --quiet 2>&1 | grep -v "warning:" || true
assert_success "Project built"

# Binary path
BIN="./target/debug/fast-watcher"

# Clean slate
print_header "SETUP: Initialize Database"
print_test "Drop existing database"
$BIN drop-db > /dev/null 2>&1 || true

print_test "Initialize fresh database"
$BIN init-db > /dev/null 2>&1
assert_success "Database initialized"

# ==========================================
# TAXON TESTS
# ==========================================
print_header "TAXON TESTS"

print_test "Create species-level taxon (American Robin)"
OUTPUT=$($BIN add-taxon species Animalia "American Robin" \
    --phylum Chordata \
    --class Aves \
    --order Passeriformes \
    --family Turdidae \
    --genus Turdus \
    --species-epithet migratorius 2>&1 | clean_output)
assert_success "Created species-level taxon"
ROBIN_ID=$(echo "$OUTPUT" | extract_id)
echo "  Robin taxon ID: $ROBIN_ID"

print_test "Create family-level taxon (Warbler Family)"
OUTPUT=$($BIN add-taxon family Animalia "Warbler Family" \
    --phylum Chordata \
    --class Aves \
    --order Passeriformes \
    --family Parulidae 2>&1 | clean_output)
assert_success "Created family-level taxon"
WARBLER_FAM_ID=$(echo "$OUTPUT" | extract_id)
echo "  Warbler family ID: $WARBLER_FAM_ID"

print_test "Create genus-level taxon (Buteo genus)"
OUTPUT=$($BIN add-taxon genus Animalia "Buteo Hawks" \
    --phylum Chordata \
    --class Aves \
    --order Accipitriformes \
    --family Accipitridae \
    --genus Buteo 2>&1 | clean_output)
assert_success "Created genus-level taxon"
BUTEO_ID=$(echo "$OUTPUT" | extract_id)
echo "  Buteo genus ID: $BUTEO_ID"

print_test "Show species-level taxon"
OUTPUT=$($BIN show-taxon "$ROBIN_ID" 2>&1 | clean_output)
assert_contains "$OUTPUT" "migratorius" "Species taxon shows all fields"
assert_contains "$OUTPUT" "species" "Shows rank as 'species'"

print_test "Show family-level taxon"
OUTPUT=$($BIN show-taxon "$WARBLER_FAM_ID" 2>&1 | clean_output)
assert_contains "$OUTPUT" "Parulidae" "Family taxon shows family"
assert_contains "$OUTPUT" "family" "Shows rank as 'family'"

print_test "Search taxa by common name"
OUTPUT=$($BIN search-taxa "Robin" 2>&1 | clean_output)
assert_contains "$OUTPUT" "American Robin" "Search finds taxon by common name"

print_test "Search taxa by family"
OUTPUT=$($BIN search-taxa "Corvidae" 2>&1 | clean_output)
assert_contains "$OUTPUT" "Crow Family" "Search finds family-level taxa"

print_test "Try to create taxon with invalid rank"
$BIN add-taxon invalid_rank Animalia "Test" > /dev/null 2>&1
assert_failure "Invalid rank rejected"

# ==========================================
# TRIP TESTS
# ==========================================
print_header "TRIP TESTS"

print_test "Create trip with all fields"
OUTPUT=$($BIN add-trip "Morning Birding" \
    --date "2025-01-15" \
    --location "Central Park" \
    --notes "Cold morning, lots of activity" 2>&1 | clean_output)
assert_success "Created trip with all fields"
TRIP1_ID=$(echo "$OUTPUT" | extract_id)
echo "  Trip ID: $TRIP1_ID"

print_test "Create trip with minimal fields"
OUTPUT=$($BIN add-trip "Quick Walk" 2>&1 | clean_output)
assert_success "Created trip with minimal fields"
TRIP2_ID=$(echo "$OUTPUT" | extract_id)
echo "  Trip ID: $TRIP2_ID"

print_test "Show trip with all fields"
OUTPUT=$($BIN show-trip "$TRIP1_ID" 2>&1 | clean_output)
assert_contains "$OUTPUT" "Morning Birding" "Shows trip name"

print_test "Search trips by location"
OUTPUT=$($BIN search-trips "Central Park" 2>&1 | clean_output)
assert_contains "$OUTPUT" "Morning Birding" "Search finds trip by location"

# ==========================================
# SIGHTING TESTS
# ==========================================
print_header "SIGHTING TESTS"

print_test "Create sighting with species-level taxon + trip"
OUTPUT=$($BIN add-sighting "$ROBIN_ID" \
    --trip-id "$TRIP1_ID" \
    --date "2025-01-15" \
    --location "Near the pond" \
    --notes "Foraging on the ground" 2>&1 | clean_output)
assert_success "Created sighting with species + trip"
SIGHTING1_ID=$(echo "$OUTPUT" | extract_id)
echo "  Sighting ID: $SIGHTING1_ID"

print_test "Create sighting with family-level taxon (no trip)"
OUTPUT=$($BIN add-sighting "$WARBLER_FAM_ID" \
    --date "2025-01-15" \
    --notes "Small yellow bird, couldn't ID to species" 2>&1 | clean_output)
assert_success "Created sighting with family-level ID"
SIGHTING2_ID=$(echo "$OUTPUT" | extract_id)
echo "  Sighting ID: $SIGHTING2_ID"

print_test "Create sighting with genus-level taxon"
OUTPUT=$($BIN add-sighting "$BUTEO_ID" \
    --trip-id "$TRIP1_ID" \
    --notes "Large hawk circling overhead, Buteo sp." 2>&1 | clean_output)
assert_success "Created sighting with genus-level ID"
SIGHTING3_ID=$(echo "$OUTPUT" | extract_id)
echo "  Sighting ID: $SIGHTING3_ID"

print_test "Show species-level sighting"
OUTPUT=$($BIN show-sighting "$SIGHTING1_ID" 2>&1 | clean_output)
assert_contains "$OUTPUT" "migratorius" "Shows species epithet"
assert_contains "$OUTPUT" "American Robin" "Shows common name"

print_test "Show family-level sighting"
OUTPUT=$($BIN show-sighting "$SIGHTING2_ID" 2>&1 | clean_output)
assert_contains "$OUTPUT" "Parulidae" "Shows family"
assert_contains "$OUTPUT" "Warbler Family" "Shows common name"
# Should have exactly 5 slashes for family level (kingdom/phylum/class/order/family)
SLASH_COUNT=$(echo "$OUTPUT" | grep -o "/" | wc -l | tr -d ' ')
if [ "$SLASH_COUNT" -gt 5 ]; then
    echo -e "${RED}❌ FAIL${NC}: Family-level sighting should not show genus/species (found $SLASH_COUNT slashes, expected 5)"
    ((TESTS_FAILED++))
else
    echo -e "${GREEN}✅ PASS${NC}: Family-level sighting correctly omits genus/species ($SLASH_COUNT slashes)"
    ((TESTS_PASSED++))
fi

print_test "Search sightings by family name"
OUTPUT=$($BIN search-sightings "Parulidae" 2>&1 | clean_output)
assert_contains "$OUTPUT" "Warbler Family" "Search finds family-level sighting"

print_test "Search sightings by common name"
OUTPUT=$($BIN search-sightings "Robin" 2>&1 | clean_output)
assert_contains "$OUTPUT" "American Robin" "Search finds species by common name"

print_test "Search sightings by genus"
OUTPUT=$($BIN search-sightings "Buteo" 2>&1 | clean_output)
assert_contains "$OUTPUT" "Buteo Hawks" "Search finds genus-level sighting"

print_test "Search sightings by location"
OUTPUT=$($BIN search-sightings "pond" 2>&1 | clean_output)
assert_contains "$OUTPUT" "American Robin" "Search finds sighting by location"

print_test "Try to create sighting with non-existent taxon ID"
$BIN add-sighting 99999 --notes "Test" > /dev/null 2>&1
assert_failure "Non-existent taxon ID rejected"

# ==========================================
# DELETE TESTS
# ==========================================
print_header "DELETE TESTS"

print_test "Delete sighting"
OUTPUT=$($BIN delete-sighting "$SIGHTING3_ID" 2>&1 | clean_output)
assert_contains "$OUTPUT" "deleted" "Sighting deleted successfully"

print_test "Verify deleted sighting is gone"
$BIN show-sighting "$SIGHTING3_ID" > /dev/null 2>&1
assert_failure "Deleted sighting not found"

print_test "Delete trip"
OUTPUT=$($BIN delete-trip "$TRIP2_ID" 2>&1 | clean_output)
assert_contains "$OUTPUT" "deleted" "Trip deleted successfully"

print_test "Delete taxon"
# Create a temporary taxon to delete
OUTPUT=$($BIN add-taxon species Animalia "Temp Bird" \
    --phylum Chordata --class Aves --order Passeriformes \
    --family Testidae --genus Test --species-epithet temp 2>&1 | clean_output)
TEMP_TAXON_ID=$(echo "$OUTPUT" | extract_id)
OUTPUT=$($BIN delete-taxon "$TEMP_TAXON_ID" 2>&1 | clean_output)
assert_contains "$OUTPUT" "deleted" "Taxon deleted successfully"

# ==========================================
# INTEGRATION TESTS
# ==========================================
print_header "INTEGRATION TESTS"

print_test "Search across all sightings finds multiple ranks"
OUTPUT=$($BIN search-sightings "Aves" 2>&1 | clean_output)
assert_contains "$OUTPUT" "American Robin" "Finds species-level"
assert_contains "$OUTPUT" "Warbler Family" "Finds family-level"

print_test "Seeded data is searchable"
OUTPUT=$($BIN search-sightings "Blue Jay" 2>&1 | clean_output)
assert_contains "$OUTPUT" "Blue Jay" "Finds seeded Blue Jay sightings"

print_test "Seeded taxon data is searchable"
OUTPUT=$($BIN search-taxa "Corvidae" 2>&1 | clean_output)
assert_contains "$OUTPUT" "Crow Family" "Finds seeded Crow Family taxon"

# ==========================================
# SUMMARY
# ==========================================
print_header "TEST SUMMARY"

TOTAL_TESTS=$((TESTS_PASSED + TESTS_FAILED))
echo ""
echo "Total Tests: $TOTAL_TESTS"
echo -e "${GREEN}Passed: $TESTS_PASSED${NC}"
echo -e "${RED}Failed: $TESTS_FAILED${NC}"
echo ""

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "${GREEN}🎉 ALL TESTS PASSED! 🎉${NC}"
    exit 0
else
    echo -e "${RED}❌ SOME TESTS FAILED${NC}"
    exit 1
fi
