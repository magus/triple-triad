export function preview(list) {
  console.log(list.length, 'items');
  console.log(list[0]);
  console.log(list[list.length - 1]);
}

export function to_map(keyFn, list) {
  const map = {};

  for (const item of list) {
    map[keyFn(item)] = item;
  }

  return map;
}
