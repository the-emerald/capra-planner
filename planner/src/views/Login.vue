<template>
    <div class="login">
        <b-container>
            <b-row class="mb-2 mt-4">
                <b-col>
                    <h1 class="text-center">Capra Dive Planner</h1>
                </b-col>
            </b-row>
            <b-row class="mb-4 mt-5">
                <b-col>
                    <b-list-group-item
                            v-for="(users, index) in usersList"
                            :key="`users-${index}`"
                            href="#"
                            v-bind:class="{active: isSelected(index)}"
                            v-on:click="selectedUser = index">
                        {{users.name}}
                    </b-list-group-item>
<!--                    <p class="text-center">Maybe put some divers here?</p>-->
                </b-col>
            </b-row>
            <b-row class="mb-4 mt-5">
                <b-col>
                    <p class="text-center">And a few buttons here!</p>
                </b-col>
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