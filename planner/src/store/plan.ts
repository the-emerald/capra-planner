import {VuexModule, Module, Mutation} from 'vuex-module-decorators'
import {diveSegment} from "@/common/dive_segment";
import {gas} from "@/common/gas";
import {Vue} from "vue-property-decorator";

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
    public updateBottomSegmentAtIndex(elem: [[diveSegment, gas], number]): void {
        Vue.set(this.bottomSegments, elem[1], elem[0]);
    }

    @Mutation
    public removeBottomSegmentAtIndex(idx: number): void {
        this.bottomSegments.splice(idx, 1);
    }

    @Mutation
    public resetBottomSegments(): void {
        this.bottomSegments = [];
    }

    @Mutation
    public replaceBottomSegments(to: Array<[diveSegment, gas]>): void {
        this.bottomSegments = to;
    }

    @Mutation
    public pushDecoGas(elem: [gas, number?]): void {
        this.decoGases.push(elem);
    }

    @Mutation
    public updateDecoGasAtIndex(elem: [[gas, number?], number]): void {
        Vue.set(this.decoGases, elem[1], elem[0]);
    }

    @Mutation
    public removeDecoGasAtIndex(idx: number): void {
        this.decoGases.splice(idx, 1);
    }

    @Mutation
    public resetDecoGases(): void {
        this.decoGases = [];
    }

    @Mutation
    public replaceDecoGases(to: Array<[gas, number?]>): void {
        this.decoGases = to;
    }
}

export default Plan