<template>
  <div>
    <img style="width: 100%; height: 100%" svg-inline src="../../assets/metric_market.svg" alt="example" />
  </div>
</template>

<script lang="ts">
import axios from 'axios';
import { Component, Vue } from 'vue-property-decorator';
import { IMetricValue } from '../../views/Dashboard.vue';

import * as formatter from './formatter';

const METRICS = [
  "caph_collector::market::time::complete",
  "caph_collector::market::time::region::fetch",
  "caph_collector::market::time::market_data::fetch",
  "caph_collector::market::time::prep::order_id",
  "caph_collector::market::time::prep::market_info",
  "caph_collector::market::time::send::market_info",
  "caph_collector::market::time::prep::market_data",
  "caph_collector::market::time::send::market_data",
  "caph_collector::market::time::market_data::insert",
  "caph_db::fetch::regions::complete",
  "caph_db::insert::market_order_info::complete",
  "caph_db::insert::market_order::current::complete",
];

@Component
export default class SvgMarketInsert extends Vue {
  public values: { [key: string]: IMetricValue } = {};

  public async mounted() {
    for (const metricName of METRICS) {
      const doc = document.getElementById(metricName);
      if (doc) {
        const metric = await this.fetch_metric(metricName);
        doc.textContent = formatter.nanoSeconds(metric.value);
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
