<template>
    <div>
        <b-table striped :items="unpackedTotalSegments" :fields="fields">
            <template v-slot:head(depth)="">
                Depth (m)
            </template>
        </b-table>
    </div>
</template>

<script lang="ts">
    import {Component, Prop, Vue} from "vue-property-decorator";
    import {Gas} from "@/common/serde/gas";
    import {DiveSegment} from "@/common/serde/dive_segment";
    import {prettyFromMilliseconds} from "@/common/time";
    import {displayGas} from "@/common/display";

    @Component({

    })
    export default class PlanTable extends Vue {
        @Prop() private totalSegments!: Array<[DiveSegment, Gas]>
        fields: Array<string> = ["segment_type", "depth", "time", "runtime", "gas"]

        get unpackedTotalSegments() {
            return this.totalSegments
            .map((trt => (elem: [DiveSegment, Gas]) => {
                trt += elem[0].time;
                return {
                    segment_type: elem[0].segment_type,
                    depth: elem[0].start_depth == elem[0].end_depth ? elem[0].start_depth.toString() : `${elem[0].start_depth} > ${elem[0].end_depth}`,
                    time: prettyFromMilliseconds(elem[0].time),
                    runtime: prettyFromMilliseconds(trt),
                    gas: displayGas(elem[1])
                }
            })(0))
        }
    }

</script>

<style scoped>

</style>