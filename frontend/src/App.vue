<script setup>
import NavBar from './components/NavBar.vue'
</script>

<template>
  <main>
    <NavBar />
    <div class="content">
      <router-view :key="$route.fullPath" />
    </div>
  </main>
</template>

<style>
@font-face {
  font-family: 'Departure Mono';
  src:
    url('@/assets/fonts/DepartureMono-Regular.woff2') format('woff2'),
    url('@/assets/fonts/DepartureMono-Regular.woff') format('woff');
  font-weight: 400;
  font-style: normal;
}

html,
body {
  background-color: #111314;
  color: #f5f5f5;
  margin: 0;
  padding: 0;
  min-height: 100vh;
}

main {
  display: flex;
  flex-direction: column;
  font-family: 'Departure Mono', monospace;
  flex: 1 1 auto;
  width: 100%;
  max-width: 900px;
  margin: 0 auto;
  padding: 0;
  border: 3px solid #f5f5f5;
  padding-top: 6em;
}

.content {
  padding: 3px;
}
</style>
