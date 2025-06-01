import { createRouter, createWebHistory } from 'vue-router'

const routes = [
  {
    path: '/',
    name: 'Home',
    component: () => import('./views/Article.vue'),
    props: { article_name: 'home.md' },
  },
  {
    path: '/about',
    name: 'About',
    component: () => import('./views/Article.vue'),
    props: { article_name: 'this site.md' },
  },
  {
    path: '/article/:article_name',
    name: 'Article',
    component: () => import('./views/Article.vue'),
    props: (route) => ({ article_name: route.params.article_name }),
  },
  {
    path: '/lua',
    name: 'LuaEditor',
    component: () => import('./views/Lua.vue'),
  },
  {
    path: '/timeline',
    name: 'Timeline',
    component: () => import('./views/Timeline.vue'),
  },
  {
    path: '/:pathMatch(.*)*',
    name: 'ServerHandled',
    beforeEnter: (to) => {
      window.location.href = to.fullPath
      return false
    },
  },
]

export const router = createRouter({
  history: createWebHistory(),
  routes,
})

export default router
