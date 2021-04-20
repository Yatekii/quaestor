<template>
  <div class="about">
    <b-container
      fluid
      id="content"
      style="padding-left: 10em; padding-right: 10em"
    >
      <h1>Invoices</h1>
      <b-row v-for="invoice in invoices" :key="invoice.name">
        <b-col>
          {{ invoice.name }} :
          <a
            v-for="version in invoice.versions"
            :key="version"
            target="_BLANK"
            :href="`${API_URL}/get/${invoice.name}/${version}`"
            >v{{ version }},
          </a>
        </b-col>
      </b-row>
    </b-container>
  </div>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import axios from 'axios';
import { API_URL } from '@/globals';

@Component({
  components: {},
  data() {
    return {
      invoices: [],
    };
  },
  methods: {
    list() {
      axios.get(`${API_URL}/list`).then((response) => {
        this.$data.invoices = response.data.invoices;
      });
    },
  },
})
export default class Home extends Vue {
  created(): void {
    (this as any).list();
  }
}
</script>
