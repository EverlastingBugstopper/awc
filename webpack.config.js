const path = require("path");

module.exports = {
  mode: 'production',
  entry: path.resolve(__dirname, "src/ui/index.ts"),
  output: {
    path: path.resolve(__dirname, "public"),
    filename: "index.js"
  },
  devtool: "source-map",
  module: {
    rules: [
      {
        test: /\.ts$/,
        exclude: /(node_modules)/,
        use: {
          // `.swcrc` can be used to configure swc
          loader: "swc-loader"
        }
      },
      {
        test: /\.ts$/,
        exclude: /(node_modules)/,
        enforce: "pre",
        use: ["source-map-loader"],
      },
    ]
  }
}