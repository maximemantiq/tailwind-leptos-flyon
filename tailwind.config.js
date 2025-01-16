import twContainerQueries from "@tailwindcss/container-queries";
import twCssAnimate from "tailwindcss-animate";

/** @type {import('tailwindcss').Config} */
module.exports = {
	darkMode: "class",
	content: {
		files: ["*.html", "./src/**/*.rs", "./input.css", "./node_modules/flyonui/dist/js/*.js"],
	},
	theme: {
		container: {
			center: true,
			padding: "2rem",
			screens: {
				"2xl": "1400px",
			},
		},
		extend: {
			fontFamily: {
				opensans: ["Open Sans", "sans-serif"],
				robotomono: ["Roboto Mono", "monospace"],
			},
			colors: {
				myborder: "hsl(var(--myborder))",
				myinput: "hsl(var(--myinput))",
				myring: "hsl(var(--myring))",
				mybackground: "hsl(var(--mybackground))",
				myforeground: "hsl(var(--myforeground))",
				myprimary: {
					DEFAULT: "hsl(var(--myprimary))",
					foreground: "hsl(var(--myprimary-foreground))",
				},
				mysecondary: {
					DEFAULT: "hsl(var(--mysecondary))",
					foreground: "hsl(var(--mysecondary-foreground))",
				},
				mysuccess: {
					DEFAULT: "hsl(var(--mysuccess))",
					foreground: "hsl(var(--mysuccess-foreground))",
				},
				myinfo: {
					DEFAULT: "hsl(var(--myinfo))",
					foreground: "hsl(var(--myinfo-foreground))",
				},
				mywarning: {
					DEFAULT: "hsl(var(--mywarning))",
					foreground: "hsl(var(--mywarning-foreground))",
				},
				myerror: {
					DEFAULT: "hsl(var(--myerror))",
					foreground: "hsl(var(--myerror-foreground))",
				},
				mydestructive: {
					DEFAULT: "hsl(var(--mydestructive))",
					foreground: "hsl(var(--mydestructive-foreground))",
				},
				mymuted: {
					DEFAULT: "hsl(var(--mymuted))",
					foreground: "hsl(var(--mymuted-foreground))",
				},
				myaccent: {
					DEFAULT: "hsl(var(--myaccent))",
					foreground: "hsl(var(--myaccent-foreground))",
				},
				mypopover: {
					DEFAULT: "hsl(var(--mypopover))",
					foreground: "hsl(var(--mypopover-foreground))",
				},
				mycard: {
					DEFAULT: "hsl(var(--mycard))",
					foreground: "hsl(var(--mycard-foreground))",
				},
			},
			borderRadius: {
				lg: "var(--myradius)",
				md: "calc(var(--myradius) - 2px)",
				sm: "calc(var(--myradius) - 4px)",
			},
		},
	},
	plugins: [
		twCssAnimate,
		twContainerQueries,
		require("flyonui"),
		require("flyonui/plugin") // Require only if you want to use FlyonUI JS component
	],
};
