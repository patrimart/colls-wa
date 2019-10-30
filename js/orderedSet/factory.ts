import * as Module from "../../pkg/collections_wa";

let ID = 0;

export interface OrderedSet {
  _getId_(): number;
}

export interface OrderedSetItem<Item> {
  readonly data: Item;
  readonly internalItem: {
    readonly id: number;
    readonly equals: ReadonlyArray<ReadonlyArray<number>>;
    readonly compares: ReadonlyArray<ReadonlyArray<number>>;
  };
}

type EqualityFn<Item> = (item: Item) => ReadonlyArray<ReadonlyArray<number>>;

type SortFn = (
  val: string | number | boolean | Date | undefined | null
) => number[];

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

const buildSetItem = <Item>(
  equals: EqualityFn<Item>,
  compares: EqualityFn<Item>
) => (data: Item): OrderedSetItem<Item> => ({
  data,
  internalItem: {
    id: ID++,
    equals: equals(data),
    compares: compares(data)
  }
});

export const orderedSetFactory = (module: typeof Module) => {
  /**
   *
   */
  return function createOrderedSet<Item>(
    equals: EqualityFn<Item>,
    compares: EqualityFn<Item>,
    items: ReadonlyArray<Item> = []
  ) {
    const _items = new Map<string, OrderedSetItem<Item> | undefined>();
    const _buildSetItem = buildSetItem<Item>(equals, compares);
    const _buildSetItems = (items: ReadonlyArray<Item>) => {
      const setItems: OrderedSetItem<Item>["internalItem"][] = [];
      for (let i = 0; i < items.length; i++) {
        const item = _buildSetItem(items[i]);
        setItems[i] = item.internalItem;
        _items.set(_bigIntToStr(item.internalItem.id), item);
      }
      return setItems;
    };
    const _bigIntToStr = (v: number | BigInt | string) => String(v);
    const _bigIntToNum = (v: number | BigInt | string) => parseInt(String(v));
    const _getItemById = (id: number | BigInt | string): Item | undefined => {
      const nId = _bigIntToStr(id);
      if (nId !== "") {
        const item = _items.get(nId);
        return item && item.data;
      }
    };
    const _mapIdsToItems = (ids: Uint32Array): ReadonlyArray<Item> => {
      const items: Item[] = [];
      let index = 0;
      for (let i = 0; i < ids.length; i++) {
        const id = ids[i];
        const item = _getItemById(id);
        if (item) {
          items[index++] = item;
        }
      }
      return items;
    };

    const _ID_ = module.ordered_set_new(_buildSetItems(items));

    const _getId_ = () => _ID_;

    /**
     *
     */
    const add = (items: ReadonlyArray<Item>): void => {
      module.ordered_set_add(_ID_, _buildSetItems(items));
    };

    /**
     *
     */
    const getAt = (index: number): Item | undefined => {
      const id = module.ordered_set_get_at(_ID_, index);
      return _getItemById(id);
    };

    /**
     *
     */
    const contains = (item: Item): boolean =>
      module.ordered_set_contains(_ID_, _buildSetItem(item));

    /**
     *
     */
    const find = (item: Item): number =>
      _bigIntToNum(module.ordered_set_find(_ID_, _buildSetItem(item)));

    /**
     *
     */
    const slice = (start: number, end: number): ReadonlyArray<Item> =>
      _mapIdsToItems(module.ordered_set_slice(_ID_, start, end));

    /**
     *
     */
    const shift = (): Item | undefined => {
      const id = _bigIntToStr(module.ordered_set_shift(_ID_));
      const item = _getItemById(id);
      _items.delete(id);
      return item;
    };

    /**
     *
     */
    const pop = (): Item | undefined => {
      const id = _bigIntToStr(module.ordered_set_pop(_ID_));
      const item = _getItemById(id);
      _items.delete(id);
      return item;
    };

    /**
     *
     */
    const remove = (items: ReadonlyArray<Item>): ReadonlyArray<Item> =>
      _mapIdsToItems(module.ordered_set_remove(_ID_, _buildSetItems(items)));

    /**
     *
     */
    const removeAt = (index: number): Item | undefined => {
      const id = module.ordered_set_remove_at(_ID_, index);
      return _getItemById(id);
    };

    /**
     *
     */
    const size = (): number => module.ordered_set_size(_ID_);

    /**
     *
     */
    const toArray = (): ReadonlyArray<Item> =>
      _mapIdsToItems(module.ordered_set_to_array(_ID_));

    /**
     *
     */
    const clear = (): void => module.ordered_set_clear(_ID_);

    /**
     *
     */
    const union = (other: OrderedSet): ReadonlyArray<Item> =>
      _mapIdsToItems(module.ordered_set_union(_ID_, other._getId_()));

    /**
     *
     */
    const intersection = (other: OrderedSet): ReadonlyArray<Item> =>
      _mapIdsToItems(module.ordered_set_union(_ID_, other._getId_()));

    /**
     *
     */
    const difference = (other: OrderedSet): ReadonlyArray<Item> =>
      _mapIdsToItems(module.ordered_set_union(_ID_, other._getId_()));

    /**
     *
     */
    const destroy = () => module.ordered_set_delete(_ID_);

    return {
      _getId_,
      add,
      getAt,
      contains,
      find,
      slice,
      shift,
      pop,
      remove,
      removeAt,
      size,
      toArray,
      clear,
      union,
      intersection,
      difference,
      destroy
    };
  };
};
