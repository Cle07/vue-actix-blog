<script setup>
import { computed } from 'vue'
import Article from './Article.vue'
import LuaNotebook from './LuaNotebook.vue'
import Timeline from './Timeline.vue'

const props = defineProps({
  article_name: {
    type: String,
    required: true,
  },
})

const isLuaNotebook = computed(() => {
  return props.article_name.endsWith('.lua.md')
})
const isTimeline = computed(() => {
  return props.article_name.endsWith('.timeline.md')
})
</script>

<template>
  <LuaNotebook v-if="isLuaNotebook" :article_name="article_name" />
  <Timeline v-else-if="isTimeline" :article_name="article_name" />
  <Article v-else :article_name="article_name" />
</template>
