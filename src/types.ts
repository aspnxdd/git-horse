export enum GitStatus {
  Modified = "Modified",
  Unknown = "Unknown",
  Deleted = "Deleted",
  New = "New"
}

export type Replace<T, U extends string | number | symbol, V> = Omit<T, U> & {
  [K in U]: V;
};

export interface Repos {
  path: string;
  name: string;
}

export interface GitDiff {
  diffContent: string;
  newLine: number | null;
  oldLine: number | null;
  origin: string;
}

export interface FileStatus {
  fileName: string;
  status: number;
}

export interface TreeItem {
  [key: string]: TreeItem  | boolean | number;
  isFile: boolean;
  depth: number;
}