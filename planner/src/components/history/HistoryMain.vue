<template>
  <div>
    <b-container>
      <b-form @submit="onSubmit">
        <b-form-row inline>
          <b-col align-self="center">
            <b-form-select
                required
                id="planTypes"
                v-model="selectedPlanTypes"
                :options="planTypeOptions"
            ></b-form-select>
          </b-col>
          <b-col align-self="center">
            <b-form-datepicker value-as-date v-model="datetimeStart"></b-form-datepicker>
          </b-col>
          —
          <b-col align-self="center">
            <b-form-datepicker value-as-date v-model="datetimeEnd"></b-form-datepicker>
          </b-col>
          <b-col align-self="center">
            <b-button type="submit" variant="primary">Query</b-button>
          </b-col>
        </b-form-row>
      </b-form>
      <br>
      <b-row>
        <b-col v-if="showHistoryTable">
          todo: history table component
        </b-col>
        <b-col v-else>
          Crickets lol
        </b-col>
      </b-row>
    </b-container>
  </div>
</template>

<script lang="ts">
import {Component, Vue} from "vue-property-decorator";
import {PlanType} from "@/common/serde/plan_type";
import {getHistory} from "@/common/routes";
import {namespace} from "vuex-class";
import {User} from "@/common/serde/user";
import {Dive} from "@/common/serde/dive";
import {handleAxiosError} from "@/common/axios_error";
import {makeErrorToast} from "@/common/toast";

const userInfo = namespace('UserInfo')

@Component({
        name: "HistoryMain",
        components: {}
      })

  export default class HistoryMain extends Vue {
    selectedPlanTypes = [];
    datetimeStart: Date | null = null;
    datetimeEnd: Date | null = null;

    planTypeOptions = [
        { value: [], text: "Show only...", disabled: true },
        { value: [PlanType.PLAN], text: "Plans" },
        { value: [PlanType.EXECUTE], text: "Executed" },
        { value: [PlanType.PLAN, PlanType.EXECUTE], text: "Plans + Executed" }
      ]

    history: Array<[number, Dive]> = [];

    contactServerForHistory() {
      getHistory(
          this.user,
          this.selectedPlanTypes,
          (this.datetimeStart == null || this.datetimeEnd == null) ? undefined : [this.datetimeStart, this.datetimeEnd]
      )
      .then(r => {
        this.history = r.data;
        console.log(this.history.length);

      })
      .catch((error) => {
        makeErrorToast(this, handleAxiosError(error));
      });
    }

    onSubmit() {
      console.log("onSubmit called");
      this.contactServerForHistory();
    }

    get showHistoryTable(): boolean {
      return this.history.length !== 0;
    }

    @userInfo.State
    public user!: User;
  }
</script>

<style scoped>

</style>