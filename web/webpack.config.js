const path = require("path");
const { VueLoaderPlugin } = require("vue-loader")
const { VuetifyLoaderPlugin } = require('vuetify-loader')

module.exports = {
    mode: "production",
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
          {
            test: /\.sass$/,
            use: ['vue-style-loader', 'css-loader', 'sass-loader']
          }
        ]
      },
      plugins: [new VueLoaderPlugin(), new VuetifyLoaderPlugin({ autoImport: true, registerStylesSSR: true })],
      resolve: {
        extensions: [".vue", ".js", ".css", ".ts", "less", "sass"]
      },
      devServer: {
        // compress: true,
        port: 9000,
        static: {
          directory: path.join(__dirname, 'wwwroot'),
        },
        proxy: {
          "/json/stats.json": {
            target: "https://tz-rust.vercel.app",
            secure: false,
            changeOrigin: true
          }
        }
      },
}