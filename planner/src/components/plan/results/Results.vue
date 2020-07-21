<template>
    <div>
        <b-card-group>
            <b-card no-body>
                <b-card-header>
                    <b-container class="p-0">
                        <b-form-row>
                            <b-col sm="6" style="margin-top: 0.2rem">
                                Results
                            </b-col>
                            <!--UI buttons-->
                            <b-col>
                                <b-button block
                                          variant="primary"
                                          class="float-right" id="si"
                                          size="sm"
                                          title="SI">
                                    SI
                                </b-button>
                            </b-col>

                            <b-col>
                                <b-button block
                                          variant="primary"
                                          class="float-right" id="exec"
                                          size="sm"
                                          title="Execute">
                                    Plan
                                </b-button>
                            </b-col>

                            <b-col>
                                <b-button block
                                          variant="success"
                                          class="float-right" id="plan"
                                          size="sm"
                                          title="Plan">
                                    Execute
                                </b-button>
                            </b-col>

                        </b-form-row>
                    </b-container>

                    <!--Relevant modals-->
                </b-card-header>
                <b-container>
                    <br>
                    <b-row>
                        <b-col>
                            <p>Dive plan will appear here...</p>
                        </b-col>
                    </b-row>
                    <b-row>
                        <b-col>
                            <p>Gas plan will appear here...</p>
                        </b-col>
                    </b-row>
                </b-container>
            </b-card>
        </b-card-group>
    </div>
</template>

<script lang="ts">
    import {Component, Vue} from "vue-property-decorator";

    @Component({})
    export default class Results extends Vue {

    }


</script>

<style scoped>
    .list_group {
        height: 75vh;
        overflow-y: auto;
    }
</style>