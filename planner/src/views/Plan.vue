<template>
    <div class="plan">
        <Navbar></Navbar>
        <br>
        <b-container fluid="true" class="ml-3 mr-3">
            <b-row>
                <b-col cols="3">
                    <!--b-list group for dive segments-->
                    <BottomSegments></BottomSegments>
                </b-col>
                <b-col cols="3">
                    <!--b-list group for deco gases-->
                    <DecoGases></DecoGases>
                </b-col>
                <b-col cols="6">
                    <!--reserved for showing dive results-->
                    <Results></Results>
                </b-col>
            </b-row>
        </b-container>
    </div>
</template>

<script lang="ts">
    import Navbar from "@/components/Navbar.vue";
    import {Component, Vue} from "vue-property-decorator";
    import BottomSegments from "@/components/plan/bottom_segments/BottomSegments.vue";
    import DecoGases from "@/components/plan/deco_gases/DecoGases.vue";
    import Results from "@/components/plan/results/Results.vue";

    @Component({
        name: "Plan",
        components: {
            Results,
            DecoGases,
            BottomSegments,
            Navbar,

        }
    })

    export default class Plan extends Vue {

    }
</script>
