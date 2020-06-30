<template>
    <div>
        <b-navbar toggleable="lg" type="dark" variant="dark">
            <b-navbar-brand href="#">🐐</b-navbar-brand>

            <b-navbar-toggle target="nav-collapse"></b-navbar-toggle>

            <b-collapse id="nav-collapse" is-nav>
                <b-navbar-nav>
                    <b-nav-item to="plan">Planner</b-nav-item>
                    <b-nav-item to="test">History</b-nav-item>
                </b-navbar-nav>

                <!-- Right aligned nav items -->
                <b-navbar-nav class="ml-auto">
                    <b-nav-item-dropdown right>
                        <!-- Using 'button-content' slot -->
                        <template v-slot:button-content>
                            {{user.name}}
                        </template>
                        <b-dropdown-item href="#" @click="$bvModal.show('settings-modal')">
                            Settings
                        </b-dropdown-item>
                        <b-dropdown-item href="#">Sign Out</b-dropdown-item>
                    </b-nav-item-dropdown>
                </b-navbar-nav>
            </b-collapse>
        </b-navbar>
        <SettingsModal></SettingsModal>
    </div>
</template>

<script lang="ts">
    import {Component, Vue} from "vue-property-decorator";
    import {namespace} from "vuex-class";
    import {User} from "@/common/serde/user";
    import SettingsModal from "@/components/SettingsModal.vue";

    const userInfo = namespace('UserInfo');
    @Component({
        components: {SettingsModal}
    })
    export default class Navbar extends Vue {
        @userInfo.State
        public user!: User | null;

        @userInfo.Mutation
        public resetSelectedUser!: () => void;
    }

</script>

<style scoped>

</style>