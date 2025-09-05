<template>
  <div v-if="exercise">
    <img src="/schema.svg" />

    <h2>{{ exercise.title }}</h2>
    <p>{{ exercise.description }}</p>

    <textarea v-model="query" rows="6" cols="60" placeholder="Write your SQL here"></textarea>
    <br />
    <button @click="submit">Submit</button>

    <div v-if="result" class="result">
      <p :style="{ color: result.correct ? 'green' : 'red' }">
        {{ result.message }}
      </p>
      <button v-if="result.correct" @click="goNext">
        Next Exercise
      </button>
    </div>
  </div>

  <p v-else>Loading...</p>
</template>

<script setup>
import { ref, onMounted, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'

const route = useRoute()
const router = useRouter()
const exercise = ref(null)
const query = ref('')
const result = ref(null)
const exercises = ref([]) // We'll fetch all exercises to know next

async function loadExercise(id) {
  const res = await fetch(`/api/exercise/${id}`)
  exercise.value = await res.json()
  query.value = ''
  result.value = null
}

onMounted(async () => {
  // Fetch all exercises once
  const exRes = await fetch('/api/exercises')
  exercises.value = (await exRes.json()).sort((a, b) => a.id - b.id)

  // Load current exercise
  await loadExercise(route.params.id)
})

// Watch for route changes (when navigating to next exercise)
watch(() => route.params.id, async (newId) => {
  await loadExercise(newId)
})

async function submit() {
  result.value = null
  const res = await fetch('/api/submit', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      id: parseInt(route.params.id),
      query: query.value,
    }),
  })
  result.value = await res.json()
}

function goNext() {
  const currentIndex = exercises.value.findIndex(e => e.id == exercise.value.id)
  const nextExercise = exercises.value[currentIndex + 1]

  if (nextExercise) {
    router.push(`/exercise/${nextExercise.id}`)
  } else {
    router.push('/') // back to home if last exercise
  }
}
</script>

<style>
textarea {
  width: 100%;
  margin: 1rem 0;
  padding: 0.5rem;
  font-family: monospace;
}

button {
  padding: 0.5rem 1rem;
  background: #007acc;
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
}

button:hover {
  background: #005fa3;
}

.result {
  margin-top: 1rem;
  font-weight: bold;
}
</style>

<style>
textarea {
  width: 100%;
  margin: 1rem 0;
  padding: 0.5rem;
  font-family: monospace;
}

button {
  padding: 0.5rem 1rem;
  background: #007acc;
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
}

button:hover {
  background: #005fa3;
}

.result {
  margin-top: 1rem;
  font-weight: bold;
}

img {
  border-radius: 15px;
  display: block;
  max-width: 100%;
  margin: 1rem auto;
  shape-rendering: crispEdges;
  overflow: visible;
}
</style>
