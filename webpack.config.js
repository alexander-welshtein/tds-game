const path = require("path")
const HtmlWebpackPlugin = require("html-webpack-plugin")

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
    devServer: {
        contentBase: path.join(__dirname, "dist"),
        compress: true,
        watchContentBase: true,
        progress: true,
        port: 3000,
        hot: true,
        open: true
    },
    plugins: [new HtmlWebpackPlugin({
        template: "./public/src/index.html"
    })],
    module: {
        rules: [
            {
                test: /\.ts$/,
                use: "ts-loader",
                exclude: /node_modules/
            },
            {
                test: /\.(jpg|png)$/,
                loader: "file-loader",
                options: {
                    name: "[name].[ext]"
                },
                exclude: /node_modules/
            }
        ]
    }
}