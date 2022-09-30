const { readFileSync } = require("fs");

const text = readFileSync("/bin/true", {encoding: "utf8"});
console.log(text);
