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
};

export const GitStatusColors = {
  [GitStatus.Modified]: "text-[#b57219]",
  [GitStatus.New]: "text-[#22a81b]",
  [GitStatus.Deleted]: "text-[#bf1b1b]",
  [GitStatus.Unknown]: "text-[#575757]",
};
