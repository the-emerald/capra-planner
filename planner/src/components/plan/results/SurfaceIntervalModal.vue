<template>
    <div>
        <b-modal id="surface-interval-modal" title="Add surface interval" @shown="syncFields" hide-footer>
            <ValidationObserver ref="observer" v-slot="{passes}">
                <b-form @submit.prevent="passes(onSubmit)" @reset="resetForm">
                    <b-container>
                        <!--Mix-->
                        <b-form-row>
                            <b-col style="margin-top: 0.4rem" sm="4">
                                Duration
                            </b-col>
                            <b-col>
                                <ValidationProvider name="hours" rules="required|positive" v-slot="{valid, errors}">
                                    <b-form-input
                                            placeholder="hh"
                                            type="number"
                                            v-model.number="siTimeHour"
                                            :state="errors[0] ? false : (valid ? true : null)"
                                    ></b-form-input>
                                </ValidationProvider>
                            </b-col>
                            <div style="margin-top: 0.4rem">:</div>
                            <b-col>
                                <ValidationProvider name="minutes" rules="required|positive|lt:60" v-slot="{valid, errors}">
                                    <b-form-input
                                            placeholder="mm"
                                            type="number"
                                            v-model.number="siTimeMin"
                                            :state="errors[0] ? false : (valid ? true : null)"
                                    ></b-form-input>
                                </ValidationProvider>
                            </b-col>
                        </b-form-row>
                    </b-container>
                    <br>
                    <div class="modal-footer">
                        <b-button @click="$bvModal.hide('surface-interval-modal')">Cancel</b-button>
                        <b-button type="reset" variant="danger">Reset</b-button>
                        <b-button @click="zeroFields" variant="danger">Zero</b-button>
                        <b-button type="submit" variant="primary">Update</b-button>
                    </div>
                </b-form>
            </ValidationObserver>
        </b-modal>
    </div>
</template>

<script lang="ts">
    import {Component, Prop, Vue} from "vue-property-decorator";
    import {ValidationObserver, ValidationProvider} from "vee-validate"
    import "@/common/validation"
    import {millisecondsToHourMinutes} from "@/common/time";

    @Component({
        components: {
            ValidationProvider,
            ValidationObserver
        }
    })
    export default class SurfaceIntervalModal extends Vue {
        @Prop() private currentSI!: number;

        siTimeHour = '';
        siTimeMin = '';

        zeroFields() {
            this.siTimeHour = '0';
            this.siTimeMin = '0';
        }

        syncFields() {
            const hm = millisecondsToHourMinutes(this.currentSI);
            this.siTimeHour = hm[0].toString();
            this.siTimeMin = hm[1].toString();
        }

        $refs!: {
            observer: InstanceType<typeof ValidationObserver>;
        };

        onSubmit() {
            const timeMillis = (Number(this.siTimeHour) * 3600000) + (Number(this.siTimeMin) * 60000)

            this.$emit(
                'submitted',
                timeMillis
            )
            this.resetForm();
            this.$bvModal.hide('surface-interval-modal')
        }

        resetForm() {
            this.syncFields();
            requestAnimationFrame(() => {
                this.$refs.observer.reset();
            });
        }
    }
</script>

<style scoped>

</style>