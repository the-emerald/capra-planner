<template>
    <div>
        <b-modal id="settings-modal" title="Settings" @shown="syncFields" hide-footer>

        </b-modal>
    </div>
</template>

<script lang="ts">
    import Component from "vue-class-component";
    import {Vue} from "vue/types/vue";
    import {namespace} from "vuex-class";
    import {GeneralSettings, VPMSettings, ZHLSettings} from "@/common/serde/settings";

    const userInfo = namespace('UserInfo');

    @Component
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
    }
</script>

<style scoped>

</style>