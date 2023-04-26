const resolve = require("path").resolve;
const { createThemes } = require("tw-colors");

module.exports = {
  content: [
    resolve(__dirname, "index.html"),
    resolve(__dirname, "src/**/*.{vue,ts}"),
  ],
  theme: {
    extend: {},
  },
  plugins: [
    createThemes({
      "github-dimmed": {
        primary: "#f33d60",
        background: "#22272e",
        "text-area-background": "#4c4653",
        text: "white",
        "text-hover": "#a3a3a3",
      },
      "github-light": {
        primary: "#f33d60",
        background: "#e7e5e4",
        "text-area-background": "#f6f8fa",
        text: "#22272e",
        "text-hover": "#404040",
      },
    }),
  ],
};
