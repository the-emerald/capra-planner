<template>
    <div>
        <b-modal id="login-new-modal" title="Add new user" hide-footer>
            <ValidationObserver ref="observer" v-slot="{passes}">
                <b-form @submit.prevent="passes(onSubmit)" @reset="resetForm">
                    <b-container>
                        <!--Name-->
                        <b-form-row>
                            <b-col style="margin-top: 0.4rem" sm="4">
                                Name
                            </b-col>
                            <b-col>
                                <ValidationProvider name="name" :rules="{required: true, uniqueName: {val: existingUsers}}" v-slot="{valid, errors}">
                                    <b-form-input
                                        type="text"
                                        v-model.number="userName"
                                        :state="errors[0] ? false : (valid ? true : null)"
                                    ></b-form-input>
                                </ValidationProvider>
                            </b-col>
                        </b-form-row>
                    </b-container>
                    <br>
                    <div class="modal-footer">
                        <b-button type="reset" variant="danger">Reset</b-button>
                        <b-button type="submit" variant="primary">Add</b-button>
                    </div>
                </b-form>
            </ValidationObserver>
        </b-modal>
    </div>
</template>

<script lang="ts">
    import {Component, Prop, Vue} from "vue-property-decorator";
    import {ValidationObserver, ValidationProvider} from "vee-validate"
    import "@/common/validation"
    import {User} from "@/common/serde/user";

    @Component({
        components: {
            ValidationProvider,
            ValidationObserver
        }
    })
    export default class BSNewModal extends Vue {
        $refs!: {
            observer: InstanceType<typeof ValidationObserver>;
        };

        @Prop() private existingUsers!: Array<User>;


        userName = '';

        onSubmit() {
            this.$emit(
                'submitted',
                this.userName
            );

            this.resetForm();
            this.$bvModal.hide('login-new-modal')
        }

        resetForm() {
            this.userName = '';
            requestAnimationFrame(() => {
                this.$refs.observer.reset();
            });
        }
    }
</script>

<style scoped>

</style>