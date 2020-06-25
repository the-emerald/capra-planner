import {segmentType} from "@/common/segment_type";
<template>
    <div>
        <b-modal id="bottom-segment-new-modal" title="Add bottom segment" hide-footer>
            <ValidationObserver ref="observer" v-slot="{passes}">
                <b-form @submit.prevent="passes(onSubmit)" @reset="resetForm">
                    <b-container>
                        <!--Depth-->
                        <b-form-row>
                            <b-col style="margin-top: 0.4rem" sm="4">
                                Depth
                            </b-col>
                            <b-col>
                                <ValidationProvider name="depth" rules="required|positive" v-slot="{valid, errors}">
                                    <b-form-input
                                        placeholder="m"
                                        type="number"
                                        v-model.number="depth"
                                        :state="errors[0] ? false : (valid ? true : null)"
                                    ></b-form-input>
                                </ValidationProvider>
                            </b-col>
                        </b-form-row>
                        <br>

                        <!--Time-->
                        <b-form-row>
                            <b-col style="margin-top: 0.4rem" sm="4">
                                Time
                            </b-col>
                            <b-col>
                                <ValidationProvider name="minutes" rules="required|positive|gt:0" v-slot="{valid, errors}">
                                    <b-form-input
                                        placeholder="mm"
                                        type="number"
                                        v-model.number="timeMin"
                                        :state="errors[0] ? false : (valid ? true : null)"
                                    ></b-form-input>
                                </ValidationProvider>
                            </b-col>
                            <div style="margin-top: 0.4rem">:</div>
                            <b-col>
                                <ValidationProvider name="seconds" rules="required|positive|lt:60" v-slot="{valid, errors}">
                                    <b-form-input
                                        placeholder="ss"
                                        type="number"
                                        v-model.number="timeSec"
                                        :state="errors[0] ? false : (valid ? true : null)"
                                    ></b-form-input>
                                </ValidationProvider>
                            </b-col>
                        </b-form-row>
                        <br>

                        <!--Mix-->
                        <b-form-row>
                            <b-col style="margin-top: 0.4rem" sm="4">
                                Mix
                            </b-col>
                                <b-col>
                                    <ValidationProvider name="o2_" rules="required|positive|gas:@he_" v-slot="{valid, errors}">
                                        <b-form-input
                                            placeholder="O₂"
                                            type="number"
                                            v-model.number="o2"
                                            :state="errors[0] ? false : (valid ? true : null)"
                                        ></b-form-input>
                                    </ValidationProvider>
                                </b-col>
                                <div style="margin-top: 0.4rem">/</div>
                                <b-col>
                                    <ValidationProvider name="he_" rules="required|positive|gas:@o2_" v-slot="{valid, errors}">
                                        <b-form-input
                                            placeholder="He"
                                            type="number"
                                            v-model.number="he"
                                            :state="errors[0] ? false : (valid ? true : null)"
                                        ></b-form-input>
                                    </ValidationProvider>
                                </b-col>
                        </b-form-row>
                    </b-container>
                    <br>
                    <div class="modal-footer">
                        <b-button type="reset" variant="danger">Reset</b-button>
                        <b-button type="submit" variant="primary">Add</b-button>
                    </div>
                </b-form>
            </ValidationObserver>
        </b-modal>
    </div>
</template>

<script lang="ts">
    import {Component, Vue} from "vue-property-decorator";
    import {ValidationObserver, ValidationProvider} from "vee-validate"
    import "@/common/validation"
    import {DiveSegment} from "@/common/serde/dive_segment";
    import {segmentType} from "@/common/serde/segment_type";
    import {Gas} from "@/common/serde/gas";
    import {minutesSecondToMinutes} from "@/common/time";
    import {BottomSegmentElement} from "@/store/plan";

    @Component({
        components: {
            ValidationProvider,
            ValidationObserver
        }
    })
    export default class BSNewModal extends Vue {
        $refs!: {
            observer: InstanceType<typeof ValidationObserver>;
        };

        depth = '';
        timeMin = '';
        timeSec = '';
        o2 = '';
        he = '';

        onSubmit() {
            // TODO: Fix hardcoded values
            const newDiveSegment: DiveSegment = {
                ascentRate: -10,
                descentRate: 20,
                startDepth: Number(this.depth),
                endDepth: Number(this.depth),
                segmentType: segmentType.DiveSegment,
                time: minutesSecondToMinutes(Number(this.timeMin), Number(this.timeSec)) // Parse this or die trying
            };

            const newGas: Gas = {
                he: Number(this.he),
                o2: Number(this.o2)
            };

            const elem: BottomSegmentElement = {
                diveSegment: newDiveSegment,
                gas: newGas

            };

            this.$emit(
                'submitted',
                elem
            );

            this.resetForm();
            this.$bvModal.hide('bottom-segment-modal')
        }

        resetForm() {
            this.depth = '';
            this.timeMin = '';
            this.timeSec = '';
            this.o2 = '';
            this.he = '';
            requestAnimationFrame(() => {
                this.$refs.observer.reset();
            });
        }
    }
</script>

<style scoped>

</style>