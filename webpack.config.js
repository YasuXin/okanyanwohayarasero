const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = {
  experiments: {
    asyncWebAssembly: true
  },
  module: {
    rules: [
      /*{
        test: /\.wasm$/,
        type: "webassembly/async"
      }*/
    ]
  },
  mode: "production",
  entry: {
    index: "./js/index.js"
  },
  output: {
    path: dist,
    filename: "[name].js"
  },
  devServer: {
    static: {
      directory: dist,
    }
  },
  plugins: [
    new CopyPlugin([
      path.resolve(__dirname, "static")
    ]),

    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),
  ],
  performance: {
    maxAssetSize: 500000, // 上限を500KiBに引き上げ
    maxEntrypointSize: 500000,
    hints: 'warning' // または false で警告を完全にオフ
  }
};
