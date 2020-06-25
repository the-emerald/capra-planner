<template>
    <div>
        <b-modal id="deco-gas-edit-modal" title="Edit deco gas" @shown="syncFields" hide-footer>
            <ValidationObserver ref="observer" v-slot="{passes}">
                <b-form @submit.prevent="passes(onSubmit)" @reset="resetForm">
                    <b-container>
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
                        <br>

                        <b-form-row>
                            <b-col sm="4">
                                MOD
                            </b-col>
                            <b-col>
                                <ValidationProvider name="maxOpDepth" rules="positive|gt:0" v-slot="{valid, errors}">
                                    <b-form-input
                                        placeholder="m (optional)"
                                        type="number"
                                        v-model.number="maxOpDepth"
                                        :state="errors[0] ? false : (valid ? true : null)"
                                        ></b-form-input>
                                </ValidationProvider>
                            </b-col>
                        </b-form-row>
                    </b-container>
                    <br>
                    <div class="modal-footer">
                        <b-button @click="$bvModal.hide('deco-gas-edit-modal')">Cancel</b-button>
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
    import {DecoGasElement} from "@/store/plan";

    @Component({
        components: {
            ValidationProvider,
            ValidationObserver
        }
    })
    export default class DecoGasNewModal extends Vue {
        @Prop() private editGas!: DecoGasElement;

        syncFields() {
            this.he = this.editGas.gas.he.toString();
            this.o2 = this.editGas.gas.o2.toString();
            if (this.editGas.gas.maxOpDepth) {
                this.maxOpDepth = this.editGas.gas.maxOpDepth.toString();
            }
        }

        $refs!: {
            observer: InstanceType<typeof ValidationObserver>;
        };

        o2 = '';
        he = '';
        maxOpDepth = '';

        onSubmit() {
            const elem: DecoGasElement = {
                gas: {
                    o2: Number(this.o2),
                    he: Number(this.he)
                }
            };

            if (this.maxOpDepth != '') { // Try to read MOD
                elem.gas.maxOpDepth = Number(this.maxOpDepth);
            }

            this.$emit(
                'submitted',
                elem
            );

            this.resetForm();
            this.$bvModal.hide('deco-gas-edit-modal')
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