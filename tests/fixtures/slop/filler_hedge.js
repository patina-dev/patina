// SPDX-FileCopyrightText: 2026 Steven Mosley <astrosteveo>
//
// SPDX-License-Identifier: GPL-3.0-only

// expect: slop-003
// Basically, this sets the user's name
user.setName(name);

// expect: slop-003
// Simply put this handles the validation logic
function validate(input) {
    return input != null;
}

// expect: slop-003
// Obviously, we return the cached value here
function getCached(key) {
    return cache.get(key);
}

// expect: slop-003
// It's worth noting that this resets the counter
let counter = 0;

// expect: slop-003
// Please note the timeout is set to 30 seconds
const timeout = 30000;

// expect: slop-003
// Essentially, we just basically filter the invalid items
function filterItems(items) {
    return items.filter(Boolean);
}

// This should NOT trigger — "Note:" followed by a reference
// Note: see RFC 7231 for details on status codes
function handleStatus(code) {
    return code >= 200 && code < 300;
}

// This should NOT trigger — normal explanatory comment
// Retry with exponential backoff to avoid thundering herd
async function fetchWithRetry(url) {
    return await fetch(url);
}

// This should NOT trigger — JSDoc block
/**
 * Just a simple helper for formatting dates
 */
function formatDate(date) {
    return date.toISOString();
}

// This should NOT trigger — explains a real constraint
// The buffer must be exactly 1024 bytes for the hardware interface
const BUFFER_SIZE = 1024;
