<template>
  <div id="pdf">
    <pdf src="./Rechnung_Pallas_Kurs.pdf"></pdf>
  </div>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import pdf from 'vue-pdf';

@Component({
  components: {
    pdf,
  },
})
export default class HelloWorld extends Vue {
  @Prop() private msg!: string;
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="scss">
div#pdf {
  border: 1px solid black;
  max-height: 100%;
  overflow: scroll;
}
</style>
