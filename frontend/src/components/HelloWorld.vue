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
      Welcome to my project built with
      <a href="https://vite.dev/" target="_blank" rel="noopener">Vite</a> +
      <a href="https://vuejs.org/" target="_blank" rel="noopener">Vue 3</a> +
      <a href="https://actix.rs" class="api-message">{{ apiMessage }}</a
      >. <br /><br />
      Use the navigation bar to explore.
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

.api-message {
  font-weight: bold;
  color: #42b883;
}
</style>
