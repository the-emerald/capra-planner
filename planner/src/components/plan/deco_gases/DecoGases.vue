import {segmentType} from "@/common/segment_type";
<template>
    <div>
        <b-card-group>
            <b-card no-body>
                <b-card-header>
                    <b-container class="p-0">
                        <b-form-row>
                            <b-col sm="8" style="margin-top: 0.2rem">
                                Deco gases
                            </b-col>
                            <!-- UI buttons -->
                            <!-- Clear all-->
                            <b-col>
                                <b-dropdown block
                                            class="float-right" id="clear-dg"
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
                                                              @click="resetDecoGases">
                                                        Confirm
                                                    </b-button>
                                                </b-col>
                                            </b-form-row>
                                        </b-container>
                                    </b-dropdown-text>
                                </b-dropdown>
                            </b-col>

                            <!-- Add gas -->
                            <b-col>
                                <b-button block
                                          class="float-right" id="show-deco-gas-new-modal-btn"
                                          @click="$bvModal.show('deco-gas-new-modal')"
                                          variant="info"
                                          size="sm"
                                          title="Add">
                                    <b-icon-plus-circle></b-icon-plus-circle>
                                </b-button>
                            </b-col>
                        </b-form-row>
                    </b-container>

                    <!-- Relevant modals -->
                    <DecoGasNewModal @submitted="onDecoGasNewSubmitted"></DecoGasNewModal>
                </b-card-header>
                <b-list-group flush class="overflow-auto list_group">
                    <b-list-group-item v-for="(gas, index) in decoGases" :key="`gas-${index}`">
                        <b-container class="p-0">
                            <b-form-row>
                                <b-col sm="1">
                                    <b-form-checkbox :checked="gas[0]" @change="invertGasCheckbox(index)">
                                    </b-form-checkbox>
                                </b-col>
                                <b-col sm="15">
                                    {{displayDecoGas(gas[1])}}
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
                                                        Delete gas?
                                                    </b-col>
                                                    <b-col sm="5">
                                                        <b-button block
                                                                  size="sm"
                                                                  variant="danger"
                                                                  @click="removeDecoGasAtIndex(index)">
                                                            Confirm
                                                        </b-button>
                                                    </b-col>
                                                </b-form-row>
                                            </b-container>
                                        </b-dropdown-text>
                                    </b-dropdown>
                                </b-col>
                                <b-col sm="2">
                                    <b-button size="sm" class="float-right" @click="setEditGas(gas[1], index)">
                                        <b-icon-pencil></b-icon-pencil>
                                    </b-button>
                                </b-col>
                            </b-form-row>
                        </b-container>
                    </b-list-group-item>
                </b-list-group>
                <DecoGasEditModal :edit-gas="editGas" @submitted="onDecoGasEditSubmitted"></DecoGasEditModal>
            </b-card>
        </b-card-group>
    </div>
</template>

<script lang="ts">
    import {Component, Vue} from "vue-property-decorator";
    import {namespace} from 'vuex-class';
    import {DecoGasElement} from "@/store/plan";
    import {displayDecoGasElement} from "@/common/display";
    import DecoGasNewModal from "@/components/plan/deco_gases/DecoGasNewModal.vue";
    import DecoGasEditModal from "@/components/plan/deco_gases/DecoGasEditModal.vue";

    const plan = namespace('Plan');
    @Component({
        components: {DecoGasEditModal, DecoGasNewModal}
    })
    export default class DecoGases extends Vue {
        editGas: DecoGasElement = {
            gas: {
                o2: 0,
                he: 0
            }
        };

        editingIdx = 0;

        // eslint-disable-next-line
        setEditGas(to: DecoGasElement, idx: number) {
            // TODO: Populate edit gas
            // Refer to BottomSegments.vue
            this.editGas = to;
            this.editingIdx = idx;
            this.$bvModal.show('deco-gas-edit-modal')
        }

        invertGasCheckbox(idx: number) {
            const selected = this.decoGases[idx][0];
            const value = this.decoGases[idx][1];
            this.updateDecoGasAtIndex([[!selected, value], idx]);
        }

        @plan.State
        public decoGases!: Array<[boolean, DecoGasElement]>;

        @plan.Mutation
        public pushDecoGas!: (to: [boolean, DecoGasElement]) => void;

        @plan.Mutation
        public resetDecoGases!: () => void;

        @plan.Mutation
        public updateDecoGasAtIndex!: (elem: [[boolean, DecoGasElement], number]) => void;

        @plan.Mutation
        public removeDecoGasAtIndex!: (idx: number) => void;

        @plan.Mutation
        public replaceDecoGases!: (to: Array<[boolean, DecoGasElement]>) => void;

        // Emits
        onDecoGasNewSubmitted(value: DecoGasElement) {
            this.pushDecoGas([true, value]);
        }

        onDecoGasEditSubmitted(value: DecoGasElement) {
            const selected = this.decoGases[this.editingIdx][0];
            this.updateDecoGasAtIndex([[selected, value], this.editingIdx]);
            this.editingIdx = 0;
        }

        // Re-export
        displayDecoGas(input: DecoGasElement): string {
            return displayDecoGasElement(input);
        }
    }
</script>

<style scoped>
    .list_group {
        height: 75vh;
        overflow-y: auto;
    }
</style>