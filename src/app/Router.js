import Home from "@/pages/Home";
import {createRouter, createWebHistory} from "vue-router";
import Error from "@/pages/Error";

const routes = [
    {path: '/', name: 'Home', component: Home},
    {path: "/:catchAll(.*)", name: 'Error', component: Error}
]

const router = createRouter({
    history: createWebHistory(),
    routes
});

export default router;
