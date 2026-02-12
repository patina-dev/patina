// SPDX-FileCopyrightText: 2026 Steven Mosley <astrosteveo>
//
// SPDX-License-Identifier: GPL-3.0-only

// Normalize Unicode before comparison to handle locale-specific equivalence
user.setName(normalizeName(name));

// Using post-increment here because the value is read before update
counter++;

// Early return — downstream expects null, not undefined
function getValue(key) {
    const result = cache.get(key);
    return result ?? null;
}

// Retry with exponential backoff to avoid thundering herd
async function fetchWithRetry(url, attempts) {
    for (let i = 0; i < attempts; i++) {
        try {
            return await fetch(url);
        } catch (err) {
            await sleep(Math.pow(2, i) * 1000);
        }
    }
    throw new Error("Max retries exceeded");
}

/**
 * @param {string} name - The user's display name
 * @returns {boolean} Whether the name was updated successfully
 */
function updateDisplayName(name) {
    if (!name || name.trim().length === 0) {
        return false;
    }
    this.displayName = name.trim();
    return true;
}

// TODO: Replace with a proper LRU cache when we exceed 10k entries
const cache = new Map();
