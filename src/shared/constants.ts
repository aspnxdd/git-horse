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

export const githubDimmedTheme = {
  bg: "#24292e",
  language: "#f47067",
  functionColor: "#dcbdfb",
  variable: "#6cb6ff",
  meta: "#96d0ff",
  symbol: "#f69d50",
  formula: "#768390",
  selector: "#8ddb8c",
  subst: "#adbac7",
  section: "#316dca",
  bullet: "#eac55f",
  emphasis: "#adbac7",
  strong: "#adbac7",
  addition: "#b4f1b4",
  deletion: "#ffd8d3",
  additionBg: "#f0fff4",
  deletionBg: "#ffeef0",
};

export const githubLightTheme = {
  bg: "#fff",
  language: "#b8868b",
  functionColor: "#6f42c1",
  variable: "#005cc5",
  meta: "#032f62",
  symbol: "#e36209",
  formula: "#6a737d",
  selector: "#22863a",
  subst: "#24292e",
  section: "#005cc5",
  bullet: "#735c0f",
  emphasis: "#24292e",
  strong: "#24292e",
  addition: "#22863a",
  deletion: "#b31d28",
  additionBg: "#f0fff4",
  deletionBg: "#ffeef0",
};

export type Theme = typeof githubDimmedTheme;
