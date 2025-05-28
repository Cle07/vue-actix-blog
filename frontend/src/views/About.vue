<script setup>
// About page component
</script>

<template>
  <div class="about">
    <h1>About This Project</h1>
    <p>This is a Vue.js application with Actix Web backend.</p>
    <p>The application uses:</p>
    <ul>
      <li>A Rust Actix Web server</li>
      <li>Vue.js frontend components</li>
      <li>A Lua code editor with syntax highlighting</li>
      <li>mlua crate for Lua Compilation</li>
      <li>Vue Router for client-side navigation</li>
    </ul>
  </div>

</template>

<style scoped>
.about {
  padding: 1.5rem;
  font-family: 'Departure Mono', monospace;
}

h1 {
  color: #f5f5f5;
  font-size: 1.8rem;
  margin-bottom: 1rem;
  border-bottom: 4px solid #f5f5f5;
  padding-bottom: 0.5rem;
}

p {
  margin-bottom: 1rem;
  line-height: 1.6;
}

ul {
  margin-left: 1.5rem;
  margin-bottom: 1rem;
  border-left: 4px solid #333;
  padding-left: 1rem;
}

li {
  margin-bottom: 0.5rem;
  list-style-type: square;
}
</style>