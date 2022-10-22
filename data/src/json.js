import fs from "fs";

export function read(filepath) {
  const content = fs.readFileSync(filepath);
  const data = JSON.parse(content);
  return data;
}

export function write(filepath, content) {
  fs.writeFileSync(filepath, pretty(content));
}

function pretty(obj) {
  return JSON.stringify(obj, null, 2);
}
