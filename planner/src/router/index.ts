import Vue from 'vue'
import VueRouter, {Route, RouteConfig} from 'vue-router'
import Plan from '../views/Plan.vue'
import Login from '../views/Login.vue'
import store from "@/store";
import History from "@/views/History.vue";

Vue.use(VueRouter);

// eslint-disable-next-line
function requireLogin(to: Route, from: Route, next: any) {
    const selected: boolean = store.getters['UserInfo/hasUserSelected'];
    if (!selected) {
        next({
            name: "login"
        })
    }
    else {
        next()
    }
}

const routes: Array<RouteConfig> = [
    {
        path: '/',
        redirect: {
            name: "login"
        }
    },
    {
        path: '/login',
        name: 'login',
        component: Login,
        beforeEnter: (to, from, next) => {
            if (store.getters['UserInfo/hasUserSelected']) {
                next({
                    name: "plan" // Default redirect if user is already logged in
                })
            }
            else {
                next()
            }
        }
    },
    {
        path: '/plan',
        name: 'plan',
        component: Plan,
        beforeEnter: requireLogin
    },
    {
        path: '/history',
        name: 'history',
        component: History,
        beforeEnter: requireLogin
    }
];

const router = new VueRouter({
    routes
});

export default router
