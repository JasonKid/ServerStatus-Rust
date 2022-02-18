const path = require("path");
const { VueLoaderPlugin } = require("vue-loader")

module.exports = {
    mode: "development",
    entry: "./src",
    output: {
      path: path.resolve(__dirname, "wwwroot"),
      filename: "index.js"
    },
    module: {
        rules: [
          {
            test: /\.vue$/,
            loader: "vue-loader"
          },
          {
            test: /\.m?js$/,
            exclude: /(node_modules|bower_components)/,
            use: {
              loader: "babel-loader",
              options: {
                presets: ["@babel/preset-env"],
                plugins: [["babel-plugin-import", {
                    libraryName: "ant-design-vue",
                    libraryDirectory: "es",
                    style: "css"
                }]]
              }
            }
          },
          {
            test: /\.css$/,
            use: ["vue-style-loader", {
              loader: "css-loader", 
              options: {esModule: false}
              }]
          },
          {
            test: /\.less$/,
            use: ["vue-style-loader", {
              loader: "css-loader", 
              options: {esModule: false}
              },
            "less-loader"]
          },
        ]
      },
      plugins: [new VueLoaderPlugin()],
      resolve: {
        extensions: [".vue", ".js", ".css", ".ts"]
      },
      devServer: {
        // compress: true,
        port: 9000,
        static: {
          directory: path.join(__dirname, 'wwwroot'),
        },
      },
}