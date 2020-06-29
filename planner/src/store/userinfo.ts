import {VuexModule, Module, Mutation} from 'vuex-module-decorators'

@Module({
    namespaced: true,
    name: 'userInfo'
})
class UserInfo extends VuexModule {
    public selectedUser = -1; // Default value is -1

    @Mutation
    public updateSelectedUser(elem: number): void {
        this.selectedUser = elem;
    }

    @Mutation
    public resetSelectedUser(): void {
        this.selectedUser = -1;
    }

    get hasUserSelected() {
        return this.selectedUser != -1;
    }
}

export default UserInfo