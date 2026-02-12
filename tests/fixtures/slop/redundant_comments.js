// expect: slop-001
// Set the user name
user.setUserName(name);

// expect: slop-001
// Get the user name
const userName = getUserName();

// expect: slop-001
// Create the new user account
createNewUserAccount();

// This should NOT trigger — comment adds context beyond the code
// Normalize Unicode before comparison to handle locale-specific equivalence
user.setName(normalizeName(name));

// This should NOT trigger — explains WHY, not WHAT
// Using post-increment here because the value is read before update
counter++;

// This should NOT trigger — too short (fewer than 3 meaningful words)
// Do it
doSomething();

// This should NOT trigger — JSDoc block
/**
 * @param {string} name - The user's name
 * @returns {void}
 */
function setName(name) {
    this.name = name;
}

// This should NOT trigger — TODO directive
// TODO: refactor this later
processData();

// expect: slop-001
// Initialize the application config
initializeApplicationConfig();

// expect: slop-001
// Update the user profile data
updateUserProfileData(data);
