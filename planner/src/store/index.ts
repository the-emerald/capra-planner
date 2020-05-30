import Vue from 'vue'
import Vuex from 'vuex'

import Plan from "@/store/plan";

Vue.use(Vuex);

const store = new Vuex.Store({
    modules: {
        Plan
    }
});

export default store
