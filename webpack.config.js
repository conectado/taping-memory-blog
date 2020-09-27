const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const OptimizeCSSAssetsPlugin = require('optimize-css-assets-webpack-plugin');
const TerserJSPlugin = require('terser-webpack-plugin');

const distPath = path.resolve(__dirname, "dist");
const clientPath = path.resolve(__dirname, "client");
const staticPath = path.resolve(clientPath, "static");
module.exports = (env, argv) => {
  return {
    entry: path.resolve(staticPath, "bootstrap.js"),
    optimization: {
      minimizer: [new TerserJSPlugin({}), new OptimizeCSSAssetsPlugin({})],
    },
    output: {
      path: distPath,
      filename: "blog.js",
      webassemblyModuleFilename: "client.wasm"
    },
    plugins: [
      new CopyWebpackPlugin([
        { from: staticPath, to: distPath }
      ]),
      new WasmPackPlugin({
        crateDirectory: "client",
        extraArgs: "--no-typescript",
      }),
      new MiniCssExtractPlugin({
        filename: '[name].css',
        chunkFilename: '[id].css',
      }),
    ],
    module: {
      rules: [
        {
          test: /\.css$/,
          use: [MiniCssExtractPlugin.loader, 'css-loader'],
        },
      ],
    },
  };
};
