const resolve = require("path").resolve;
const { createThemes } = require("tw-colors");

module.exports = {
  content: [
    resolve(__dirname, "index.html"),
    resolve(__dirname, "src/**/*.{vue,ts}"),
  ],
  theme: {
    extend: {
      colors: {
        "gitstatus-new": "#22a81b",
        "gitstatus-modified": "#b57219",
        "gitstatus-deleted": "#bf1b1b",
        "gitstatus-unknown": "#9ca3af",
      },
    },
  },
  plugins: [
    createThemes({
      "github-dimmed": {
        primary: "#f33d60",
        background: "#22272e",
        "text-area-background": "#4c4653",
        text: "white",
        "text-hover": "#a3a3a3",
        "red-deletion": "#cb2431",
        "green-addition": "#22863a",
      },
      "github-light": {
        primary: "#f33d60",
        background: "#e7e5e4",
        "text-area-background": "#f6f8fa",
        text: "#22272e",
        "text-hover": "#404040",
        "red-deletion": "#b91c1c",
        "green-addition": "#166534",
      },
    }),
  ],
};
