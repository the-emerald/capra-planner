import {VuexModule, Module, Mutation} from 'vuex-module-decorators'
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
    public pushBottomSegment(elem: [diveSegment, gas]): void {
        this.bottomSegments.push(elem);
    }

    @Mutation
    public pushDecoGas(elem: [gas, number?]): void {
        this.decoGases.push(elem);
    }
}

export default Plan