<template>
  <div>
    <b-row class="mb-3 mt-4 text-center">
      <b-col></b-col>
      <b-form inline @submit="onSubmit">
        <b-col>Checkboxes for plan, execute</b-col>
        <b-col>Datetime range (and a disable button)</b-col>
        <b-col>Query</b-col>
      </b-form>
      <b-col></b-col>
    </b-row>
    <b-row>
      <b-col>
        A lot of history goes here
      </b-col>
    </b-row>
  </div>
</template>

<script lang="ts">
  import {Component, Vue} from "vue-property-decorator";

  @Component({
        name: "HistoryMain",
        components: {}
      })

  export default class HistoryMain extends Vue {
    onSubmit() {
      console.log("onSubmit called");
    }

  }
</script>

<style scoped>

</style>