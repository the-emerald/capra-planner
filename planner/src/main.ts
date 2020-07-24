import Vue from 'vue'
import App from './App.vue'
import {BootstrapVue, IconsPlugin} from "bootstrap-vue";
import 'bootstrap/dist/css/bootstrap.css'
import 'bootstrap-vue/dist/bootstrap-vue.css'
import router from './router'
import store from './store'
import axios from 'axios'

// Install BootstrapVue
Vue.use(BootstrapVue);
// Install BootstrapVue icon components
Vue.use(IconsPlugin);

// Set axios defaults
axios.defaults.baseURL = "http://localhost:8000";
axios.defaults.headers.post['content-type'] = 'application/json';
axios.defaults.timeout = 5000; // 5 seconds

Vue.config.productionTip = false;

new Vue({
    router,
    store,
    render: h => h(App)
}).$mount('#app');
