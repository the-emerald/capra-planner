import {segmentType} from "@/common/segment_type";
<template>
    <b-container class="float-left">
        <b-row>
            <b-col class="col-3">
                <b-card-group>
                    <b-card no-body header="Bottom segments">
                        <b-list-group flush class="overflow-auto dive_param">
                            <!--Placeholder group items-->
                            <b-list-group-item v-for="(segment, index) in bottom_segments" :key="`segment-${index}`">
                                <b-form-checkbox>{{segment[0].startDepth}} - ({{segment[1].o2}}/{{segment[1].he}})</b-form-checkbox>
                            </b-list-group-item>
                            <b-list-group-item>
                                <b-form-checkbox>123m - 45:67 - 89/10</b-form-checkbox>
                            </b-list-group-item>
                            <b-list-group-item>
                                <b-form-checkbox>123m - 45:67 - 89/10</b-form-checkbox>
                            </b-list-group-item>
                        </b-list-group>
                    </b-card>
                </b-card-group>
            </b-col>
            <b-col class="col-3">
                <b-card-group>
                    <b-card no-body header="Deco gases">
                        <b-list-group flush class="overflow-auto dive_param">
                            <!--Placeholder group items-->
                            <b-list-group-item>
                                <b-form-checkbox>12/56</b-form-checkbox>
                            </b-list-group-item>
                            <b-list-group-item>
                                <b-form-checkbox>87/11</b-form-checkbox>
                            </b-list-group-item>
                            <b-list-group-item>
                                <b-form-checkbox>55/24</b-form-checkbox>
                            </b-list-group-item>
                            <b-list-group-item>
                                <b-form-checkbox>27/84</b-form-checkbox>
                            </b-list-group-item>
                            <b-list-group-item>
                                <b-form-checkbox>75/0</b-form-checkbox>
                            </b-list-group-item>
                            <b-list-group-item>
                                <b-form-checkbox>100/0</b-form-checkbox>
                            </b-list-group-item>
                        </b-list-group>
                    </b-card>
                </b-card-group>
            </b-col>
            <b-col>
                <p>Reserved for showing dive results</p>
                <b-button @click="exampleBottomSegment">Click me</b-button>
            </b-col>
        </b-row>

    </b-container>
</template>

<script lang="ts">
    import {Component, Vue} from "vue-property-decorator";
    import {diveSegment} from "@/common/dive_segment";
    import {gas} from "@/common/gas";
    import {segmentType} from "@/common/segment_type";

    @Component
    export default class Parameters extends Vue {
        bottom_segments: Array<[diveSegment, gas]> = [];
        depth = NaN; // Default value should be hidden! Can't find a better way.

        exampleBottomSegment() {
            const exSeg: diveSegment = {
                segmentType: segmentType.DiveSegment,
                startDepth: 35,
                endDepth: 35,
                time: 600_000,
                ascentRate: -10,
                descentRate: 20
            };

            const exGas: gas =  {
                he: 0,
                o2: 21

            };
            this.bottom_segments.push([exSeg, exGas]);
        }
    }
</script>

<style scoped>
    .dive_param {
        height: 75vh;
        overflow-y: auto;
    }

</style>