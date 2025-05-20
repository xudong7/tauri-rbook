import { createRouter, createWebHistory } from 'vue-router'
import type { RouteLocationNormalized } from 'vue-router'
import MenuView from '../views/MenuView/MenuView.vue'
import ReaderView from '../views/ReaderView/ReaderView.vue'
import SearchView from '../views/SearchView/SearchView.vue'
import SettingsView from '../views/SettingsView/SettingsView.vue'

const routes = [
  {
    path: '/',
    name: 'Menu',
    component: MenuView
  },
  {
    path: '/reader',
    name: 'Reader',
    component: ReaderView,
    props: (route: RouteLocationNormalized) => ({ initialFilePath: route.query.filePath as string })
  },
  {
    path: '/search',
    name: 'Search',
    component: SearchView
  },
  {
    path: '/settings',
    name: 'Settings',
    component: SettingsView
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router
