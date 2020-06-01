import {segmentType} from "@/common/segment_type";
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

                            <!--Reorder-->
                            <b-col>
                                <b-button block
                                          class="float-right" id="rearrange"
                                          size="sm"
                                          title="Reorder">
                                    <b-icon-list-ol></b-icon-list-ol>
                                </b-button>
                            </b-col>

                            <!--Clear all-->
                            <b-col>
                                <b-dropdown block
                                          class="float-right" id="clear"
                                          size="sm"
                                          variant="danger"
                                          title="Clear all" no-caret style="width: 45px; padding-right: 1px">
                                    <template v-slot:button-content>
                                        <b-icon-backspace></b-icon-backspace>
                                    </template>
                                    <b-dropdown-text style="min-width: 15rem">
                                        <b-container>
                                            <b-form-row>
                                                <b-col style="margin-top: 0.2rem">
                                                    Clear all?
                                                </b-col>
                                                <b-col>
                                                    <b-button block
                                                              size="sm"
                                                              variant="danger"
                                                              @click="resetBottomSegments">
                                                        Confirm
                                                    </b-button>
                                                </b-col>
                                            </b-form-row>
                                        </b-container>
                                    </b-dropdown-text>
                                </b-dropdown>
                            </b-col>

                            <!--Add segment modal-->
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
                        <b-container class="p-0">
                            <b-form-row>
                                <b-col sm="1">
                                    <b-form-checkbox>
                                    </b-form-checkbox>
                                </b-col>
                                <b-col sm="15">
                                    {{formatBottomSegment(segment)}}
                                </b-col>
                                <b-col>
                                    <b-dropdown class="float-right" variant="danger" title="Delete segment" size="sm" no-caret>
                                        <template v-slot:button-content>
                                            <b-icon-trash></b-icon-trash>
                                        </template>
                                        <b-dropdown-text style="min-width: 20rem">
                                            <b-container>
                                                <b-form-row>
                                                    <b-col style="margin-top: 0.2rem" sm="7">
                                                        Delete segment?
                                                    </b-col>
                                                    <b-col sm="5">
                                                        <b-button block
                                                                  size="sm"
                                                                  variant="danger"
                                                                  @click="removeBottomSegmentAtIndex(index)">
                                                            Confirm
                                                        </b-button>
                                                    </b-col>
                                                </b-form-row>
                                            </b-container>
                                        </b-dropdown-text>
                                    </b-dropdown>
                                </b-col>
                                <b-col sm="2">
                                    <b-button size="sm" class="float-right" @click="setEditSegment(segment, index)">
                                        <b-icon-pencil></b-icon-pencil>
                                    </b-button>
                                </b-col>
                            </b-form-row>
                        </b-container>
                    </b-list-group-item>
                </b-list-group>
                <BSEditModal :edit-segment="editSegment" @submitted="onBSEditSubmitted"></BSEditModal>
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
    import {prettyFromMilliseconds} from "@/common/time";
    import BSEditModal from "@/components/plan/BSEditModal.vue";
    import {segmentType} from "@/common/segment_type";

    const plan = namespace('Plan');
    @Component({
        components: {BSEditModal, BSModal}
    })
    export default class BottomSegments extends Vue {
        editSegment: [diveSegment, gas] = [
            {
                segmentType: segmentType.DiveSegment,
                startDepth: 0,
                endDepth: 0,
                time: 0,
                ascentRate: 0,
                descentRate: 0
            },
            {
                o2: 0,
                he: 0
            }
        ];

        editingIdx = 0;

        setEditSegment(to: [diveSegment, gas], idx: number): void {
            this.editSegment = to;
            this.editingIdx = idx;

            this.$bvModal.show('bottom-segment-edit-modal')
        }

        @plan.State
        public bottomSegments!: Array<[diveSegment, gas]>;

        @plan.Mutation
        public pushBottomSegment!: (elem: [diveSegment, gas]) => void;

        @plan.Mutation
        public resetBottomSegments!: () => void;

        @plan.Mutation
        public updateBottomSegmentAtIndex!: (elem: [[diveSegment, gas], number]) => void;

        @plan.Mutation
        public removeBottomSegmentAtIndex!: (idx: number) => void;

        onBSSubmitted(value: [diveSegment, gas]) {
            this.pushBottomSegment(value);
        }

        onBSEditSubmitted(value: [diveSegment, gas]) {
            this.updateBottomSegmentAtIndex([value, this.editingIdx]);
            this.editingIdx = 0; // Maybe zero-initialise the editSegment as well?
        }

        formatBottomSegment(segment: [diveSegment, gas]): string {
            return `${segment[0].startDepth}m | ${prettyFromMilliseconds(segment[0].time)} | ${segment[1].o2}/${segment[1].he}`
        }
    }


</script>

<style scoped>
    .list_group {
        height: 75vh;
        overflow-y: auto;
    }
</style>