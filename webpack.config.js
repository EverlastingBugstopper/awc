const path = require("path");

const BASE = path.resolve(__dirname, "crates/awc-server");

module.exports = {
  mode: "production",
  entry: path.resolve(BASE, "ui/index.ts"),
  output: {
    path: path.resolve(BASE, "public"),
    filename: "index.js",
  },
  devtool: "source-map",
  module: {
    rules: [
      {
        test: /\.ts$/,
        exclude: /(node_modules)/,
        use: {
          // `.swcrc` can be used to configure swc
          loader: "swc-loader",
        },
      },
      {
        test: /\.ts$/,
        exclude: /(node_modules)/,
        enforce: "pre",
        use: ["source-map-loader"],
      },
    ],
  },
};
