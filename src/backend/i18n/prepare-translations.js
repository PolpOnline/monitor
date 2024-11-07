#!/usr/bin/node

const fs = require("fs");
const path = require("path");
const process = require("process");

const flattenJson = (obj, prefix = "") => {
  let result = {};
  for (const key in obj) {
    if (obj.hasOwnProperty(key)) {
      const newKey = prefix ? `${prefix}.${key}` : key;
      if (
        typeof obj[key] === "object" &&
        obj[key] !== null &&
        !Array.isArray(obj[key])
      ) {
        result = { ...result, ...flattenJson(obj[key], newKey) };
      } else {
        result[newKey] = obj[key];
      }
    }
  }
  return result;
};

// adds a % before all { inside strings if they are not already there
function addPercentToStrings(record) {
  for (const key in record) {
    if (typeof record[key] === "string") {
      record[key] = record[key].replaceAll(/(?<!%)\{/g, "%{");
    }
  }
}

function addVersion(record) {
  record["_version"] = 1;
}

// Get the directory the script is located in
const dir = path.dirname(process.argv[1]);

console.log("dirname", dir);

// Read all files in the directory
const filePaths = fs
  .readdirSync(dir)
  .filter((file) => file.endsWith(".json"))
  .map((file) => path.join(dir, file));

for (const filePath of filePaths) {
  const readFile = fs.readFileSync(filePath, "utf-8");

  const data = JSON.parse(readFile);
  const flattenedData = flattenJson(data);

  addPercentToStrings(flattenedData);
  addVersion(flattenedData);

  const fileToWrite = JSON.stringify(flattenedData, null, 2);
  fs.writeFileSync(filePath, fileToWrite);
}
