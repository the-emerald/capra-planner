<template>
    <div>
        <b-modal size="lg" id="settings-modal" title="Settings" @shown="syncFields" hide-footer>
            <ValidationObserver ref="observer" v-slot="{passes}">
                <b-form @submit.prevent="passes(onSubmit)" @reset="resetForm">
                    <b-tabs content-class="mt-3">
                        <b-tab title="General">
                            <b-container>
                                <b-form-row>
                                    Surface air consumption:
                                </b-form-row>
                                <b-form-row class="mt-3">
                                    <b-col style="margin-top: 0.2rem" sm="4">
                                        Bottom segments
                                    </b-col>
                                    <b-col>
                                        <ValidationProvider name="sacBottom" rules="required|positive" v-slot="{valid, errors}">
                                            <b-form-input
                                                placeholder="ltr/min"
                                                type="number"
                                                v-model.number="sacBottom"
                                                :state="errors[0] ? false : (valid ? true : null)"
                                            ></b-form-input>
                                        </ValidationProvider>
                                    </b-col>
                                </b-form-row>
                                <b-form-row class="mt-3">
                                    <b-col style="margin-top: 0.2rem" sm="4">
                                        Deco segments
                                    </b-col>
                                    <b-col>
                                        <ValidationProvider name="sacDeco" rules="required|positive" v-slot="{valid, errors}">
                                            <b-form-input
                                                    placeholder="ltr/min"
                                                    type="number"
                                                    v-model.number="sacDeco"
                                                    :state="errors[0] ? false : (valid ? true : null)"
                                            ></b-form-input>
                                        </ValidationProvider>
                                    </b-col>
                                </b-form-row>
                                <hr class="my-3">
                                <b-form-row>
                                    <b-col style="margin-top: 0.2rem" sm="4">
                                        Ascent rate
                                    </b-col>
                                    <b-col>
                                        <ValidationProvider name="ascentRate" rules="required|positive" v-slot="{valid, errors}">
                                            <b-form-input
                                                    placeholder="m/min"
                                                    type="number"
                                                    v-model.number="ascentRate"
                                                    :state="errors[0] ? false : (valid ? true : null)"
                                            ></b-form-input>
                                        </ValidationProvider>
                                    </b-col>
                                </b-form-row>
                                <b-form-row class="mt-3">
                                    <b-col style="margin-top: 0.2rem" sm="4">
                                        Descent rate
                                    </b-col>
                                    <b-col>
                                        <ValidationProvider name="descentRate" rules="required|positive" v-slot="{valid, errors}">
                                            <b-form-input
                                                    placeholder="m/min"
                                                    type="number"
                                                    v-model.number="descentRate"
                                                    :state="errors[0] ? false : (valid ? true : null)"
                                            ></b-form-input>
                                        </ValidationProvider>
                                    </b-col>
                                </b-form-row>
                                <b-form-row class="mt-3">
                                  <b-col style="margin-top: 0.2rem" sm="4">
                                    Water density
                                  </b-col>
                                  <b-col>
                                    <b-form-select
                                        v-model="waterDensity"
                                        :options="waterDensityOptions"
                                    ></b-form-select>
                                  </b-col>
                                  <b-col sm="4">
                                    <ValidationProvider name="waterDensity" rules="required|positive|gt:0" v-slot="{valid, errors}">
                                      <b-form-input
                                          placeholder="kg/m^-3"
                                          type="number"
                                          v-model.number="waterDensity"
                                          :state="errors[0] ? false : (valid ? true : null)"
                                      >
                                      </b-form-input>
                                    </ValidationProvider>
                                  </b-col>
                                </b-form-row>
                            </b-container>
                        </b-tab>
                        <b-tab title="ZHL">
                            <b-container>
                                <b-form-row class="mt-3">
                                    <b-col style="margin-top: 0.2rem" sm="4">
                                        Algorithm subtype:
                                    </b-col>
                                    <b-col>
                                        <b-form-select v-model="zhlSubtype" :options="zhlSubtypeOptions"></b-form-select>
                                    </b-col>
                                </b-form-row>
                                <b-form-row class="mt-3">
                                    <b-col style="margin-top: 0.2rem" sm="4">
                                        GF Low
                                    </b-col>
                                    <b-col>
                                        <ValidationProvider name="gfLow" rules="required|positive|lt:101" v-slot="{valid, errors}">
                                            <b-form-input
                                                    placeholder="0-100"
                                                    type="number"
                                                    v-model.number="gfl"
                                                    :state="errors[0] ? false : (valid ? true : null)"
                                            ></b-form-input>
                                        </ValidationProvider>
                                    </b-col>
                                </b-form-row>
                                <b-form-row class="mt-3">
                                    <b-col style="margin-top: 0.2rem" sm="4">
                                        GF High
                                    </b-col>
                                    <b-col>
                                        <ValidationProvider name="gfHigh" rules="required|positive|lt:101" v-slot="{valid, errors}">
                                            <b-form-input
                                                    placeholder="0-100"
                                                    type="number"
                                                    v-model.number="gfh"
                                                    :state="errors[0] ? false : (valid ? true : null)"
                                            ></b-form-input>
                                        </ValidationProvider>
                                    </b-col>
                                </b-form-row>
                            </b-container>
                        </b-tab>
                        <b-tab title="VPM" disabled>
                            <b-card-text>VPM settings</b-card-text>
                        </b-tab>
                    </b-tabs>
                    <br>
                    <div class="modal-footer">
                        <b-button @click="$bvModal.hide('settings-modal')">Cancel</b-button>
                        <b-button type="submit" variant="primary">Save</b-button>
                    </div>
                </b-form>
            </ValidationObserver>
        </b-modal>
    </div>
</template>

<script lang="ts">
    import Component from "vue-class-component";
    import {namespace} from "vuex-class";
    import {GeneralSettings, VPMSettings, ZHLSettings, ZHLSubtypeFromString} from "@/common/serde/settings";
    import {ValidationObserver, ValidationProvider} from "vee-validate";
    import {Vue} from "vue-property-decorator";
    import {updateGeneralSettings, updateZHLSettings} from "@/common/routes";
    import {makeErrorToast, makeSuccessToast} from "@/common/toast";
    import {handleAxiosError} from "@/common/axios_error";
    import {User} from "@/common/serde/user";

    const userInfo = namespace('UserInfo');

    @Component({
        components: {
            ValidationProvider,
            ValidationObserver
        }
    })
    export default class SettingsModal extends Vue {
        @userInfo.State
        public user!: User;

        @userInfo.State
        public userZHLSettings!: ZHLSettings;

        zhlSubtypeOptions: Array<string> = ["B", "C"]
        zhlSubtype = '';
        gfl = '';
        gfh = '';

        @userInfo.State
        public userVPMSettings!: VPMSettings;

        // Nothing to see here

        @userInfo.State
        public userGeneralSettings!: GeneralSettings;

        sacBottom = '';
        sacDeco = '';
        ascentRate = '';
        descentRate = '';
        waterDensity = '';

        // waterDensityOptions: Array<string> = ["Fresh", "EN13319", "Salt", "Custom"];
        // waterDensityValues: Array<number> = [997, 1020, 1024] // TODO: Are these values correct?
        waterDensityOptions = [
          { value: '', text: "Choose preset", disabled: true },
          { value: 997, text: "Fresh" },
          { value: 1020, text: "EN13319" },
          { value: 1024, text: "Salt" },
        ];

        @userInfo.Mutation
        public updateZHLSettings!: (elem: ZHLSettings) => void;

        @userInfo.Mutation
        public updateVPMSettings!: (elem: VPMSettings) => void;

        @userInfo.Mutation
        public updateGeneralSettings!: (elem: GeneralSettings) => void;

        syncFields() {
            // ZHL
            this.zhlSubtype = this.userZHLSettings.variant.toString()
            this.gfl = this.userZHLSettings.gfl.toString();
            this.gfh = this.userZHLSettings.gfh.toString();

            // General settings
            this.sacBottom = this.userGeneralSettings.sacBottom.toString();
            this.sacDeco = this.userGeneralSettings.sacDeco.toString();
            this.ascentRate = (this.userGeneralSettings.ascentRate * -1).toString();
            this.descentRate = this.userGeneralSettings.descentRate.toString();
            this.waterDensity = this.userGeneralSettings.waterDensity.toString();
        }

        onSubmit() {
            const zhlSettings: ZHLSettings = {
                variant: ZHLSubtypeFromString(this.zhlSubtype),
                gfh: Number(this.gfh),
                gfl: Number(this.gfl)
            };
            const generalSettings: GeneralSettings = {
                ascentRate: Number(this.ascentRate) * -1,
                descentRate: Number(this.descentRate),
                sacBottom: Number(this.sacBottom),
                sacDeco: Number(this.sacDeco),
                waterDensity: Number(this.waterDensity)
            };

            const z = updateZHLSettings(this.user, zhlSettings);
            const g = updateGeneralSettings(this.user, generalSettings);

            Promise.all([z, g])
            .then(() => {
                makeSuccessToast(this, "Saved settings.");

                this.updateGeneralSettings(generalSettings);
                this.updateZHLSettings(zhlSettings);

                this.$bvModal.hide('settings-modal')
            })
            .catch((error) => {
                makeErrorToast(this, handleAxiosError(error))
            });
        }

        resetForm() {
            this.syncFields();
        }
    }
</script>

<style scoped>

</style>