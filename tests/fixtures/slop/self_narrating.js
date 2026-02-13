// SPDX-FileCopyrightText: 2026 Steven Mosley <astrosteveo>
//
// SPDX-License-Identifier: GPL-3.0-only

// expect: slop-005
// Here we initialize the configuration
const config = {};

// expect: slop-005
// We need to validate the input before processing
function validate(input) {
    return input != null;
}

// expect: slop-005
// This function handles the data transformation
function transform(data) {
    return data.map(String);
}

// expect: slop-005
// This handles the error case for invalid inputs
function handleError(err) {
    console.error(err);
}

// expect: slop-005
// The following code sets up the database connection
const db = null;

// This should NOT trigger — explains WHY not WHAT
// Normalize Unicode before comparison to handle locale-specific equivalence
user.setName(normalizeName(name));

// This should NOT trigger — JSDoc
/**
 * This function formats dates for display
 */
function formatDate(date) {
    return date.toISOString();
}

// This should NOT trigger — TODO directive
// TODO: we should refactor this later
function legacyCode() {
    return true;
}
