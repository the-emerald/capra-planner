<template>
    <div>
        <b-modal id="execute-dive-modal" title="Execute dive" hide-footer>
            <p>By executing this dive, the planner will update your tissue according to the parameters set.</p>
            <p>Please make sure the dive profile is correct before proceeding. You can use "plan" mode to preview a dive.</p>
            <p><b>There is no going back!</b></p>
            <div class="modal-footer">
                <b-button @click="$bvModal.hide('execute-dive-modal')">Cancel</b-button>
                <b-button @click="onSubmit" variant="primary">Execute</b-button>
            </div>
        </b-modal>
    </div>
</template>

<script lang="ts">
    import {Component, Vue} from "vue-property-decorator";

    @Component({})
    export default class SurfaceIntervalModal extends Vue {
        onSubmit() {
            this.$emit(
                'submitted',
            );
            this.$bvModal.hide('execute-dive-modal');
        }
    }
</script>

<style scoped>

</style>