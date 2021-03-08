import App from './App.vue';
import ECharts from 'vue-echarts';
import router from './router';
import Vue from 'vue';
import VueCompositionAPI from '@vue/composition-api';
import vuetify from './plugins/vuetify';
import VueGridLayout from 'vue-grid-layout';

import 'echarts';

Vue.config.productionTip = false;

Vue.use(VueCompositionAPI);
Vue.component('v-grid-layout', VueGridLayout.GridLayout);
Vue.component('v-grid-item', VueGridLayout.GridItem);
Vue.component('v-chart', ECharts);

Vue.component('svg-market-insert', () => import(/* webpackChunkName: "svg_market_insert" */ './components/svg/MarketInsert.vue'));

new Vue({
  router,
  vuetify,
  render: h => h(App)
}).$mount('#app');
