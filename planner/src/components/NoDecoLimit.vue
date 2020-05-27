<template>
    <div>
        <label>
            <input placeholder="Depth" v-model="depth" @blur="calculateNDL">
        </label>
        <p>No Deco Time: {{ndl}}</p>
    </div>
</template>

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import axios from 'axios';

@Component
export default class NoDecoLimit extends Vue {
    depth = 0;
    ndl = "";

    calculateNDL() {
        if (this.depth == 0) {
            return;
        }

        axios.post('http://localhost:8000/ndl',
            {
                depth: Number(this.depth)
            },
            {
                headers: {'content-type' : 'application/json'}
            })
        .then(
            response => {
                if (response.data == 18446744073709551615) {
                    this.ndl = "Unlimited"
                }
                else if (response.data == "deco") {
                    this.ndl = "Deco Reached"
                }
                else {
                    this.ndl = response.data
                }
            }
        )
        .catch(
            error => console.log(error)
        );
    }

}
</script>

<style scoped>

</style>