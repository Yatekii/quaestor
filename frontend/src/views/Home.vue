<template>
  <b-container id="content">
    <b-row>
      <b-col>
        <b-form>
          <label for="language">Choose a date</label>
          <b-form-select id="language" v-model="invoice.language" class="mb-3">
            <b-form-select-option value="en-US">English</b-form-select-option>
            <b-form-select-option value="de-DE">Deutsch</b-form-select-option>
          </b-form-select>
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
          <pdf :src="preview"></pdf>
        </div>
      </b-col>
    </b-row>
    <b-row>
      <b-col>
        <div id="pdf">
          <pdf :src="preview"></pdf>
        </div>
      </b-col>
    </b-row>
  </b-container>
</template>

<script lang="ts">
import pdf from 'vue-pdf';
import { Component, Vue } from 'vue-property-decorator';
import axios from 'axios';

@Component({
  components: {
    pdf,
  },
  data() {
    const now = new Date();
    const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());

    setTimeout(() => {
      axios
        .post('http://localhost:8000/generate', this.$data.invoice, {
          responseType: 'arraybuffer',
        })
        .then((response) => {
          const data = new Uint8Array(response.data);
          console.log(data);
          this.$data.preview = pdf.createLoadingTask(data);
        });
    }, 1000);

    const due = new Date();
    due.setDate(today.getDate() + 30);

    return {
      preview: undefined,
      previewUrl:
        'http://localhost:8000/get/eyJkYXRlIjoiMjAyMS0wNC0wOVQyMjowMDowMC4wMDBaIiwidGl0bGUiOiIxVEFUQTEiLCJhZGRyZXNzIjoiQUJCIFNjaHdlaXogQUdcblNBUy0wMVxuUG9zdGZhY2ggMTk0NlxuNTQwMSBCYWRlblxuIiwibm8iOiJSRTE5LTI0IiwiY29udGFjdCI6Ik5vYWggSMO8c3NlciIsInJlZmVyZW5jZSI6IjQ1MDA1OTI0MTMiLCJ0ZXh0IjoiSWNoIG3DtmNodGUgZGFyYXVmIGhpbndlaXNlbiwgZGFzcyBkZXIgQmVpdHJhZyBhdWNoIGRhbm4gZ2VzY2h1bGRldCBpc3QsIHdlbm4ga2VpbmUgVHJhaW5pbmdzIGJlc3VjaHQgd2VyZGVuLiBFaW4gQXVzdHJpdHQga2FubiBqZXdlaWxzIGJpcyB6dW0gMzEuMTIgZGVzIFZvcmphaHJlcyBzY2hyaWZ0bGljaCBiZWltIFZvcnN0YW5kIGVpbmdlcmVpY2h0IHdlcmRlbiB1bmQgd2lyZCBmw7xyIGRlbiBCZWl0cmFnIGRlcyBGb2xnZWphaHJlcyBiZXLDvGNrc2ljaHRpZ3QuIiwicG9zaXRpb25zIjpbeyJ0ZXh0IjoiVGVzdCIsImNvdW50Ijo0MiwiY29zdCI6MTIwLCJjdXJlbmN5IjoiQ0hGIiwidmF0X2luY2x1ZGVkIjpmYWxzZSwidmF0X211c3QiOnRydWV9XX0=',
      invoice: {
        language: 'de-DE',
        date: today.toISOString().split('T')[0],
        due: due.toISOString().split('T')[0],
        title: 'TATA',
        address: 'ABB Schweiz AG\nSAS-01\nPostfach 1946\n5401 Baden\n',
        no: 'RE19-24',
        contact: 'Noah Hüsser',
        reference: '4500592413',
        text:
          'Ich möchte darauf hinweisen, dass der Beitrag auch dann geschuldet ist, wenn keine Trainings besucht werden. Ein Austritt kann jeweils bis zum 31.12 des Vorjahres schriftlich beim Vorstand eingereicht werden und wird für den Beitrag des Folgejahres berücksichtigt.',
        positions: [
          {
            text: 'Test',
            count: 42,
            cost: 120,
            currency: 'CHF',
            vat_included: false,
            vat_must: true,
          },
        ],
      },
    };
  },
  watch: {
    invoice: {
      handler(newValue) {
        // const s = btoa(unescape(encodeURIComponent(JSON.stringify(this.$data.invoice))));
        // this.$data.previewUrl = `http://localhost:8000/get/${s}`;
        axios
          .post('http://localhost:8000/generate', this.$data.invoice, {
            responseType: 'arraybuffer',
          })
          .then((response) => {
            const data = new Uint8Array(response.data);
            console.log(data);
            this.$data.preview = pdf.createLoadingTask(data);
          });
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
