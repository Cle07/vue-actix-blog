<script setup>
import LuaPlayground from '../components/LuaPlayground.vue'
</script>

<template>
  <div class="lua-page">
    <h1>Lua Playground</h1>
    <p>Here you can run Lua code through the Rust backend of the website.</p>
    <LuaPlayground
      :defaultCode="
        String.raw`-- This Lua editor runs on luau for safety.

to_print = {&quot;Hello&quot;, &quot;World&quot;}
print(to_print[1]..&quot;, &quot;..to_print[2]..&quot;!&quot;)`
      "
    />
  </div>
</template>

<style scoped>
.lua-page {
  padding: var(--space-lg);
  font-family: var(--font-body);
}

h1 {
  color: var(--color-fg);
  font-size: 1.8rem;
  margin-bottom: var(--space-md);
  border-bottom: 4px solid var(--color-fg);
  padding-bottom: var(--space-xs);
}

p {
  margin-bottom: var(--space-md);
  line-height: var(--line-height-body);
}
</style>
