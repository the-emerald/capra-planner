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
                                    SI ({{hourMinuteFromMilliseconds}})
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
                                          @click="$bvModal.show('execute-dive-modal')"
                                          title="Execute"> <!-- TODO: Make a modal for execute -->
                                    Execute
                                </b-button>
                            </b-col>

                        </b-form-row>
                    </b-container>

                    <!--Relevant modals-->
                    <SurfaceIntervalModal :current-s-i="surfaceIntervalDuration" @submitted="onSIUpdateSubmitted"></SurfaceIntervalModal>
                    <ExecuteDiveModal @submitted="onExecuteModalConfirmed"></ExecuteDiveModal>
                </b-card-header>
                <b-container class="tbl">
                    <br>
                    <b-row v-if="showModifiedAlert">
                        <b-col>
                            <b-alert show variant="danger">
                                <b>Plan modified:</b> Results shown below do not reflect the current parameters.
                            </b-alert>
                        </b-col>
                    </b-row>
                    <template v-if="showTables">
                        <b-row>
                            <b-col>
                                <PlanTable :total-segments="planResults.segments"></PlanTable>
                            </b-col>
                        </b-row>
                        <b-row>
                            <b-col>
                                <GasTable :gas-plan="planResults.gasUsed"></GasTable>
                            </b-col>
                        </b-row>
                    </template>
                    <template v-else>
                        <b-row>
                            <b-col>
                                <b-alert show variant="info">
                                    To begin, construct your dive using the sections on your left, then click "Plan"
                                    or "Execute".
                                </b-alert>
                            </b-col>
                        </b-row>
                    </template>
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
    import {getDivePlan, PlanDiveResponse} from "@/common/routes";
    import {makeErrorToast, makeSuccessToast} from "@/common/toast";
    import {handleAxiosError} from "@/common/axios_error";
    import PlanTable from "@/components/plan/results/PlanTable.vue";
    import GasTable from "@/components/plan/results/GasTable.vue";
    import ExecuteDiveModal from "@/components/plan/results/ExecuteDiveModal.vue";
    import {PlanType} from "@/common/serde/plan_type";
    import {prettyHourMinuteFromMilliseconds} from "@/common/time";

    const plan = namespace('Plan');
    const userInfo = namespace('UserInfo')

    @Component({
        components: {ExecuteDiveModal, GasTable, PlanTable, SurfaceIntervalModal}
    })
    export default class Results extends Vue {
        planResults: PlanDiveResponse = {
            gasUsed: [],
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

        onExecuteModalConfirmed() {
            this.contactServerForPlan(PlanType.EXECUTE);
            makeSuccessToast(this, "Executed dive.")
        }

        onPlanButtonClick() {
            this.contactServerForPlan(PlanType.PLAN);
        }

        contactServerForPlan(planType: PlanType) {
            const ascentRate = this.userGeneralSettings.ascentRate;
            const descentRate = this.userGeneralSettings.descentRate;

            const segments = this.bottomSegments
            .filter(x => x[0]) // Filter disabled segments
            .map(y => y[1]) // Remove boolean
            .map(function (z): [DiveSegment, Gas] { // Destructure
                return [z.diveSegment, z.gas]
            })
            .map(function (a): [DiveSegment, Gas] { // Add ascent rate and descent rate
                a[0].ascentRate = ascentRate;
                a[0].descentRate = descentRate;

                return [a[0], a[1]];
            })

            const gases = this.decoGases
            .filter(x => x[0]) // Filter disabled gases
            .map(y => y[1].gas) // Map element

            getDivePlan(
                this.user,
                planType,
                this.selectedAlgo,
                this.surfaceIntervalDuration,
                segments,
                gases,
            )
            .then(r => {
                this.planResults = r.data;
                this.updateResultSync(true);
            })
            .catch((error) => {
                makeErrorToast(this, handleAxiosError(error));
            })
        }

        get showModifiedAlert(): boolean {
            return !this.resultSync && !(this.planResults.segments.length === 0);
        }

        get showTables(): boolean {
            return this.planResults.segments.length !== 0
        }

        @plan.State
        public bottomSegments!: Array<[boolean, BottomSegmentElement]>;

        @plan.State
        public decoGases!: Array<[boolean, DecoGasElement]>;

        @plan.State
        public resultSync!: boolean;

        @plan.Mutation
        public updateResultSync!: (b: boolean) => void;

        @userInfo.State
        public user!: User;

        @userInfo.State
        public userZHLSettings!: ZHLSettings;

        // @userInfo.State
        // public userVPMSettings!: VPMSettings;

        @userInfo.State
        public userGeneralSettings!: GeneralSettings;

        get hourMinuteFromMilliseconds(): string {
            return prettyHourMinuteFromMilliseconds(this.surfaceIntervalDuration);
        }
    }


</script>

<style scoped>
    .tbl {
        height: 75vh;
        overflow-y: auto;
    }
</style>