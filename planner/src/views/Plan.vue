<template>
    <div class="plan">
        <Navbar></Navbar>
        <br>
        <b-container class="float-left">
            <b-row>
                <b-col class="col-4">
                    <!--b-list group for dive segments-->
                    <BottomSegments></BottomSegments>
                </b-col>
                <b-col class="col-4">
                    <!--b-list group for deco gases-->
                    <DecoGases></DecoGases>
                </b-col>
                <b-col>
                    <!--reserved for showing dive results-->
                    <p>Reserved for showing dive results</p>
                </b-col>
            </b-row>
        </b-container>
    </div>
</template>

<script lang="ts">
    import Navbar from "@/components/Navbar.vue";
    import {Component, Vue} from "vue-property-decorator";
    import BottomSegments from "@/components/plan/BottomSegments.vue";
    import DecoGases from "@/components/plan/DecoGases.vue";

    @Component({
        name: "Plan",
        components: {
            DecoGases,
            BottomSegments,
            Navbar,

        }
    })

    export default class Plan extends Vue {

    }
</script>
