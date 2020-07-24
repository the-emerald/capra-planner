import {VuexModule, Module, Mutation} from 'vuex-module-decorators'
import {User} from "@/common/serde/user";
import {GeneralSettings, VPMSettings, ZHLSettings} from "@/common/serde/settings";
import {Tissue} from "@/common/serde/tissue";

@Module({
    namespaced: true,
    name: 'userInfo'
})
class UserInfo extends VuexModule {
    public user: User | null = null; // User that is selected

    public userZHLSettings: ZHLSettings | null = null;
    public userVPMSettings: VPMSettings | null = null;
    public userGeneralSettings: GeneralSettings | null = null;

    public userTissue: Tissue | null = null;

    @Mutation
    public updateSelectedUser(elem: User): void {
        this.user = elem;
    }

    @Mutation
    public resetUserAll(): void {
        this.user = null;
        this.userZHLSettings = null;
        this.userVPMSettings = null;
        this.userGeneralSettings = null;
    }

    @Mutation
    public updateZHLSettings(elem: ZHLSettings): void {
        this.userZHLSettings = elem;
    }

    @Mutation
    public updateVPMSettings(elem: VPMSettings): void {
        this.userVPMSettings = elem;
    }

    @Mutation
    public updateGeneralSettings(elem: GeneralSettings): void {
        this.userGeneralSettings = elem;
    }

    @Mutation
    public updateTissue(elem: Tissue): void {
        this.userTissue = elem;
    }

    get hasUserSelected() {
        return this.user != null;
    }
}

export default UserInfo