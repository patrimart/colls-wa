import * as module from "../pkg/collections_wa";
import { orderedSetFactory } from "./orderedSet/factory";

const createOrderedSet = orderedSetFactory(module);

export { createOrderedSet };
