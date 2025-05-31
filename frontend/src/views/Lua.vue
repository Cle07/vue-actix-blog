<script setup>
// About page component
import LuaComputer from '../components/LuaComputer.vue'
</script>

<template>
  <div class="about">
    <h1>Lua Code Compiler</h1>
    <p>Here you can run Lua code through the Rust backend of the website.</p>
    <LuaComputer
      :defaultCode="
        String.raw`-- This Lua editor runs on luau for safety.

to_print = {&quot;Hello&quot;, &quot;World&quot;}
print(to_print[1]..&quot;, &quot;..to_print[2]..&quot;!&quot;)`
      "
    />
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
