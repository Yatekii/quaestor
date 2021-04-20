<template>
  <b-container
    fluid
    id="content"
    style="padding-left: 10em; padding-right: 10em"
  >
    <b-row>
      <b-col>
        <b-button class="mb-5" size="sm" variant="outline-success" @click="save"
          >Save</b-button
        >
      </b-col>
    </b-row>
    <b-row>
      <b-col>
        <b-form>
          <b-form-group description="Language" label-for="language">
            <b-form-select
              id="language"
              v-model="invoice.language"
              class="mb-3"
            >
              <b-form-select-option value="en-US">English</b-form-select-option>
              <b-form-select-option value="de-DE">Deutsch</b-form-select-option>
            </b-form-select>
          </b-form-group>

          <b-form-group
            description="Invoice Date"
            label-for="invoice-datepicker"
          >
            <b-form-datepicker
              id="invoice-datepicker"
              v-model="invoice.date"
              class="mb-2"
            ></b-form-datepicker>
          </b-form-group>

          <b-form-group description="Due date" label-for="due-datepicker">
            <b-form-datepicker
              id="due-datepicker"
              v-model="invoice.due"
              class="mb-2"
            ></b-form-datepicker>
          </b-form-group>

          <b-form-group
            description="Invoice Number"
            label-for="invoice-no"
            invalid-feedback="Please specify an invoice number."
            :state="invoice.no.length > 0"
          >
            <b-form-input
              id="invoice-no"
              v-model="invoice.no"
              :state="invoice.no.length > 0"
              trim
            ></b-form-input>
          </b-form-group>

          <b-form-group
            description="Customer Address"
            label-for="customer-address"
            invalid-feedback="Please specify an invoice number."
            :state="invoice.address.length > 0"
          >
            <b-form-textarea
              id="customer-address"
              v-model="invoice.address"
              :state="invoice.address.length > 0"
              trim
              style="height: 7em"
            ></b-form-textarea>
          </b-form-group>

          <b-form-group
            description="Contact Person"
            label-for="contact-person"
            invalid-feedback="Please specify a contact person."
            :state="invoice.contact.length > 0"
          >
            <b-form-input
              id="Contact Person"
              v-model="invoice.contact"
              :state="invoice.contact.length > 0"
              trim
            ></b-form-input>
          </b-form-group>

          <b-form-group
            description="Customer Reference"
            label-for="customer-reference"
          >
            <b-form-input
              id="customer-reference"
              v-model="invoice.reference"
              trim
            ></b-form-input>
          </b-form-group>

          <b-form-group
            description="Text"
            label-for="text"
            invalid-feedback="Please specify an invoice number."
            :state="invoice.text.length > 0"
          >
            <b-form-textarea
              id="text"
              v-model="invoice.text"
              :state="invoice.text.length > 0"
              trim
              style="height: 15em"
            ></b-form-textarea>
          </b-form-group>
          <b-form-group
            description="Currency"
            invalid-feedback="Please specify a currency."
            :state="invoice.currency.length > 0"
          >
            <b-form-input
              v-model="invoice.currency"
              :state="invoice.currency.length > 0"
              trim
            ></b-form-input>
          </b-form-group>

          <b-container
            class="p-0 pt-4 pr-3 mb-4 border border-secondary rounded"
          >
            <draggable
              v-model="invoice.positions"
              group="people"
              @start="drag = true"
              @end="drag = false"
            >
              <b-row v-for="position in invoice.positions" :key="position.id">
                <b-col>
                  <h2 style="float: left">
                    <b-icon icon="grip-vertical"></b-icon>
                  </h2>
                  <b-form-group
                    description="Description"
                    invalid-feedback="Please specify a position description."
                    :state="position.text.length > 0"
                  >
                    <b-form-input
                      v-model="position.text"
                      :state="position.text.length > 0"
                      trim
                    ></b-form-input>
                  </b-form-group>
                </b-col>
                <b-col>
                  <b-form-group
                    description="Count"
                    invalid-feedback="Please select a count > 0."
                    :state="position.count > 0"
                  >
                    <b-form-input
                      v-model="position.count"
                      :state="position.count > 0"
                      trim
                      type="number"
                      number
                    ></b-form-input>
                  </b-form-group>
                </b-col>
                <b-col>
                  <b-form-group
                    description="Cost per piece"
                    invalid-feedback="Please select a cost > 0."
                    :state="position.cost > 0"
                  >
                    <b-form-input
                      v-model="position.cost"
                      :state="position.cost > 0"
                      trim
                      type="number"
                      number
                    ></b-form-input>
                  </b-form-group>
                </b-col>
              </b-row>
            </draggable>
          </b-container>
          <b-button
            class="mb-5"
            size="sm"
            variant="outline-primary"
            @click="addPosition"
            >Add Position</b-button
          >
        </b-form>
      </b-col>
      <b-col class="col-100">
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
import draggable from 'vuedraggable';

@Component({
  components: {
    pdf,
    draggable,
  },
  data() {
    const now = new Date();
    const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());

    setTimeout(() => {
      axios
        .post(`${window.location.host}/generate`, this.$data.invoice, {
          responseType: 'arraybuffer',
        })
        .then((response) => {
          const data = new Uint8Array(response.data);
          this.$data.preview = pdf.createLoadingTask(data);
        });
    }, 1000);

    const due = new Date();
    due.setDate(today.getDate() + 30);

    return {
      preview: undefined,
      invoice: {
        language: 'de-DE',
        date: today.toISOString().split('T')[0],
        due: due.toISOString().split('T')[0],
        title: 'TATA',
        address: 'ABB Schweiz AG\nSAS-01\nPostfach 1946\n5401 Baden',
        no: 'RE19-24',
        contact: 'Noah Hüsser',
        reference: '4500592413',
        text:
          'Sehr geehrte Damen und Herren\n\nVielen Dank für das entgegengebrachte Vertrauen und die Beauftragung mit der Softwareentwicklung. Gemäss Offerte 19-2019 erlauben wir uns, Ihnen die untenstehenden Leistungen in Rechnung zu stellen.',
        positions: [
          {
            id: 0,
            text: 'Test',
            count: 42,
            cost: 120,
            vat_included: false,
            vat_must: true,
          },
        ],
        currency: 'CHF',
      },
    };
  },
  watch: {
    invoice: {
      handler(newValue) {
        // const s = btoa(unescape(encodeURIComponent(JSON.stringify(this.$data.invoice))));
        // this.$data.previewUrl = `http://localhost:8000/get/${s}`;
        axios
          .post(`${window.location.host}/generate`, this.$data.invoice, {
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
  methods: {
    addPosition() {
      this.$data.invoice.positions.push({
        id: this.$data.invoice.positions.length,
        text: 'Test',
        count: 42,
        cost: 120,
        vat_included: false,
        vat_must: true,
      });
    },
    save() {
      axios.post(`${window.location.host}/store`, this.$data.invoice, {
        responseType: 'arraybuffer',
      });
      // .then((response) => ());
    },
  },
})
export default class Home extends Vue {}
</script>

<style socped lang="scss">
#content {
  min-height: calc(100% - 84px);
}

.row {
  min-height: 100%;
}

.col-100 {
  height: 100%;
}

div#pdf {
  border: 1px solid black;
  max-height: 100%;
  overflow: scroll;
}
</style>
