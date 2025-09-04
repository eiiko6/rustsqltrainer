<script setup>
import { ref, onMounted } from 'vue'

const exercises = ref([])

onMounted(async () => {
  const res = await fetch('/api/exercises')
  const data = await res.json()
  exercises.value = data.sort((a, b) => a.id - b.id)  // Sort by id
})
</script>

<template>
  <div>
    <h2>Exercises</h2>
    <ul class="exercise-list">
      <li v-for="ex in exercises" :key="ex.id">
        <router-link :to="`/exercise/${ex.id}`">{{ ex.title }}</router-link>
      </li>
    </ul>
  </div>
</template>

<style>
.exercise-list {
  padding-left: 1rem;
  list-style-position: inside;
  text-align: left;
}

.exercise-list li {
  margin-bottom: 0.5rem;
  text-align: left;
}

.exercise-list a {
  text-decoration: none;
  color: #007acc;
}

.exercise-list a.router-link-exact-active {
  font-weight: bold;
}
</style>
