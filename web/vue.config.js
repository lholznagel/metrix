module.exports = {
  transpileDependencies: [
    "vuetify"
  ],
  devServer: {
    port: 1338,
    proxy: {
      '/api': {
        target: 'http://192.168.178.199:8889'
      }
    }
  },
  chainWebpack: config => {
    config.module
      .rule("vue")
      .use("vue-svg-inline-loader")
        .loader("vue-svg-inline-loader")
        .options({
          svgo: {
            plugins: [
              {
                cleanupIDs: false,
              }
            ]
          }
        });
  }
}
