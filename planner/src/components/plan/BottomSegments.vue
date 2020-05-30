import {segmentType} from "@/common/segment_type";
<template>
    <div>
        <b-card-group>
            <b-card no-body>
                <b-card-header>
                    <b-container class="p-0">
                        <b-row no-gutters>
                            <b-col sm="8" style="margin-top: 0.2rem">
                                Bottom segments
                            </b-col>
                            <b-col>
                                <b-button block
                                class="float-right" id="show-bottom-segment-modal-btn"
                                @click="$bvModal.show('bottom-segment-modal')"
                                size="sm">
                                    <b-icon-plus></b-icon-plus>
                                </b-button>
                            </b-col>
                        </b-row>
                    </b-container>
                    <BSModal @submitted="onBSSubmitted"></BSModal>
                </b-card-header>
                <b-list-group flush class="overflow-auto dive_param">
                    <b-list-group-item v-for="(segment, index) in bottomSegments" :key="`segment-${index}`">
                        <b-form-checkbox>{{segment[0].startDepth}} - ({{segment[1].o2}}/{{segment[1].he}})</b-form-checkbox>
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
    import {diveSegment} from "@/common/dive_segment";
    import {gas} from "@/common/gas";

    import {namespace} from 'vuex-class';
    import BSModal from "@/components/plan/BSModal.vue";
    const plan = namespace('Plan');
    @Component({
        components: {BSModal}
    })
    export default class BottomSegments extends Vue {
        @plan.State
        public bottomSegments!: Array<[diveSegment, gas]>;

        @plan.Mutation
        public setBottomSegments!: (to: Array<[diveSegment, gas]>) => void;

        onBSSubmitted(value: [diveSegment, gas]) {
            this.setBottomSegments([value]);
        }
    }


</script>

<style scoped>
    .dive_param {
        height: 75vh;
        overflow-y: auto;
    }

</style>