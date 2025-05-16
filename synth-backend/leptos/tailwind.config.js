module.exports = {
	// ...other settings
	plugins: [
		require("@catppuccin/tailwindcss")({
			// which flavour of colours to use by default, in the `:root`
			defaultFlavour: "mocha",
		}),
	],
};
