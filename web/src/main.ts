import App from './App.vue';
import ECharts from 'vue-echarts';
import router from './router';
import Vue from 'vue';
import VueCompositionAPI from '@vue/composition-api';
import vuetify from './plugins/vuetify';

import 'echarts';

Vue.config.productionTip = false;

Vue.use(VueCompositionAPI);
Vue.component('v-chart', ECharts);

new Vue({
  router,
  vuetify,
  render: h => h(App)
}).$mount('#app');
