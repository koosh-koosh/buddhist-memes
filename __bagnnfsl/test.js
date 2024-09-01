const fs = require("fs");
const readline = require("readline");

// // Read JSON file
const jsonData = JSON.parse(fs.readFileSync("input.json", "utf8"));

// // Function to search for matches
// function searchEntries(searchTerm, entries) {
// 	const matchedEntries = [];
// 	for (const key in entries) {
// 		const entry = entries[key];
// 		const lowerContent = entry.content.toLowerCase();
// 		if (entry.keys.some((k) => lowerContent.includes(searchTerm.toLowerCase()))) {
// 			matchedEntries.push(entry);
// 			delete entries[key];
// 			const words = entry.content.split(/\s+/);
// 			words.forEach((word) => {
// 				if (word.toLowerCase().includes(searchTerm.toLowerCase())) {
// 					searchEntries(word, entries);
// 				}
// 			});
// 		}
// 	}
// 	return matchedEntries;
// }

// // Create interface to take user input
// const rl = readline.createInterface({
// 	input: process.stdin,
// 	output: process.stdout,
// });

// // Ask user for input
// rl.question("Enter search term: ", (searchTerm) => {
// 	// Search for matches
// 	const matchedEntries = searchEntries(searchTerm, jsonData.entries);

// 	// Display matched entries
// 	console.log("Matched Entries:");
// 	matchedEntries.forEach((entry, index) => {
// 		console.log(`${index + 1}. Content: ${entry.content}`);
// 	});

// 	// Close interface
// 	rl.close();
// });

function findEntriesByKey(keyToFind, entries, foundEntries = new Set(), depth = 0) {
	if (depth > 2) {
		// Preventing infinite recursion
		// console.warn("Maximum recursion depth reached.");
		return;
	}

	for (let entryId in entries) {
		const entry = entries[entryId];
		// Check if the entry has been processed already
		if (foundEntries.has(entryId)) continue;

		// Case-insensitive search in keys
		if (entry.keys.some((key) => key.toLowerCase() === keyToFind.toLowerCase())) {
			foundEntries.add(entryId);

			// Recursively search within content
			const contentWords = entry.content.match(/\b(\w+)\b/g);
			if (contentWords) {
				contentWords.forEach((word) => findEntriesByKey(word, entries, foundEntries, depth + 1));
			}
		}
	}
	return foundEntries;
}

function displayEntries(foundEntriesIds, allEntries) {
	const entries = Array.from(foundEntriesIds).map((id) => [allEntries[id].keys, allEntries[id].content]);
	entries.forEach(([a, b]) => {
		console.log("<world_info>");
		console.log(a.map((v) => `<name>${v}</name>`).join("\n"));
		console.log("<description>");
		console.log(b);
		console.log("</description>");
		console.log("</world_info>");
		console.log();
	});
}

const rl = readline.createInterface({
	input: process.stdin,
	output: process.stderr,
});

(function main() {
	rl.question("Enter search term: ", (searchTerm) => {
		// Search for matches
		const foundEntries = findEntriesByKey(searchTerm, jsonData.entries);
		displayEntries(foundEntries, jsonData.entries);

		// Close interface
		rl.close();
	});
})();
