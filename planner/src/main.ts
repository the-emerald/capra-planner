import Vue from 'vue'
import App from './App.vue'
import {BootstrapVue, IconsPlugin} from "bootstrap-vue";
import 'bootstrap/dist/css/bootstrap.css'
import 'bootstrap-vue/dist/bootstrap-vue.css'
import router from './router'

// Install BootstrapVue
Vue.use(BootstrapVue);
// Install BootstrapVue icon components
Vue.use(IconsPlugin);

Vue.config.productionTip = false;

new Vue({
    router,
    render: h => h(App)
}).$mount('#app');
