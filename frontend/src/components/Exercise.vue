<script setup>
import { ref, onMounted } from 'vue'
import { useRoute } from 'vue-router'

const route = useRoute()
const exercise = ref(null)
const query = ref('')
const result = ref(null)

onMounted(async () => {
  const res = await fetch(`/api/exercise/${route.params.id}`)
  exercise.value = await res.json()
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
</script>

<template>
  <div v-if="exercise">
    <h2>{{ exercise.title }}</h2>
    <p>{{ exercise.description }}</p>

    <textarea v-model="query" rows="6" cols="60" placeholder="Write your SQL here"></textarea>
    <br />
    <button @click="submit">Submit</button>

    <div v-if="result" class="result">
      <p :style="{ color: result.correct ? 'green' : 'red' }">
        {{ result.message }}
      </p>
    </div>
  </div>

  <p v-else>Loading...</p>
</template>

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
