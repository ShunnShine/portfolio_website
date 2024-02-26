/** @type {import('tailwindcss').Config} */
module.exports = {
    content: { 
      files: ["*.html", "./src/**/*.rs"],
    },
    theme: {
      colors: {
        '60': '#12A64F',
        '30': '#A61E13',
        '10': '#512724',
      }
    },
    plugins: [],
  }