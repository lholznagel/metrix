import Vue from 'vue';
import VueRouter, { RouteConfig } from 'vue-router';
import Home from '../views/Home.vue';

Vue.use(VueRouter);

const routes: RouteConfig[] = [
  {
    path: '/',
    redirect: '/all',
  },
  {
    path: '/all',
    name: 'AllMetrics',
    component: () => import(/* webpackChunkName: "all_metrics" */ '../views/AllMetrics.vue')
  },
];

const router = new VueRouter({
  mode: 'history',
  base: process.env.BASE_URL,
  routes
});

export default router;
