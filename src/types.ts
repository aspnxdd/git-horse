export enum GitStatus {
  Modified = "Modified",
  Unknown = "Unknown",
  Removed = "Removed",
  New = "New"
}

export type Replace<T, U extends string | number | symbol, V> = Omit<T, U> & {
  [K in U]: V;
};
