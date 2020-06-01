<template>
    <div>
        <b-modal id="bottom-segment-reorder-modal" title="Reorder bottom segments" @shown="setToOriginal" hide-footer>
                <b-form @submit="onSubmit" @reset="resetForm">
                    <br>
                    <b-container>
                        <b-row>
                            <!--TODO: List goes here-->
                            <b-col>
                                <p>List of segments</p>
                            </b-col>
                            <!--TODO: Buttons go here-->
                            <b-col>
                                <p>Buttons!</p>
                            </b-col>
                        </b-row>
                    </b-container>
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
    @Component
    export default class BSReorderModal extends Vue {
        // Original ordering. Do not touch!
        @Prop() private originalOrdering!: Array<[diveSegment, gas]>;

        onSubmit() {
            // TODO: Add submission logic
            this.resetForm();
            this.$bvModal.hide('bottom-segment-reorder-modal')
        }

        resetForm() {
            // TODO: Add reset logic
        }

        setToOriginal() {
            // TODO: Add logic for copying original ordering
        }
    }
</script>

<style scoped>

</style>