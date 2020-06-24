<template>
    <div>
        <b-modal id="bottom-segment-reorder-modal" title="Reorder bottom segments" @shown="setToOriginal" hide-footer>
                <b-form @submit.prevent="onSubmit" @reset="resetForm">
                    <br>
                    <b-container>
                        <b-row>
                            <b-col>
                                <b-list-group>
                                    <b-list-group-item
                                            v-for="(segments, index) in modalOrdering"
                                            :key="`reorder-${index}`"
                                            href="#"
                                            v-bind:class="{active: isSelected(index)}"
                                            v-on:click="selected = index">
                                        {{displayBottomSegmentElement(segments[1])}}
                                    </b-list-group-item>
                                </b-list-group>
                            </b-col>
                            <b-col sm="3">
                                <b-form-row>
                                    <b-button @click="moveSelectedUp">
                                        <b-icon-arrow-up></b-icon-arrow-up>
                                    </b-button>
                                </b-form-row>
                                <br>
                                <b-form-row>
                                    <b-button @click="moveSelectedDown">
                                        <b-icon-arrow-down></b-icon-arrow-down>
                                    </b-button>
                                </b-form-row>
                            </b-col>
                        </b-row>
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
    import {displayBottomSegmentElement} from "@/common/display";
    import {BottomSegmentElement} from "@/store/plan";

    @Component
    export default class BSReorderModal extends Vue {
        // Original ordering. Do not touch!
        @Prop() private originalOrdering!: Array<[boolean, BottomSegmentElement]>;

        // Ordering to modify using the modal
        modalOrdering: Array<[boolean, BottomSegmentElement]> = [];

        // The current selected value. -1 is none
        selected = -1;

        isSelected(idx: number) {
            return this.selected == idx;
        }

        onSubmit() {
            this.$emit(
                'submitted',
                this.modalOrdering
            );
            this.setToOriginal();
            this.$bvModal.hide('bottom-segment-reorder-modal')
        }

        resetForm() {
            this.setToOriginal();
        }

        setToOriginal() {
            this.modalOrdering = [...this.originalOrdering]; // Copy prop
            this.selected = -1; // Set default
        }

        // Move selected element up once
        moveSelectedUp() {
            if (this.selected != 0) {
                const swap = [this.modalOrdering[this.selected - 1], this.modalOrdering[this.selected]];
                this.selected -= 1;
                this.modalOrdering.splice(this.selected, 2, swap[1], swap[0]);
            }
        }

        // Move selected element down once
        moveSelectedDown() {
            if (this.selected != this.modalOrdering.length - 1) {
                const swap = [this.modalOrdering[this.selected], this.modalOrdering[this.selected + 1]];
                this.modalOrdering.splice(this.selected, 2, swap[1], swap[0]);
                this.selected += 1;
            }
        }

        // Re-export
        displayBottomSegmentElement(input: BottomSegmentElement): string {
            return displayBottomSegmentElement(input);
        }
    }
</script>

<style scoped>

</style>