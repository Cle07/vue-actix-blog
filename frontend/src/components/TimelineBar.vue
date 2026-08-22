<script setup>
const props = defineProps({
  entryCount: {
    type: Number,
    default: 0,
  },
  sortOrder: {
    type: String,
    default: 'desc',
  },
})

const emit = defineEmits(['sort-change'])

function toggleSort() {
  const newOrder = props.sortOrder === 'desc' ? 'asc' : 'desc'
  emit('sort-change', newOrder)
}
</script>

<template>
  <div class="timeline-bar">
    <div class="bar-left">
      <span class="bar-icon"></span>
      <span class="bar-title"></span>
      <span class="bar-count">{{ entryCount }} entrée(s)</span>
    </div>
    <div class="bar-right">
      <button @click="toggleSort" class="sort-btn">
        {{ sortOrder === 'desc' ? '↓ Plus récent' : '↑ Plus ancien' }}
      </button>
    </div>
  </div>
</template>

<style scoped>
.timeline-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--space-sm) var(--space-md);
  margin-bottom: var(--space-lg);
  background: var(--color-bg-soft);
  border: var(--border-width-thin) solid var(--color-border-muted);
  border-radius: var(--radius-lg);
}

.bar-left {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
}

.bar-title {
  font-weight: 700;
  color: var(--color-fg);
}

.bar-count {
  color: var(--color-fg-muted);
  font-size: var(--font-size-sm);
}

.sort-btn {
  padding: 0.4rem 0.8rem;
  border: var(--border-width-thin) solid var(--color-border-subtle);
  border-radius: var(--radius-md);
  background: transparent;
  color: var(--color-accent);
  cursor: pointer;
  font-family: inherit;
  font-size: var(--font-size-sm);
  transition: all 0.2s;
}

.sort-btn:hover {
  background: var(--color-accent-soft);
  border-color: var(--color-accent);
}
</style>
