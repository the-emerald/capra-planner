<template>
    <div class="history">
        <Navbar></Navbar>
        <br>
        <b-container fluid="true" class="ml-3 mr-3">
              <HistoryMain></HistoryMain>
        </b-container>
    </div>
</template>

<script lang="ts">
    import Navbar from "@/components/Navbar.vue";
    import {Component, Vue} from "vue-property-decorator";
    import HistoryMain from "@/components/history/HistoryMain.vue";

    @Component({
        name: "History",
        components: {
          HistoryMain,
            Navbar,
        }
    })

    export default class History extends Vue {

    }
</script>
