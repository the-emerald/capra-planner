import Vue from 'vue'
import VueRouter, { RouteConfig } from 'vue-router'
import Plan from '../views/Plan.vue'
import Login from '../views/Login.vue'
import store from "@/store";


Vue.use(VueRouter);

const routes: Array<RouteConfig> = [
    {
        path: '/',
        redirect: 'login'
    },
    {
        path: '/login',
        name: 'login',
        component: Login
    },
    {
        path: '/plan',
        name: 'plan',
        component: Plan
    },
];

const router = new VueRouter({
    routes
});

router.beforeEach((to, from, next) => {
    if (to.name != 'login' && !store.getters.hasUserSelected) {
        next({
            name: "login"
        })
    }
    else {
        next()
    }
});

export default router
