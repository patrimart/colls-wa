import { orderedSetFactory } from "./orderedSet/factory";

// import * as module from "../pkg/collections_wa";

// const createOrderedSet = orderedSetFactory(module);

// export * from "./orderedSet/index";
// export default { createOrderedSet };

// const createOrderedSet = orderedSetFactory(module);
// console.log({ createOrderedSet });

const collections = import("../pkg/collections_wa").then(module => {
  const createOrderedSet = orderedSetFactory(module);
  return { createOrderedSet };
});

export * from "./orderedSet/index";
export default collections;
