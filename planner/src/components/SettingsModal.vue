<template>
    <div>
        <b-modal id="settings-modal" title="Settings" @shown="syncFields" hide-footer>
            <ValidationObserver ref="observer" v-slot="{passes}">
                <b-form @submit.prevent="passes(onSubmit)" @reset="resetForm">
                    <b-tabs content-class="mt-3">
                        <b-tab title="General">
                            <b-card-text>General settings</b-card-text>
                        </b-tab>
                        <b-tab title="ZHL">
                            <b-card-text>ZHL settings</b-card-text>
                        </b-tab>
                        <b-tab title="VPM" disabled>
                            <b-card-text>ZHL settings</b-card-text>
                        </b-tab>
                    </b-tabs>
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
            // TODO: Implement on submit behaviour
        }

        resetForm() {
            this.syncFields();
        }
    }
</script>

<style scoped>

</style>