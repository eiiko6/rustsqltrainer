import { createApp } from 'vue'
import { createRouter, createWebHistory } from 'vue-router'
import './style.css'

import App from './App.vue'

import Exercises from './components/Exercises.vue'
import Exercise from './components/Exercise.vue'

const routes = [
  { path: '/', component: Exercises },
  { path: '/exercise/:id', component: Exercise, props: true }
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

createApp(App).use(router).mount('#app')

