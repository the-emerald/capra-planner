import {VuexModule, Module, Mutation, Action} from 'vuex-module-decorators'
import {diveSegment} from "@/common/dive_segment";
import {gas} from "@/common/gas";

@Module({
    namespaced: true,
    name: 'plan'
})
class Plan extends VuexModule {
    public bottomSegments: Array<[diveSegment, gas]> = [];
    public decoGases: Array<[gas, number?]> = [];

    @Mutation
    public setBottomSegments(to: Array<[diveSegment, gas]>): void {
        this.bottomSegments = to;
    }

    @Mutation
    public setDecoGases(to: Array<[gas, number?]>): void {
        this.decoGases = to;
    }

    @Action
    public updateBottomSegments(updated: Array<[diveSegment, gas]>): void {
        this.context.commit('setBottomSegments', updated)
    }

    @Action
    public updateDecoGases(updated: Array<[gas, number?]>): void {
        this.context.commit('setDecoGases', updated)
    }
}

export default Plan