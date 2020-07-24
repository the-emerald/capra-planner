import Vue from 'vue'
import Vuex from 'vuex'

import Plan from "@/store/plan";
import UserInfo from "@/store/userinfo";

Vue.use(Vuex);

const store = new Vuex.Store({
    modules: {
        Plan,
        UserInfo
    }
});

export default store
