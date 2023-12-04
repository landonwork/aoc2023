/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
	  "./assets/static/html/**/*.html",
	  "./assets/layouts/**/*.html",
	  "./assets/templates/**/*.html"
  ],
  theme: {
    extend: {
		colors: {
			my_color: '#4dcb7a',
		},
		fontSize: {
			my_size: '1rem',
		}
	},
  },
  plugins: [],
}

