<template>
    <div class="login">
        <b-container>
            <b-row class="mb-2 mt-4 text-center" align-v="center">
                <b-col>
                    <h1>Capra Dive Planner</h1>
                </b-col>
            </b-row>
            <b-row class="mb-4 mt-5" align-v="center">
                <b-col></b-col>

                <b-col>
                    <!-- TODO: Add a placeholder for loading-->
                    <b-list-group-item
                            v-for="(users, index) in usersList"
                            :key="`users-${index}`"
                            href="#"
                            v-bind:class="{active: isSelected(index)}"
                            v-on:click="selectedUser = index">
                        {{users.name}}
                    </b-list-group-item>
                </b-col>

                <b-col></b-col>
            </b-row>
            <b-row class="mb-4 mt-2 text-center" align-v="center">
                <b-col></b-col>

                <b-col sm="1">
                    <b-button block variant="primary"><b-icon-arrow-right></b-icon-arrow-right></b-button>
                </b-col>

                <b-col></b-col>
            </b-row>
            <b-row class="mb-4 mt-2 text-center" align-v="center">
                <b-col></b-col>

                <b-col sm="1">
                    <b-button block><b-icon-person-plus-fill></b-icon-person-plus-fill></b-button>
                </b-col>

                <b-col></b-col>
            </b-row>
        </b-container>
    </div>
</template>

<script lang="ts">
    import {Component, Vue} from "vue-property-decorator";
    import {User, userFromResponse} from "@/common/serde/user"
    import {listAllUsers} from "@/common/routes";

    @Component({
        name: "Login"
    })
    export default class Login extends Vue {
        usersList: Array<User> = [];
        selectedUser = -1;

        isSelected(idx: number): boolean {
            return this.selectedUser == idx;
        }

        mounted() {
            listAllUsers()
            .then(r => {
                r.data.forEach(
                    // eslint-disable-next-line
                    (value: any) => {
                        this.usersList.push(userFromResponse(value))
                    }
                )
            })
        }
    }
</script>

<style scoped>

</style>