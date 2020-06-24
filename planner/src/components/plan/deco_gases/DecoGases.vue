import {segmentType} from "@/common/segment_type";
<template>
    <div>
        <b-card-group>
            <b-card no-body header="Deco gases">
                <b-list-group flush class="overflow-auto list_group">
                    <b-list-group-item v-for="(gas, index) in decoGases" :key="`decogas-${index}`">
                        <b-form-checkbox>({{gas.o2}}/{{gas.he}})</b-form-checkbox>
                    </b-list-group-item>
                    <!--Placeholder group items-->
                    <b-list-group-item>
                        <b-form-checkbox>Placeholder</b-form-checkbox>
                    </b-list-group-item>
                </b-list-group>
            </b-card>
        </b-card-group>
    </div>
</template>

<script lang="ts">
    import {Component, Vue} from "vue-property-decorator";
    import {DiveSegment} from "@/common/serde/dive_segment";
    import {Gas} from "@/common/serde/gas";

    import {namespace} from 'vuex-class';
    const plan = namespace('Plan');

    @Component
    export default class DecoGases extends Vue {
        @plan.State
        public decoGases!: Array<[DiveSegment, Gas]>;

        @plan.Mutation
        public pushDecoGas!: (to: [Gas, number?]) => void;

    }
</script>

<style scoped>
    .list_group {
        height: 75vh;
        overflow-y: auto;
    }
</style>