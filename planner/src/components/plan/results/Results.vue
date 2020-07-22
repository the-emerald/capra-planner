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
                                <b-dropdown size="sm" class="float-right">
                                    <template v-slot:button-content>
                                        {{selectedAlgo}}
                                    </template>
                                    <b-dropdown-item @click="updateSelectedAlgo('ZHL16')">
                                        ZHL-16
                                    </b-dropdown-item>
                                    <b-dropdown-item @click="updateSelectedAlgo('VPM')" disabled>
                                        VPM
                                    </b-dropdown-item>
                                </b-dropdown>
                            </b-col>
                            <b-col>
                                <b-button block
                                          variant="info"
                                          class="float-right" id="si"
                                          size="sm"
                                          title="SI"
                                          @click="$bvModal.show('surface-interval-modal')">
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
                    <SurfaceIntervalModal :current-s-i="surfaceIntervalDuration" @submitted="onSIUpdateSubmitted"></SurfaceIntervalModal>
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
    import {namespace} from "vuex-class";
    import {BottomSegmentElement, DecoGasElement} from "@/store/plan";
    import {Algorithm} from "@/common/serde/algorithm";
    import SurfaceIntervalModal from "@/components/plan/results/SurfaceIntervalModal.vue";

    const plan = namespace('Plan');
    const userInfo = namespace('UserInfo')

    @Component({
        components: {SurfaceIntervalModal}
    })
    export default class Results extends Vue {
        selectedAlgo: Algorithm = Algorithm.ZHL16;

        updateSelectedAlgo(a: string) {
            if (a == 'ZHL16') {
                this.selectedAlgo = Algorithm.ZHL16;
            }
            else if (a == 'VPM') {
                this.selectedAlgo = Algorithm.VPM;
            }
        }

        surfaceIntervalDuration = 0;

        onSIUpdateSubmitted(value: number) {
            this.surfaceIntervalDuration = value;
        }

        @plan.State
        public bottomSegments!: Array<[boolean, BottomSegmentElement]>;

        @plan.State
        public decoGases!: Array<[boolean, DecoGasElement]>;

    }


</script>

<style scoped>

</style>