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
  padding: 0.75rem 1rem;
  margin-bottom: 1.5rem;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid #333;
  border-radius: 8px;
}

.bar-left {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.bar-title {
  font-weight: 700;
  color: #f5f5f5;
}

.bar-count {
  color: #888;
  font-size: 0.85rem;
}

.sort-btn {
  padding: 0.4rem 0.8rem;
  border: 1px solid #555;
  border-radius: 6px;
  background: transparent;
  color: #5ce2fa;
  cursor: pointer;
  font-family: inherit;
  font-size: 0.85rem;
  transition: all 0.2s;
}

.sort-btn:hover {
  background: rgba(92, 226, 250, 0.1);
  border-color: #5ce2fa;
}
</style>