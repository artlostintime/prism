// Debug script for testing tab switching
// Paste this into the browser console

console.log("=== TAB DEBUG ===");

// Check if elements exist
const contentTools = document.getElementById("contentTools");
const contentResults = document.getElementById("contentResults");

console.log("contentTools element:", contentTools);
console.log("contentResults element:", contentResults);

if (contentTools) {
  console.log("contentTools HTML length:", contentTools.innerHTML.length);
  console.log(
    "contentTools display:",
    window.getComputedStyle(contentTools).display
  );
  console.log(
    "contentTools opacity:",
    window.getComputedStyle(contentTools).opacity
  );
  console.log("contentTools height:", contentTools.offsetHeight);
  console.log("contentTools classes:", contentTools.className);
}

if (contentResults) {
  console.log("contentResults HTML length:", contentResults.innerHTML.length);
  console.log(
    "contentResults display:",
    window.getComputedStyle(contentResults).display
  );
  console.log(
    "contentResults opacity:",
    window.getComputedStyle(contentResults).opacity
  );
  console.log("contentResults height:", contentResults.offsetHeight);
  console.log("contentResults classes:", contentResults.className);
}

// Test switching to tools
console.log("\n=== SWITCHING TO TOOLS ===");
switchTab("tools");

setTimeout(() => {
  console.log("After switch:");
  console.log("contentTools classes:", contentTools.className);
  console.log(
    "contentTools display:",
    window.getComputedStyle(contentTools).display
  );
  console.log(
    "contentTools opacity:",
    window.getComputedStyle(contentTools).opacity
  );
  console.log("contentTools height:", contentTools.offsetHeight);
}, 500);
