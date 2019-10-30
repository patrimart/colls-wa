import { orderedSetFactory } from "./orderedSet/factory";

const collections = (async function() {
  const module = await import("../pkg/collections_wa");
  return {
    createOrderedSet: orderedSetFactory(module)
  };
})();

export * from "./orderedSet/index";
export default collections;
