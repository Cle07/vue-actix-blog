import { createRouter, createWebHistory } from 'vue-router'

const routes = [
    {
        path: '/',
        name: 'Home',
        component: () => import('./components/HelloWorld.vue')
    },
    {
        path: '/lua',
        name: 'LuaEditor',
        component: () => import('./views/Lua.vue')
    },
    {
        path: '/about',
        name: 'About',
        component: () => import('./views/About.vue')
    },
    {
        path: '/timeline',
        name: 'Timeline',
        component: () => import('./views/Timeline.vue')
    },
    {
        path: '/:pathMatch(.*)*',
        name: 'ServerHandled',
        beforeEnter: (to) => {
            window.location.href = to.fullPath;
            return false;
        }
    }
]

export const router = createRouter({
    history: createWebHistory(),
    routes
})

export default router