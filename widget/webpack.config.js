const path = require("path");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");

const devMode = process.env.NODE_ENV !== "production";

module.exports = {
  entry: "./src/index.ts",
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
      {
        test: /\.module\.css$/i,
        use: [
          devMode ? "style-loader" : MiniCssExtractPlugin.loader,
          {
            loader: "css-loader",
            options: {
              modules: {
                namedExport: false, // Enables named exports for TypeScript
                localIdentName: "[name]__[local]___[hash:base64:5]",
              },
              esModule: true, // Ensures correct ES6 module exports
            },
          },
          "postcss-loader",
        ],
      },
    ],
  },
  resolve: {
    extensions: [".tsx", ".ts", ".js"],
  },
  output: {
    filename: "bundle.js",
    path: path.resolve(__dirname, "dist"),
  },
  plugins: [
    ...(devMode ? [] : [new MiniCssExtractPlugin()]), // Conditionally add MiniCssExtractPlugin only in production
  ],
  optimization: {
    usedExports: true, // Helps with tree shaking
  },
  devServer: {
    static: path.resolve(__dirname, "public"),
    hot: true,
    port: 8081,
  },
};
