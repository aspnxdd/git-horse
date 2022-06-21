export enum GitStatus {
  Modified = "Modified",
  Unknown = "Unknown",
  Deleted = "Deleted",
  New = "New"
}

export type Replace<T, U extends string | number | symbol, V> = Omit<T, U> & {
  [K in U]: V;
};
