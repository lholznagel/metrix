<template>
  <div>
    <h1>Metrics for Caph Collector</h1>
    <v-grid-layout
      :layout.sync="layout"
      :row-height="30"
      :margin="[7, 3]"
      :prevent-collision="true"
      :is-resizable="false"
      :is-draggable="true"
      :vertical-compact="false"
    >
      <v-grid-item
        v-for="item in layout"
        :x="item.x"
        :y="item.y"
        :w="item.w"
        :h="item.h"
        :i="item.i"
        :min-h="item.h"
        :key="item.i">

        <v-card style="heigth: 100%">
          <v-card-title>
            <v-row align="center" justify="center">
              {{ item.title }}
            </v-row>
          </v-card-title>

          <v-skeleton-loader
            class="mx-auto"
            type="card"
            v-if="!values[item.mid]"
          ></v-skeleton-loader>

          <v-card-text class="pt-2" v-if="item.wType === 'COUNTER' && values[item.mid]">
            <v-row align="center" justify="center">
              <h1 v-if="values[item.mid].value > 5" class="text-h1" style="color: #4CAF50">{{ values[item.mid].value }}</h1>
              <h1 v-if="values[item.mid].value <= 5" class="text-h1" style="color: #F44336">{{ values[item.mid].value }}</h1>
            </v-row>
            <v-row align="center" justify="center">
              <h5>{{ values[item.mid].timestamp.toLocaleString("de-DE") }}</h5>
            </v-row>
          </v-card-text>

          <v-card-text class="pt-2" v-if="item.wType === 'DURATION' && values[item.mid]">
            <v-row align="center" justify="center">
              <h1 v-if="values[item.mid].value / 1000000 < 10000" class="text-h1">{{ Math.floor(values[item.mid].value / 1000000) }}ms</h1>
              <h1 v-else class="text-h1">{{ Math.floor(values[item.mid].value / 1000000000) }}s</h1>
            </v-row>
            <v-row align="center" justify="center">
              <h5>{{ values[item.mid].timestamp.toLocaleString("de-DE") }}</h5>
            </v-row>
          </v-card-text>
        </v-card>

      </v-grid-item>
    </v-grid-layout>
  </div>
</template>

<script lang="ts">
import axios from 'axios';
import { Component, Vue } from 'vue-property-decorator';

@Component
export default class Home extends Vue {
  public values: { [key: string]: IMetricValue } = {};
  public layout: IDashboardItem[] = [
    { x: 0, y: 1, w: 3, h: 5, i: '0', wType: 'COUNTER', mid: 'c3ba9030-8b50-4f31-bc01-ecb141df9349', title: 'Available connections' },
    { x: 3, y: 1, w: 3, h: 5, i: '1', wType: 'COUNTER', mid: '0611e2eb-ae9f-4905-bd8a-f9b51ec70034', title: 'Broken connections' },
    { x: 0, y: 6, w: 3, h: 5, i: '2', wType: 'DURATION', mid: 'b2fbdffa-acca-4220-9949-577cc55e9fc0', title: 'Market fetch (eve)' },
    { x: 3, y: 6, w: 3, h: 5, i: '3', wType: 'DURATION', mid: '7d3d620b-a128-40a5-aa76-578b2268443f', title: 'Insert db' },
    { x: 6, y: 6, w: 3, h: 5, i: '4', wType: 'DURATION', mid: '7747ce1c-d5b1-4833-9093-72ed6a9c26eb', title: 'Insert total' },
    { x: 0, y: 11, w: 3, h: 5, i: '5', wType: 'DURATION', mid: '7aa54712-1b90-4fed-ac23-3b6047afe6ac', title: 'Prep market infos' },
    { x: 3, y: 11, w: 3, h: 5, i: '6', wType: 'DURATION', mid: 'a8b2a39c-c3a1-4bcb-b529-28ff7e9bfe8b', title: 'Send market infos' },
    { x: 6, y: 11, w: 3, h: 5, i: '7', wType: 'DURATION', mid: 'a5d37bf2-7a3c-4d68-b2cf-573b44d23900', title: 'Prep market data' },
    { x: 9, y: 11, w: 3, h: 5, i: '8', wType: 'DURATION', mid: '0efb2027-263e-444d-b241-f5058f827ba0', title: 'Send market data' }
  ];

  public async created() {
    for (const x of this.layout) {
        const result = (await axios.get(`/api/history/${x.mid}`)).data;
        result.timestamp = new Date(Math.floor(result.timestamp / 1000000));
        Vue.set(this.values, x.mid, result);
    }
  }
}

export interface IDashboardItem {
  x: number;
  y: number;
  h: number;
  w: number;
  i: string;
  wType: 'DURATION' | 'COUNTER';
  mid: string;
  title: string;
}

export interface IMetricValue {
  value: number;
  timestamp: number;
}
</script>
