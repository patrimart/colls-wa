export interface OrderedSet {
  _getId_(): number;
}

export interface OrderedSetItem<Item> {
  readonly data: Item;
  readonly internalItem: [
    number, // ID
    ReadonlyArray<ReadonlyArray<number>>, // equals
    ReadonlyArray<ReadonlyArray<number>> // compares
  ];
}

export type EqualityFn<Item> = (
  item: Item
) => ReadonlyArray<ReadonlyArray<number>>;

export type SortFn = (
  val: string | number | boolean | Date | undefined | null
) => number[];
