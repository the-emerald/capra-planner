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
                                          :disabled="bottomSegments.length === 0"
                                          class="float-right" id="exec"
                                          size="sm"
                                          title="Plan"
                                          @click="onPlanButtonClick">
                                    Plan
                                </b-button>
                            </b-col>

                            <b-col>
                                <b-button block
                                          variant="success"
                                          :disabled="bottomSegments.length === 0"
                                          class="float-right" id="plan"
                                          size="sm"
                                          title="Execute"> <!-- TODO: Make a modal for execute -->
                                    Execute
                                </b-button>
                            </b-col>

                        </b-form-row>
                    </b-container>

                    <!--Relevant modals-->
                    <SurfaceIntervalModal :current-s-i="surfaceIntervalDuration" @submitted="onSIUpdateSubmitted"></SurfaceIntervalModal>
                </b-card-header>
                <b-container class="tbl">
                    <br>
                    <b-row>
                        <b-col>
                            <PlanTable :total-segments="planResults.segments"></PlanTable>
                        </b-col>
                    </b-row>
                    <b-row>
                        <b-col>
                            <GasTable :gas-plan="planResults.gas_used"></GasTable>
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
    import {GeneralSettings, ZHLSettings} from "@/common/serde/settings";
    import {User} from "@/common/serde/user";
    import {DiveSegment} from "@/common/serde/dive_segment";
    import {Gas} from "@/common/serde/gas";
    import {planDive, PlanDiveResponse} from "@/common/routes";
    import {makeErrorToast} from "@/common/toast";
    import {handleAxiosError} from "@/common/axios_error";
    import PlanTable from "@/components/plan/results/PlanTable.vue";
    import GasTable from "@/components/plan/results/GasTable.vue";

    const plan = namespace('Plan');
    const userInfo = namespace('UserInfo')

    @Component({
        components: {GasTable, PlanTable, SurfaceIntervalModal}
    })
    export default class Results extends Vue {
        planResults: PlanDiveResponse = {
            gas_used: [],
            segments: []
        };

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

        onPlanButtonClick() {
            const ascentRate = this.userGeneralSettings.ascent_rate;
            const descentRate = this.userGeneralSettings.descent_rate;

            const segments = this.bottomSegments
            .filter(x => x[0]) // Filter disabled segments
            .map(y => y[1]) // Remove boolean
            .map(function (z): [DiveSegment, Gas] { // Destructure
                return [z.diveSegment, z.gas]
            })
            .map(function (a): [DiveSegment, Gas] { // Add ascent rate and descent rate
                a[0].ascent_rate = ascentRate;
                a[0].descent_rate = descentRate;

                return [a[0], a[1]];
            })

            const gases = this.decoGases
            .filter(x => x[0]) // Filter disabled gases
            .map(y => y[1].gas) // Map element

            planDive(
                this.user,
                this.selectedAlgo,
                this.surfaceIntervalDuration,
                segments,
                gases,
            )
            .then(r => {
                this.planResults = r.data;
                console.log("OK!");
            })
            .catch((error) => {
                makeErrorToast(this, handleAxiosError(error));
            })
        }

        @plan.State
        public bottomSegments!: Array<[boolean, BottomSegmentElement]>;

        @plan.State
        public decoGases!: Array<[boolean, DecoGasElement]>;

        @userInfo.State
        public user!: User;

        @userInfo.State
        public userZHLSettings!: ZHLSettings;

        // @userInfo.State
        // public userVPMSettings!: VPMSettings;

        @userInfo.State
        public userGeneralSettings!: GeneralSettings;

    }


</script>

<style scoped>
    .tbl {
        height: 75vh;
        overflow-y: auto;
    }
</style>