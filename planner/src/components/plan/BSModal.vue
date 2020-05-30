<template>
    <div>
        <b-modal id="bottom-segment-modal" title="Add bottom segment">
            <b-container>
                <!--Depth-->
                <b-form-row>
                    <b-col style="margin-top: 0.4rem" sm="4">
                        Depth
                    </b-col>
                    <b-col>
                        <b-form-input placeholder="m" type="number" v-model.number="depth" required>
                        </b-form-input>
                    </b-col>
                </b-form-row>
                <br>

                <!--Time-->
                <b-form-row>
                    <b-col style="margin-top: 0.4rem" sm="4">
                        Time
                    </b-col>
                    <b-col>
                        <b-form-input placeholder="mm" type="number" v-model.number="timeMin" required>
                        </b-form-input>
                    </b-col>
                    <div style="margin-top: 0.4rem">:</div>
                    <b-col>
                        <b-form-input placeholder="ss" type="number" v-model.number="timeSec" required>
                        </b-form-input>
                    </b-col>
                </b-form-row>
                <br>

                <!--Mix-->
                <b-form-row>
                    <b-col style="margin-top: 0.4rem" sm="4">
                        Mix
                    </b-col>
                    <b-col>
                        <b-form-input placeholder="O2" type="number" v-model.number="o2" required>
                        </b-form-input>
                    </b-col>
                    <div style="margin-top: 0.4rem">/</div>
                    <b-col>
                        <b-form-input placeholder="He" type="number" v-model.number="he" required>
                        </b-form-input>
                    </b-col>
                </b-form-row>
            </b-container>
        </b-modal>
    </div>
</template>

<script lang="ts">
    import {Component, Vue} from "vue-property-decorator";

    @Component
    export default class BSModal extends Vue {
        depth = null;
        timeMin = null;
        timeSec = null;
        o2 = null;
        he = null;
    }
</script>

<style scoped>

</style>