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
                Id
              </th>
              <th>
              </th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="v in lastValues" :key="v.id">
              <td>{{ v.key }}</td>
              <td>{{ v.value }}</td>
              <td>{{ new Date(Math.floor(v.timestamp / 1000000)) }}</td>
              <td>{{ v.id }}</td>
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

  public async created() {
    this.metrixInfos = (await axios.get<IMetrixInfo[]>('/api/infos')).data;
    this.load(this.metrixInfos.map(x => x.id));
  }

  public async load(ids: string[]) {
    (await axios.post<IMetrixHistory[]>(`/api/history/bulk`, ids))
      .data
      .map(x => {
        return {
          key: this.getKey(x.id),
          ...x
        }
      })
      .map(x => this.lastValues.push(x));
    this.lastValues = this.lastValues.sort((a, b) => {
      if (a.key < b.key) {
        return -1;
      }
      if (a.key > b.key) {
        return 1;
      }
      return 0;
    })
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
  key: string;
  timestamp: number;
  value: number;
}
</script>
