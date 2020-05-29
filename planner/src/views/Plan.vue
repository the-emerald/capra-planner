<template>
    <div class="plan">
        <Navbar></Navbar>
        <br>
        <p>Plan!</p>
    </div>
</template>

<script lang="ts">
    import Navbar from "@/components/Navbar.vue";
    import {Component, Vue} from "vue-property-decorator";
    import Parameters from "@/components/plan/Parameters.vue";

    @Component({
        name: "Plan",
        components: {
            Navbar,
            Parameters
        }
    })

    export default class Plan extends Vue {

    }
</script>
