import {segmentType} from "@/common/segment_type";
<template>
    <div>
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
    </div>
</template>

<script lang="ts">
    import {Component, Vue} from "vue-property-decorator";
    import {diveSegment} from "@/common/dive_segment";
    import {gas} from "@/common/gas";

    import {namespace} from 'vuex-class';
    const plan = namespace('plan');

    @Component
    export default class DecoGases extends Vue {
        @plan.State
        public decoGases!: Array<[diveSegment, gas]>;

        @plan.Action
        public updateDecoGases!: (updated: Array<[gas, number?]>) => void
    }
</script>

<style scoped>
    .dive_param {
        height: 75vh;
        overflow-y: auto;
    }

</style>