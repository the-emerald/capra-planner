import {VuexModule, Module, Mutation} from 'vuex-module-decorators'
import {DiveSegment} from "@/common/serde/dive_segment";
import {Gas} from "@/common/serde/gas";
import {Vue} from "vue-property-decorator";

// Interfaces for element data
export interface BottomSegmentElement {
    diveSegment: DiveSegment;
    gas: Gas;
}

export interface DecoGasElement {
    gas: Gas;
}

@Module({
    namespaced: true,
    name: 'plan'
})
class Plan extends VuexModule {
    public bottomSegments: Array<[boolean, BottomSegmentElement]> = [];
    public decoGases: Array<[boolean, DecoGasElement]> = [];
    public resultSync = false;

    @Mutation
    public updateResultSync(b: boolean) {
        this.resultSync = b;
    }

    @Mutation
    public pushBottomSegment(elem: [boolean, BottomSegmentElement]): void {
        this.bottomSegments.push(elem);
        this.resultSync = false;
    }

    @Mutation
    public updateBottomSegmentAtIndex(elem: [[boolean, BottomSegmentElement], number]): void {
        Vue.set(this.bottomSegments, elem[1], elem[0]);
        this.resultSync = false;

    }

    @Mutation
    public removeBottomSegmentAtIndex(idx: number): void {
        this.bottomSegments.splice(idx, 1);
        this.resultSync = false;

    }

    @Mutation
    public resetBottomSegments(): void {
        this.bottomSegments = [];
        this.resultSync = false;

    }

    @Mutation
    public replaceBottomSegments(to: Array<[boolean, BottomSegmentElement]>): void {
        this.bottomSegments = to;
        this.resultSync = false;
    }

    @Mutation
    public pushDecoGas(elem: [boolean, DecoGasElement]): void {
        this.decoGases.push(elem);
        this.resultSync = false;

    }

    @Mutation
    public updateDecoGasAtIndex(elem: [[boolean, DecoGasElement], number]): void {
        Vue.set(this.decoGases, elem[1], elem[0]);
        this.resultSync = false;

    }

    @Mutation
    public removeDecoGasAtIndex(idx: number): void {
        this.decoGases.splice(idx, 1);
        this.resultSync = false;
    }

    @Mutation
    public resetDecoGases(): void {
        this.decoGases = [];
        this.resultSync = false;
    }

    @Mutation
    public replaceDecoGases(to: Array<[boolean, DecoGasElement]>): void {
        this.decoGases = to;
        this.resultSync = false;
    }
}

export default Plan