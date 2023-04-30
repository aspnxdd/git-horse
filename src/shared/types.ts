import { GitStatus } from "./constants";

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

export interface FileStatusWithStatusLabel {
  fileName: string;
  status: keyof typeof GitStatus;
  selected: boolean;
}

export type RepoDiffStats = {
  deletions: number;
  filesChanged: number;
  insertions: number;
};
