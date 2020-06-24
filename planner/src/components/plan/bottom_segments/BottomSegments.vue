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
                                          @click="$bvModal.show('bottom-segment-reorder-modal')"
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
                    <BSNewModal @submitted="onBSNewSubmitted"></BSNewModal>
                    <BSReorderModal :original-ordering="this.bottomSegments" @submitted="onBSReorderSubmitted"></BSReorderModal>
                </b-card-header>
                <b-list-group flush class="overflow-auto list_group">
                    <b-list-group-item v-for="(segment, index) in bottomSegments" :key="`segment-${index}`">
                        <b-container class="p-0">
                            <b-form-row>
                                <b-col sm="1">
                                    <b-form-checkbox :checked="segment[0]" @change="invertSegmentCheckbox(index)">
                                    </b-form-checkbox>
                                </b-col>
                                <b-col sm="15">
                                    {{displayDiveSegmentGas(segment[1])}}
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
                                    <b-button size="sm" class="float-right" @click="setEditSegment(segment[1], index)">
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
    import {namespace} from 'vuex-class';
    import BSEditModal from "@/components/plan/bottom_segments/BSEditModal.vue";
    import {segmentType} from "@/common/serde/segment_type";
    import BSReorderModal from "@/components/plan/bottom_segments/BSReorderModal.vue";
    import {displayBottomSegmentElement} from "@/common/display";
    import BSNewModal from "@/components/plan/bottom_segments/BSNewModal.vue";
    import {BottomSegmentElement} from "@/store/plan";

    const plan = namespace('Plan');
    @Component({
        components: {BSReorderModal, BSEditModal, BSNewModal}
    })
    export default class BottomSegments extends Vue {
        editSegment: BottomSegmentElement = {
            diveSegment: {
                segmentType: segmentType.DiveSegment,
                startDepth: 0,
                endDepth: 0,
                time: 0,
                ascentRate: 0,
                descentRate: 0
            },
            gas: {
                o2: 0,
                he: 0
            }

        };

        editingIdx = 0;

        setEditSegment(to: BottomSegmentElement, idx: number): void {
            this.editSegment = to;
            this.editingIdx = idx;

            this.$bvModal.show('bottom-segment-edit-modal')
        }

        invertSegmentCheckbox(idx: number) {
            const selected = this.bottomSegments[idx][0];
            const value = this.bottomSegments[idx][1];
            this.updateBottomSegmentAtIndex([[!selected, value], idx]);
        }

        @plan.State
        public bottomSegments!: Array<[boolean, BottomSegmentElement]>;

        @plan.Mutation
        public pushBottomSegment!: (elem: [boolean, BottomSegmentElement]) => void;

        @plan.Mutation
        public resetBottomSegments!: () => void;

        @plan.Mutation
        public updateBottomSegmentAtIndex!: (elem: [[boolean, BottomSegmentElement], number]) => void;

        @plan.Mutation
        public removeBottomSegmentAtIndex!: (idx: number) => void;

        @plan.Mutation
        public replaceBottomSegments!: (to: Array<[boolean, BottomSegmentElement]>) => void;

        // Emits from children components
        onBSNewSubmitted(value: BottomSegmentElement) {
            this.pushBottomSegment([true, value]);
        }

        onBSEditSubmitted(value: BottomSegmentElement) {
            const selected = this.bottomSegments[this.editingIdx][0];
            this.updateBottomSegmentAtIndex([[selected, value], this.editingIdx]);
            this.editingIdx = 0; // Maybe zero-initialise the editSegment as well?
        }

        onBSReorderSubmitted(value: Array<[boolean, BottomSegmentElement]>) {
            this.replaceBottomSegments(value);
        }

        // Re-export
        displayDiveSegmentGas(input: BottomSegmentElement): string {
            return displayBottomSegmentElement(input);
        }
    }


</script>

<style scoped>
    .list_group {
        height: 75vh;
        overflow-y: auto;
    }
</style>