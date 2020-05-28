import Vue from 'vue'
import App from './App.vue'
import {BootstrapVue, IconsPlugin} from "bootstrap-vue";

// Install BootstrapVue
Vue.use(BootstrapVue);
// Install BootstrapVue icon components
Vue.use(IconsPlugin);

Vue.config.productionTip = false;

new Vue({
    render: h => h(App),
}).$mount('#app');
