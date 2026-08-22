<script setup>
import NavBar from './components/NavBar.vue'
</script>

<template>
  <main class="app-shell">
    <NavBar />
    <div class="content">
      <router-view :key="$route.fullPath" />
    </div>
  </main>
</template>
