export const GitStatus = {
  Modified: "Modified",
  Unknown: "Unknown",
  Deleted: "Deleted",
  New: "New",
} as const;

export const GitStatusCodes = {
  256: GitStatus.Modified,
  512: GitStatus.Deleted,
  128: GitStatus.New,
  2: GitStatus.Modified,
  1: GitStatus.New,
};
