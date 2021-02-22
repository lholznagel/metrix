<template>
  <v-chart
    class="echarts"
    :option="options"
  />
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';

@Component
export default class ECharts extends Vue {
  public options = {
  };

  public mounted() {
    this.options = {
      xAxis: {
        type: 'category',
        data: ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun']
      },
      yAxis: {},
      series: [{
        data: [150, 230, 224, 218, 135, 147, 260],
        type: 'line'
      }]
    };
  }
}
</script>

<style scoped>
figure {
  display: inline-block;
  position: relative;
  margin: 2em auto;
  border: 1px solid rgba(0, 0, 0, 0.1);
  border-radius: 8px;
  box-shadow: 0 0 45px rgba(0, 0, 0, 0.2);
  padding: 1.5em 2em;
  min-width: calc(40vw + 4em);

  .echarts {
    width: 100%;
    width: 40vw;
    min-width: 400px;
    height: 300px;
  }
}
</style>
