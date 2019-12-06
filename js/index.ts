import { orderedSetFactory } from "./orderedSet/factory";

const collections = import("../pkg/collections_wa").then(module => {
  const createOrderedSet = orderedSetFactory(module);
  return { createOrderedSet };
});

export * from "./orderedSet/index";
export default collections;
