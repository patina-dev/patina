// SPDX-FileCopyrightText: 2026 Steven Mosley <astrosteveo>
//
// SPDX-License-Identifier: GPL-3.0-only

// expect: slop-002
// Wait — this might not handle edge cases correctly
function processData(data) {
    return data.filter(Boolean);
}

// expect: slop-002
// Actually, let me reconsider the approach here
function validateInput(input) {
    return input != null;
}

// expect: slop-002
// Hmm, I think we should use a different data structure
const cache = new Map();

// expect: slop-002
// Let me think about the performance implications
function sortItems(items) {
    return items.sort((a, b) => a - b);
}

// expect: slop-002
// On second thought, this is the better approach
function formatDate(date) {
    return date.toISOString();
}

// This should NOT trigger — legitimate use of "wait" in context
// Users must wait for the async operation to complete
async function fetchData(url) {
    return await fetch(url);
}

// This should NOT trigger — "actually" mid-sentence, not reasoning
// The server actually returns a 204 for empty responses
function handleResponse(res) {
    return res.status;
}

// This should NOT trigger — normal TODO comment
// TODO: think about caching strategy
function getUser(id) {
    return db.find(id);
}
