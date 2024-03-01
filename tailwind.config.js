/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./ui/*.html", "./ui/js/*.js", "./ui/css/*.css"],
  theme: {
    extend: {},
  },
  plugins: [
    require('@tailwindcss/forms'),
  ],
}

