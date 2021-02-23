<template>
  <v-card>
    <v-card-title>All metrix</v-card-title>

    <v-card-text>
      <v-simple-table>
        <template v-slot:default>
          <thead>
            <tr>
              <th>
                Metric
              </th>
              <th>
                Value
              </th>
              <th>
                Timestamp
              </th>
              <th>
              </th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="v in lastValues" :key="v.id">
              <td>{{ getKey(v.id) }}</td>
              <td>{{ v.value }}</td>
              <td>{{ new Date(Math.floor(v.timestamp / 1000000)) }}</td>
              <td>
                <v-btn
                  icon
                  color="blue"
                  :to="{ name: 'MetricHistory', params: { id: v.id } }"
                >
                  <v-icon>mdi-open-in-new</v-icon>
                </v-btn>
              </td>
            </tr>
          </tbody>
        </template>
      </v-simple-table>
    </v-card-text>
  </v-card>
</template>

<script lang="ts">
import axios from 'axios';
import { Component, Vue } from 'vue-property-decorator';

@Component
export default class AllMetrics extends Vue {
  public lastValues: IMetrixHistory[] = [];
  public metrixInfos: IMetrixInfo[] = [];
  public options = {};

  public async created() {
    this.metrixInfos = (await axios.get<IMetrixInfo[]>('/api/infos')).data;
    this.load(this.metrixInfos.map(x => x.id));

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

  public async load(ids: string[]) {
    (await axios.post<IMetrixHistory[]>(`/api/history/bulk`, ids))
      .data
      .map(x => this.lastValues.push(x));
  }

  public getKey(id: string): string {
    return (this.metrixInfos.find(x => x.id === id) || {key: ''}).key;
  }
}

interface IMetrixInfo {
  id: string;
  key: string;
}

interface IMetrixHistory {
  id?: string;
  timestamp: number;
  value: number;
}
</script>
