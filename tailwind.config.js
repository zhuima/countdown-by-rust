/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.rs",
    "./src/**/*.html",
    "./src/**/*.css",
    "./dist/**/*.html"
  ],
  theme: {
    extend: {
      colors: {
        'apple-blue': '#007AFF',
        'apple-red': '#FF3B30',
        'apple-gray': {
          50: '#F2F2F7',
          100: '#E5E5EA',
          // ... 其他灰度
        }
      }
    },
  },
  plugins: [],
}
