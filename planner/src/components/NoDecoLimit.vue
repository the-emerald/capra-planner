<template>
    <div>
        <label>
            <input type="number" placeholder="Depth" v-model.number="depth" @blur="calculateNDL">
        </label>
        <p>Result:</p>
        <p>{{ndl.timeRemaining}}</p>
    </div>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import axios from 'axios';
import {ndlFromResponse, NDLInput, NDLOutput} from "@/serde/ndl";

@Component
export default class NoDecoLimit extends Vue {
    depth = 0;
    ndl: NDLOutput = {
        decoReached: false,
        infinite: false,
        timeRemaining: 0,
    };

    calculateNDL() {
        if (this.depth == 0) {
            return;
        }

        const data: NDLInput = {
            depth: this.depth
        };

        axios.post('http://localhost:8000/ndl',
            data,
            {
                headers: {'content-type' : 'application/json'}
            })
        .then(
            response => {this.ndl = ndlFromResponse(response)}
        )
        .catch(
            error => console.log(error)
        );
    }

}
</script>

<style scoped>

</style>