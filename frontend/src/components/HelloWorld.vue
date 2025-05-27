<script setup>
import { ref, onMounted } from 'vue'

defineProps({
  msg: {
    type: String,
    required: true,
  },
})

const apiMessage = ref('Loading...')

onMounted(async () => {
  try {
    const response = await fetch('/api/actix')
    const data = await response.json()
    apiMessage.value = data.message
  } catch (error) {
    apiMessage.value = 'Error fetching API data'
    console.error('API Error:', error)
  }
})
</script>

<template>
  <div class="greetings">
    <h1 class="green">{{ msg }}</h1>
    <h3>
      You've successfully created a project with
      <a href="https://vite.dev/" target="_blank" rel="noopener">Vite</a> +
      <a href="https://vuejs.org/" target="_blank" rel="noopener">Vue 3</a> +
      <span class="api-message">{{ apiMessage }}</span>.
    </h3>
  </div>
</template>

<style scoped>
h1 {
  font-weight: 500;
  font-size: 2.6rem;
  position: relative;
  top: -10px;
}

h3 {
  font-size: 1.2rem;
}

.greetings h1,
.greetings h3 {
  text-align: center;
}

.api-message {
  font-weight: bold;
  color: #42b883;
}

@media (min-width: 1024px) {
  .greetings h1,
  .greetings h3 {
    text-align: left;
  }
}
</style>
