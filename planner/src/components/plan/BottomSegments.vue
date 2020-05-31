import {segmentType} from "@/common/segment_type";
<template>
    <div>
        <b-card-group>
            <b-card no-body>
                <b-card-header>
                    <b-container class="p-0">
                        <b-form-row>
                            <b-col sm="6" style="margin-top: 0.2rem">
                                Bottom segments
                            </b-col>
                            <!--UI buttons-->
                            <b-col>
                                <b-button block
                                          class="float-right" id="rearrange"
                                          size="sm"
                                          title="Reorder">
                                    <b-icon-list-ol></b-icon-list-ol>
                                </b-button>
                            </b-col>
                            <b-col>
                                <b-button block
                                          class="float-right" id="clear"
                                          size="sm"
                                          variant="danger"
                                          title="Clear all">
                                <b-icon-backspace></b-icon-backspace>
                                </b-button>
                            </b-col>
                            <b-col>
                                <b-button block
                                        class="float-right" id="show-bottom-segment-modal-btn"
                                        @click="$bvModal.show('bottom-segment-modal')"
                                        variant="info"
                                        size="sm"
                                        title="Add">
                                <b-icon-plus-circle></b-icon-plus-circle>
                                </b-button>
                            </b-col>
                        </b-form-row>
                    </b-container>
                    <!--Relevant modals-->
                    <BSModal @submitted="onBSSubmitted"></BSModal>
                </b-card-header>
                <b-list-group flush class="overflow-auto list_group">
                    <b-list-group-item v-for="(segment, index) in bottomSegments" :key="`segment-${index}`">
                        <b-form-checkbox>{{formatBottomSegment(segment)}}</b-form-checkbox>
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
        public pushBottomSegment!: (elem: [diveSegment, gas]) => void;

        onBSSubmitted(value: [diveSegment, gas]) {
            this.pushBottomSegment(value);
        }

        // Ugly hack
        prettyMilliseconds(millis: number): string {
            const minutes = Math.floor(millis / 60000);
            const seconds = ((millis % 60000) / 1000).toFixed(0);
            return (Number(seconds) == 60 ? (minutes+1) + ":00" : minutes + ":" + (Number(seconds) < 10 ? "0" : "") + seconds);
        }

        formatBottomSegment(segment: [diveSegment, gas]): string {
            return `${segment[0].startDepth}m | ${this.prettyMilliseconds(segment[0].time)} | ${segment[1].o2}/${segment[1].he}`
        }
    }


</script>

<style scoped>
    .list_group {
        height: 75vh;
        overflow-y: auto;
    }
</style>