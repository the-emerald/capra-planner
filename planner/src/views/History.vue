<template>
    <div class="history">
        <Navbar></Navbar>
        <br>
        <b-container fluid="true" class="ml-3 mr-3">
            <b-row>
<!--              todo: write history components-->
            </b-row>
        </b-container>
    </div>
</template>

<script lang="ts">
    import Navbar from "@/components/Navbar.vue";
    import {Component, Vue} from "vue-property-decorator";

    @Component({
        name: "History",
        components: {
            Navbar,
        }
    })

    export default class History extends Vue {

    }
</script>
