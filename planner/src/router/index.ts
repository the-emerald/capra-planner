import Vue from 'vue'
import VueRouter, { RouteConfig } from 'vue-router'
import Plan from '../views/Plan.vue'
import Test from "../views/Test.vue"

Vue.use(VueRouter);

const routes: Array<RouteConfig> = [
    {
        path: '/',
        redirect: 'plan'
    },
    {
        path: '/plan',
        name: 'plan',
        component: Plan
    },
    {
        path: '/test',
        name: 'test',
        component: Test
    },
];

const router = new VueRouter({
    routes
});

export default router
