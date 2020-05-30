import {segmentType} from "@/common/segment_type";
<template>
    <div>
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
    </div>
</template>

<script lang="ts">
    import {Component, Vue} from "vue-property-decorator";
    import {diveSegment} from "@/common/dive_segment";
    import {gas} from "@/common/gas";

    import {namespace} from 'vuex-class';
    const plan = namespace('plan');

    @Component
    export default class BottomSegments extends Vue {
        @plan.State
        public bottomSegments!: Array<[diveSegment, gas]>;

        @plan.Action
        public updateBottomSegments!: (updated: Array<[diveSegment, gas]>) => void
    }
</script>

<style scoped>
    .dive_param {
        height: 75vh;
        overflow-y: auto;
    }

</style>