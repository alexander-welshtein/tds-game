const path = require("path")
const HtmlWebpackPlugin = require("html-webpack-plugin")
const CopyWebpackPlugin = require('copy-webpack-plugin')

module.exports = {
    entry: "./public/src/index.ts",
    output: {
        path: path.resolve(__dirname, "public/dist"),
        filename: "bundle.js"
    },
    resolve: {
        extensions: [".ts", ".js"],
        alias: {
            assets: path.join(__dirname, "public/assets")
        }
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: "./public/src/index.html"
        }),
        new CopyWebpackPlugin({
            patterns: [
                { from: 'public/assets' }
            ]
        })
    ],
    module: {
        rules: [
            {
                test: /\.ts$/,
                use: "ts-loader",
                exclude: /node_modules/
            }
        ]
    }
}