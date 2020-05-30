<template>
    <div>
        <b-modal id="bottom-segment-modal" title="Add bottom segment" hide-footer>
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
                                <ValidationProvider name="minutes" rules="required|positive" v-slot="{valid, errors}">
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
                                <ValidationProvider name="seconds" rules="required|positive|max:60" v-slot="{valid, errors}">
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
                        <b-button type="submit" variant="primary">Add</b-button>
                        <b-button type="reset" variant="danger">Reset</b-button>
                    </div>
                </b-form>
            </ValidationObserver>
        </b-modal>
    </div>
</template>

<script lang="ts">
    import {Component, Vue} from "vue-property-decorator";
    import {ValidationProvider, ValidationObserver} from "vee-validate"
    import "@/common/validation"

    @Component({
        components: {
            ValidationProvider,
            ValidationObserver
        }
    })
    export default class BSModal extends Vue {
        $refs!: {
            observer: InstanceType<typeof ValidationObserver>;
        };

        depth = null;
        timeMin = null;
        timeSec = null;
        o2 = null;
        he = null;

        onSubmit() { // TODO: Add behaviour for emitting
            console.log("Submitted");
        }

        resetForm() {
            this.depth = null;
            this.timeMin = null;
            this.timeSec = null;
            this.o2 = null;
            this.he = null;
            requestAnimationFrame(() => {
                this.$refs.observer.reset();
            });
        }
    }
</script>

<style scoped>

</style>