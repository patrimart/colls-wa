const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = {
  mode: "production",
  entry: {
    index: "./js/index.ts"
  },
  output: {
    library: "collections",
    libraryTarget: "umd",
    globalObject: "this",
    path: dist,
    filename: "[name].js"
  },
  devServer: {
    contentBase: dist
  },
  resolve: {
    // Add `.ts` and `.tsx` as a resolvable extension.
    extensions: [".ts", ".js"]
  },
  module: {
    rules: [
      // all files with a `.ts` or `.tsx` extension will be handled by `ts-loader`
      { test: /\.tsx?$/, loader: "ts-loader" }
    ]
  },
  plugins: [
    new WasmPackPlugin({
      crateDirectory: __dirname,
    })
  ]
};
