/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./packages/**/*.{html,rs}"],
  theme: {
    extend: {},
  },
  plugins: [require("@tailwindcss/typography"), require("daisyui")],
};
