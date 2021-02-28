<template>
  <div>{{ data }}</div>
</template>

<script lang="ts">
import axios from 'axios';
import { Component, Vue } from 'vue-property-decorator';

@Component
export default class Home extends Vue {
  public data = [];

  public async created() {
    this.data = (await axios.get('/api/history/25284ac0-2a9b-4737-a4e0-785c6510527d/raw')).data;
  }
}
</script>
