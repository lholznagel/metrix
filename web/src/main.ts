import Vue from 'vue';
import App from './App.vue';
import router from './router';
import vuetify from './plugins/vuetify';

Vue.config.productionTip = false;

// Vue.component('c-format-number', () => import('@/components/FormatNumber.vue'));

new Vue({
  router,
  vuetify,
  render: h => h(App)
}).$mount('#app');
