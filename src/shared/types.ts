export type Replace<T, U extends PropertyKey, V> = Omit<T, U> & {
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
