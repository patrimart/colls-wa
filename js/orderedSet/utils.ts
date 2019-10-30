import { SortFn } from "./interfaces";

export const asc: SortFn = val => {
  switch (typeof val) {
    case "undefined":
      return [];
    case "string":
      return val.split("").map(s => s.charCodeAt(0));
    case "number":
      return [val];
    case "boolean":
      return [val ? 1 : -1];
    default:
      return val ? [val.getTime()] : [];
  }
};

export const desc: SortFn = val => asc(val).map(i => -i);
