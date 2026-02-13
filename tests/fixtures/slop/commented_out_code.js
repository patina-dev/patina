// SPDX-FileCopyrightText: 2026 Steven Mosley <astrosteveo>
//
// SPDX-License-Identifier: GPL-3.0-only

// expect: slop-004
// return result;
function getValue() {
    return null;
}

// expect: slop-004
// const config = getConfig();
// const user = fetchUser(config.userId);
// if (user.isActive) {
//     return user.profile;
// }
function loadProfile() {
    return null;
}

// This should NOT trigger — normal prose comment
// Users must wait for the async operation to complete
async function fetchData(url) {
    return await fetch(url);
}

// This should NOT trigger — JSDoc block
/**
 * const x = 10;
 * return x + 1;
 */
function example() {
    return 42;
}

// This should NOT trigger — expect annotation
// expect: slop-001
// Set the user name
user.setUserName(name);

// This should NOT trigger — explains why, not code
// Early return for null safety — downstream expects a value
function safeGet(obj, key) {
    if (!obj) return null;
    return obj[key];
}
