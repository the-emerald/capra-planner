<template>
    <div>
        <b-modal size="lg" id="settings-modal" title="Settings" @shown="syncFields" hide-footer>
            <ValidationObserver ref="observer" v-slot="{passes}">
                <b-form @submit.prevent="passes(onSubmit)" @reset="resetForm">
                    <b-tabs content-class="mt-3">
                        <b-tab title="General">
                            <b-container>
                                <b-form-row>
                                    Surface Air Consumption
                                </b-form-row>
                                <b-form-row class="mt-3">
                                    <b-col style="margin-top: 0.2rem" sm="4">
                                        on bottom segments:
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
                            </b-container>
                        </b-tab>
                        <b-tab title="ZHL">
                            <b-card-text>ZHL settings</b-card-text>
                        </b-tab>
                        <b-tab title="VPM" disabled>
                            <b-card-text>ZHL settings</b-card-text>
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
    import {GeneralSettings, VPMSettings, ZHLSettings} from "@/common/serde/settings";
    import {ValidationObserver, ValidationProvider} from "vee-validate";
    import {Vue} from "vue-property-decorator";
    import {updateGeneralSettings, updateZHLSettings} from "@/common/routes";
    import {makeErrorToast} from "@/common/toast";

    const userInfo = namespace('UserInfo');

    @Component({
        components: {
            ValidationProvider,
            ValidationObserver
        }
    })
    export default class SettingsModal extends Vue {
        @userInfo.State
        public selectedUser!: number;

        @userInfo.State
        public userZHLSettings!: ZHLSettings;

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

        syncFields() {
            // ZHL
            this.gfl = this.userZHLSettings.gfl.toString();
            this.gfh = this.userZHLSettings.gfh.toString();

            // General settings
            this.sacBottom = this.userGeneralSettings.sac_bottom.toString();
            this.sacDeco = this.userGeneralSettings.sac_deco.toString();
            this.ascentRate = this.userGeneralSettings.ascent_rate.toString();
            this.descentRate = this.userGeneralSettings.descent_rate.toString();
        }

        onSubmit() {
            const zhlSettings: ZHLSettings = {
                gfh: Number(this.gfh),
                gfl: Number(this.gfl)
            };
            const generalSettings: GeneralSettings = {
                ascent_rate: Number(this.ascentRate),
                descent_rate: Number(this.descentRate),
                sac_bottom: Number(this.sacBottom),
                sac_deco: Number(this.sacDeco)
            };

            const z = updateZHLSettings(zhlSettings);
            const g = updateGeneralSettings(generalSettings);

            Promise.all([z, g])
            .then()
            .catch((error) => {
                makeErrorToast(this, error)
            });
        }

        resetForm() {
            this.syncFields();
        }
    }
</script>

<style scoped>

</style>