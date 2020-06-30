<template>
    <div class="login">
        <b-container>
            <b-row class="mb-3 mt-4 text-center" align-v="center">
                <b-col>
                    <h1>Capra Dive Planner</h1>
                </b-col>
            </b-row>
            <b-row class="mb-4 mt-4" align-v="center">
                <b-col></b-col>

                <b-col>
                    <b-list-group-item v-if="usersList.length === 0">
                        <div class="d-flex justify-content-center mb-2 mt-2">
                            <b-spinner></b-spinner>
                        </div>
                    </b-list-group-item>
                    <b-list-group-item
                            v-else
                            v-for="(user, index) in usersList"
                            :key="`users-${index}`"
                            href="#"
                            v-bind:class="{active: isSelectedIdx(index)}"
                            v-on:click="onSelect(index, user.id)">
                        {{user.name}}
                    </b-list-group-item>
                </b-col>

                <b-col></b-col>
            </b-row>
            <b-row class="mb-4 mt-2 text-center" align-v="center">
                <b-col></b-col>

                <b-col sm="1">
                    <b-button block variant="primary"
                              v-bind:disabled="isSelectedIdx(-1)"
                              @click="loginButtonPressed">
                        <b-icon-arrow-right></b-icon-arrow-right>
                    </b-button>
                </b-col>

                <b-col></b-col>
            </b-row>
            <b-row class="mb-4 mt-2 text-center" align-v="center">
                <b-col></b-col>

                <b-col sm="1">
                    <b-button block @click="$bvModal.show('login-new-modal')">
                        <b-icon-person-plus-fill></b-icon-person-plus-fill>
                    </b-button>
                </b-col>

                <b-col></b-col>
            </b-row>
        </b-container>
        <LoginNewModal @submitted="onLoginNewSubmitted" :existing-users="usersList"></LoginNewModal>
    </div>
</template>

<script lang="ts">
    import {Component, Vue} from "vue-property-decorator";
    import {User} from "@/common/serde/user"
    import {getUser, listAllUsers, newUser} from "@/common/routes";
    import {namespace} from "vuex-class";
    import router from "@/router";
    import LoginNewModal from "@/components/login/LoginNewModal.vue";
    import {makeErrorToast} from "@/common/toast";
    import {handleAxiosError} from "@/common/axios_error";
    import {Tissue} from "@/common/serde/tissue";
    import {GeneralSettings, VPMSettings, ZHLSettings} from "@/common/serde/settings";

    const userInfo = namespace('UserInfo');

    @Component({
        name: "Login",
        components: {LoginNewModal}
    })
    export default class Login extends Vue {
        usersList: Array<User> = [];

        selected = -1;
        selectedId = -1;

        isSelectedIdx(idx: number): boolean {
            return this.selected == idx;
        }

        onSelect(idx: number, id: number) {
            this.selected = idx;
            this.selectedId = id;
        }

        loginButtonPressed() {
            // Update store
            getUser(this.selectedId)
            .then(r => {
                this.updateSelectedUser(r.data.user);
                this.updateZHLSettings(r.data.zhl_settings);
                this.updateVPMSettings(r.data.vpm_settings);
                this.updateGeneralSettings(r.data.general_settings);
                this.updateTissue(r.data.tissue);
                router.push({
                    name: "plan"
                })
            })
            .catch((error) => {
                makeErrorToast(this, handleAxiosError(error))
            });
        }

        onLoginNewSubmitted(name: string) {
            newUser(name)
            .then(() => {
                this.refreshUsersList();
            });
        }

        @userInfo.State
        public selectedUser!: number;

        @userInfo.Mutation
        public updateSelectedUser!: (elem: User) => void;

        @userInfo.Mutation
        public updateZHLSettings!: (elem: ZHLSettings) => void;

        @userInfo.Mutation
        public updateVPMSettings!: (elem: VPMSettings) => void;

        @userInfo.Mutation
        public updateGeneralSettings!: (elem: GeneralSettings) => void;

        @userInfo.Mutation
        public updateTissue!: (elem: Tissue) => void;

        mounted() {
            this.refreshUsersList();
        }

        resetSelected() {
            this.selected = -1; // Invalidate previous selected
            this.selectedId = -1;
        }

        refreshUsersList() {
            this.usersList = [];
            this.resetSelected();
            listAllUsers()
                .then(r => {
                    this.usersList = r.data;
                })
                .catch((error) => {
                    makeErrorToast(this, handleAxiosError(error));
                });
        }
    }
</script>

<style scoped>

</style>