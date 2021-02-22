module.exports = {
  "transpileDependencies": [
    "vuetify"
  ],
  devServer: {
    port: 1338,
    proxy: {
      '/api': {
        target: 'http://192.168.178.199:8889'
      }
    }
  }
}
