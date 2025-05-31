<script setup>
import router from '@/router'
import { ref, onMounted, computed } from 'vue'
import { marked } from 'marked'
import { parseObsidianLinks } from '../main'

const props = defineProps({
  article_name: {
    type: String,
    required: true,
  },
})

const article = ref(null)
const loading = ref(true)
const error = ref(null)

const parsedContent = computed(() => {
  if (!article.value?.content) return ''

  // First convert markdown to HTML
  const htmlContent = marked.parse(article.value.content)

  // Then process Obsidian-specific syntax
  return parseObsidianLinks(htmlContent)
})

onMounted(async () => {
  try {
    const response = await fetch(`/api/article/${props.article_name}`)
    if (!response.ok) {
      throw new Error(`Article not found: ${props.article_name}`)
    }
    article.value = await response.json()
    loading.value = false
    console.log(article.value)
  } catch (err) {
    error.value = err.message
    loading.value = false
  }
  document.addEventListener('click', (e) => {
    if (e.target.classList.contains('custom-link')) {
      e.preventDefault()
      const link = e.target.getAttribute('data-link')
      if (link) {
        router.push(`/article/${link}`)
      }
    }
  })
})
</script>

<template>
  <div class="article">
    <div v-if="loading" class="loading">Loading article...</div>
    <div v-else-if="error" class="error">
      {{ error }}
    </div>
    <div v-else-if="article" class="content">
      <div class="markdown-content" v-html="parsedContent"></div>
    </div>
  </div>
</template>

<style scoped>
.article {
  padding: 1.5rem;
  font-family: 'Departure Mono', monospace;
}

.loading,
.error {
  text-align: center;
  padding: 2rem;
  color: #f5f5f5;
}

.error {
  color: #ff6b6b;
  border: 1px solid #ff6b6b;
  border-radius: 4px;
  padding: 1rem;
}

h1 {
  color: #f5f5f5;
  font-size: 1.8rem;
  margin-bottom: 1rem;
  border-bottom: 4px solid #f5f5f5;
  padding-bottom: 0.5rem;
}

h2 {
  color: #f5f5f5;
  font-size: 1.4rem;
  margin-top: 2rem;
  margin-bottom: 1rem;
}

.markdown-content {
  margin-bottom: 2rem;
  line-height: 1.6;
}

.markdown-content :deep(.obsidian-image) {
  max-width: 100%;
  height: auto;
  margin: 1rem 0;
  border-radius: 4px;
}

.markdown-content :deep(.obsidian-link) {
  color: #42b883;
  text-decoration: underline;
  cursor: pointer;
  background: none;
  border: none;
  padding: 0;
  font: inherit;
}

.markdown-content :deep(.footnotes) {
  margin-top: 2rem;
  padding-top: 1rem;
  border-top: 1px solid #333;
  font-size: 0.9rem;
  word-wrap: break-word;
  overflow-wrap: break-word;
}

.links-section {
  margin-top: 2rem;
  padding-top: 1rem;
  border-top: 2px solid #333;
}

ul.links {
  margin-left: 1.5rem;
  margin-bottom: 1rem;
  border-left: 4px solid #333;
  padding-left: 1rem;
}

ul.links li {
  margin-bottom: 0.5rem;
  list-style-type: square;
}

ul.links a {
  color: #42b883;
  text-decoration: none;
}

ul.links a:hover {
  text-decoration: underline;
}
</style>
