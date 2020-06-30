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
    import {GasPlanSettings, VPMSettings, ZHLSettings} from "@/common/serde/settings";

    const userInfo = namespace('UserInfo');

    @Component
    export default class SettingsModal extends Vue {
        @userInfo.State
        public selectedUser!: number;

        @userInfo.State
        public userZHLSettings!: ZHLSettings;

        gfl = '';
        gfh = '';
        ascentRate = '';
        descentRate = '';

        @userInfo.State
        public userVPMSettings!: VPMSettings;

        // Nothing to see here

        @userInfo.State
        public userGasPlanSettings!: GasPlanSettings;

        sacBottom = '';
        sacDeco = '';

        syncFields() {
            // ZHL
            this.gfl = this.userZHLSettings.gfl.toString();
            this.gfh = this.userZHLSettings.gfh.toString();
            this.ascentRate = this.userZHLSettings.ascent_rate.toString();
            this.descentRate = this.userZHLSettings.descent_rate.toString();

            // Gas Plan
            this.sacBottom = this.userGasPlanSettings.sac_bottom.toString();
            this.sacDeco = this.userGasPlanSettings.sac_deco.toString();
        }
    }
</script>

<style scoped>

</style>