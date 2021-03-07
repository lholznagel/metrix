<template>
  <div>
    <img style="width: 100%; height: 100%" svg-inline src="../assets/metric_market.svg" alt="example" />
  </div>
</template>

<script lang="ts">
import axios from 'axios';
import { Component, Vue } from 'vue-property-decorator';
import { IMetricValue } from './Dashboard.vue';

const METRICS = [
  "caph_collector::market::time::complete",
  "caph_collector::market::time::fetch_region", // TODO: rename to region::fetch
  "caph_collector::market::time::market_data::fetch",
  "caph_collector::market::time::prep::order_id",
  "caph_collector::market::time::prep::market_info",
  "caph_collector::market::time::send::market_info",
  "caph_collector::market::time::prep::market_data",
  "caph_collector::market::time::send::market_data",
  "caph_collector::market::time::market_data::insert",
];

@Component
export default class Home extends Vue {
  public values: { [key: string]: IMetricValue } = {};

  public async mounted() {
    for (const metric of METRICS) {
      const doc = document.getElementById(metric);
      if (doc) {
        const metric_value = await this.fetch_metric(metric);
        doc.textContent = metric_value.value.toString();
      }
    }
  }

  public async fetch_metric(key: string): Promise<IMetricValue> { 
    const id = (await axios.post('/api/metrics', JSON.stringify(key), { headers: { 'Content-Type': 'application/json'}})).data

    const result = (await axios.get(`/api/history/${id}`)).data;
    result.timestamp = new Date(Math.floor(result.timestamp / 1000000));
    return result;
  }
}
</script>
