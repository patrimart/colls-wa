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

export type EqualityFn<Item> = (
  item: Item
) => ReadonlyArray<ReadonlyArray<number>>;

export type SortFn = (
  val: string | number | boolean | Date | undefined | null
) => number[];
