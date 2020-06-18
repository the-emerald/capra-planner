<template>
    <div>
        <b-modal id="bottom-segment-reorder-modal" title="Reorder bottom segments" @shown="setToOriginal" hide-footer>
                <b-form @submit.prevent="onSubmit" @reset="resetForm">
                    <br>
                    <b-container>
                        <b-form-row>
                            <!--TODO: List goes here-->
                            <b-col>
                                <b-list-group>
                                    <b-list-group-item
                                            v-for="(segments, index) in modalOrdering"
                                            :key="`reorder-${index}`"
                                            href="#"
                                            v-bind:class="{active: isSelected(index)}"
                                            v-on:click="selected = index">
                                        {{displayDiveSegmentGas(segments)}}
                                    </b-list-group-item>
                                </b-list-group>
                            </b-col>
                            <!--TODO: Buttons go here-->
                            <b-col sm="3">
                                <p>Buttons!</p>
                            </b-col>
                        </b-form-row>
                    </b-container>
                    <br>
                    <div class="modal-footer">
                        <b-button @click="setToOriginal">Reset</b-button>
                        <b-button type="submit" variant="primary">Update</b-button>
                    </div>
                </b-form>
        </b-modal>
    </div>
</template>

<script lang="ts">
    import {Component, Prop, Vue} from "vue-property-decorator";
    import {diveSegment} from "@/common/dive_segment";
    import {gas} from "@/common/gas";
    import {displayDiveSegmentGas} from "@/common/display";

    @Component
    export default class BSReorderModal extends Vue {
        // Original ordering. Do not touch!
        @Prop() private originalOrdering!: Array<[diveSegment, gas]>;

        // Ordering to modify using the modal
        modalOrdering: Array<[diveSegment, gas]> = [];

        // The current selected value. -1 is none
        selected = -1;

        isSelected(idx: number) {
            return this.selected == idx;
        }

        onSubmit() {
            // TODO: Add submission logic
            this.resetForm();
            this.$bvModal.hide('bottom-segment-reorder-modal')
        }

        resetForm() {
            // TODO: Add reset logic
        }

        setToOriginal() {
            this.modalOrdering = [...this.originalOrdering];
            this.selected = -1;
        }

        // Re-export
        displayDiveSegmentGas(input: [diveSegment, gas]): string {
            return displayDiveSegmentGas(input);
        }
    }
</script>

<style scoped>

</style>