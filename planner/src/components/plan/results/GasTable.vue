<template>
    <div>
        <b-table striped :items="unpackedGasPlan" :fields="fields">
            <template v-slot:head(amount)="">
                Quantity (ltr)
            </template>
        </b-table>
    </div>
</template>

<script lang="ts">
    import {Component, Prop, Vue} from "vue-property-decorator";
    import {Gas} from "@/common/serde/gas";
    import {displayGas} from "@/common/display";

    @Component({

    })
    export default class GasTable extends Vue {
        @Prop() private gasPlan!: Array<[Gas, number]>;
        fields: Array<string> = ["gas", "amount"];

        get unpackedGasPlan() {
            return this.gasPlan
            .sort(function (a, b) {
                return b[1] - a[1]
            })
            .map(function (elem) {
                return {
                    gas: displayGas(elem[0]),
                    amount: elem[1]
                }
            });
        }
    }

</script>

<style scoped>

</style>