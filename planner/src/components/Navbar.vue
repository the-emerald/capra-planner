<template>
    <div>
        <b-navbar toggleable="lg" type="dark" variant="dark">
            <b-navbar-brand href="#">🐐</b-navbar-brand>

            <b-navbar-toggle target="nav-collapse"></b-navbar-toggle>

            <b-collapse id="nav-collapse" is-nav>
                <b-navbar-nav>
                    <b-nav-item to="plan">Planner</b-nav-item>
                    <b-nav-item to="history" disabled>History</b-nav-item>
                </b-navbar-nav>

                <b-navbar-nav class="ml-auto">
                    <b-nav-item-dropdown right>
                        <template v-slot:button-content>
                            {{user.name}}
                        </template>
                        <b-dropdown-item href="#" @click="$bvModal.show('settings-modal')">
                            Settings
                        </b-dropdown-item>
                        <b-dropdown-item href="#" @click="logout">
                            Sign Out
                        </b-dropdown-item>

                        <b-dropdown-divider></b-dropdown-divider>

                        <b-dropdown-item href="#" @click="$bvModal.show('about-modal')">
                            About
                        </b-dropdown-item>
                        <b-dropdown-item href="#" @click="bugReport">
                            Report a bug
                        </b-dropdown-item>
                    </b-nav-item-dropdown>
                </b-navbar-nav>
            </b-collapse>
        </b-navbar>
        <SettingsModal></SettingsModal>
        <AboutModal></AboutModal>
    </div>
</template>

<script lang="ts">
    import {Component, Vue} from "vue-property-decorator";
    import {namespace} from "vuex-class";
    import {User} from "@/common/serde/user";
    import SettingsModal from "@/components/SettingsModal.vue";
    import router from "@/router";
    import AboutModal from "@/components/AboutModal.vue";

    const userInfo = namespace('UserInfo');
    @Component({
        components: {AboutModal, SettingsModal}
    })
    export default class Navbar extends Vue {
        @userInfo.State
        public user!: User | null;

        @userInfo.Mutation
        public resetUserAll!: () => void;

        logout() {
            this.resetUserAll();
            router.push({
                name: "login"
            });
        }

        bugReport() {
            window.open("https://github.com/the-emerald/capra-planner/issues", '_blank');
        }
    }

</script>

<style scoped>

</style>