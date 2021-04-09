<template>
  <b-container id="content">
    <b-row>
      <b-col>
        <b-form>
          <label for="example-datepicker">Choose a date</label>
          <b-form-datepicker
            id="example-datepicker"
            v-model="invoice.date"
            class="mb-2"
          ></b-form-datepicker>
          <b-form-group
            id="fieldset-1"
            description="Let us know your name."
            label="Enter your name"
            label-for="input-1"
            valid-feedback="Thank you!"
            invalid-feedback="Please specify a title."
            :state="invoice.title.length > 0"
          >
            <b-form-input
              id="input-1"
              v-model="invoice.title"
              :state="invoice.title.length > 0"
              trim
            ></b-form-input>
          </b-form-group>
          <b-container v-for="position in invoice.positions" :key="position">
            <b-form-group
              id="fieldset-1"
              description="Let us know your name."
              label="Enter your name"
              label-for="input-1"
              valid-feedback="Thank you!"
              invalid-feedback="Please specify a position description."
              :state="position.text.length > 0"
            >
              <b-form-input
                id="input-1"
                v-model="position.text"
                :state="position.text.length > 0"
                trim
              ></b-form-input>
            </b-form-group>
          </b-container>
        </b-form>
      </b-col>
      <b-col>
        <div id="pdf">
          <pdf :src="previewUrl"></pdf>
        </div>
      </b-col>
    </b-row>
  </b-container>
</template>

<script lang="ts">
import pdf from 'vue-pdf';
import { Component, Vue } from 'vue-property-decorator';

@Component({
  components: {
    pdf,
  },
  data() {
    const now = new Date();
    const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());

    return {
      previewUrl: '',
      invoice: {
        date: today,
        title: 'TATA',
        positions: [
          {
            text: 'Test',
            count: 42,
            cost: 120,
            curency: 'CHF',
            vat_included: true,
            vat_must: true,
          },
        ],
      },
    };
  },
  watch: {
    invoice: {
      handler(newValue) {
        console.log(JSON.stringify(this.$data.invoice));
        console.log(btoa(JSON.stringify(this.$data.invoice)));
        this.$data.previewUrl = `http://localhost:8000/get/${btoa(
          JSON.stringify(this.$data.invoice),
        )}`;
        console.log(this.$data.previewUrl);
      },
      deep: true,
    },
  },
})
export default class Home extends Vue {}
</script>

<style socped lang="scss">
#content {
  height: calc(100% - 84px);
}

.row {
  height: 100%;
}

.col {
  height: 100%;
}

div#pdf {
  border: 1px solid black;
  max-height: 100%;
  overflow: scroll;
}
</style>
