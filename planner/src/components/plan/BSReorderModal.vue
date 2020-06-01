<template>
    <div>
        <b-modal id="bottom-segment-reorder-modal" title="Reorder bottom segments" hide-footer>
                <b-form @submit="onSubmit" @reset="resetForm">
                    <br>
                    <div class="modal-footer">
                        <b-button type="reset" variant="danger">Reset</b-button>
                        <b-button type="submit" variant="primary">Update</b-button>
                    </div>
                </b-form>
        </b-modal>
    </div>
</template>

<script lang="ts">
    import {Component, Vue} from "vue-property-decorator";
    @Component
    export default class BSReorderModal extends Vue {

        onSubmit() {
            // TODO: Add submission logic
            this.resetForm();
            this.$bvModal.hide('bottom-segment-reorder-modal')
        }

        resetForm() {
            // TODO: Add reset logic
        }
    }
</script>

<style scoped>

</style>