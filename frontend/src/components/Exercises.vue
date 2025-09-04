<script setup>
import { ref, onMounted } from 'vue'

const exercises = ref([])

onMounted(async () => {
  const res = await fetch('/api/exercises')
  exercises.value = await res.json()
})
</script>

<template>
  <div>
    <h2>Exercises</h2>
    <ul>
      <li v-for="ex in exercises" :key="ex.id">
        <router-link :to="`/exercise/${ex.id}`">{{ ex.title }}</router-link>
      </li>
    </ul>
  </div>
</template>
