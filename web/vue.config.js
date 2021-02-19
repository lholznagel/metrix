module.exports = {
  "transpileDependencies": [
    "vuetify"
  ],
  devServer: {
    port: 1338,
    proxy: {
      '/api': {
        target: 'http://virgo:8889'
      }
    }
  }
}
