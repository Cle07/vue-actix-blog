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

  // FIRST process Obsidian-specific syntax on the raw markdown
  const obsidianProcessed = parseObsidianLinks(article.value.content)

  // THEN convert markdown to HTML
  return marked.parse(obsidianProcessed)
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

      <div v-if="article.backlinks && article.backlinks.length > 0" class="backlinks-section">
        <h2>Backlinks:</h2>
        <ul class="backlinks">
          <li v-for="backlink in article.backlinks" :key="backlink">
            <a href="#" :data-link="backlink" class="obsidian-link custom-link">{{ backlink }}</a>
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>

<style scoped>
.article {
  padding: 1.5rem;
  font-family: 'JetBrains Mono', monospace;
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
  display: flex;
  flex-direction: column;
  margin-bottom: 2rem;
  line-height: 1.6;
  gap: 0px;
}

.markdown-content :deep(a) {
  color: #5ce2fa;
}

.markdown-content :deep(.obsidian-image) {
  max-width: 100%;
  height: auto;
  margin: 1rem 0;
  border-radius: 4px;
  align-self: center;
}

.markdown-content :deep(.obsidian-link) {
  color: #6dd4a3;
  background-color: rgba(109, 212, 163, 0.3);
  text-decoration: none;
  cursor: pointer;
  border: none;
  padding: 0.5px 2px;
  border-radius: 4px;
  font: inherit;
}

.markdown-content :deep(.obsidian-link:hover) {
  color: #8de4b8;
  background-color: rgba(141, 228, 184, 0.4);
}

.markdown-content :deep(.footnotes) {
  margin-top: 0px;
  padding-top: 1rem;
  border-top: 1px solid #333;
  font-size: 0.9rem;
  word-wrap: break-word;
  overflow-wrap: break-word;
}

.markdown-content :deep(iframe) {
  margin: 1rem 0px;
  align-self: center;
  border: 2px solid #333;
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

.backlinks-section {
  margin-top: 1rem;
  padding-top: 1rem;
  border-top: 2px solid #333;
}

ul.backlinks {
  margin-left: 1.5rem;
  margin-bottom: 1rem;
  border-left: 4px solid #666;
  padding-left: 1rem;
}

ul.backlinks li {
  margin-bottom: 0.5rem;
  list-style-type: square;
}

ul.backlinks a {
  color: #6dd4a3;
  text-decoration: none;
}

ul.backlinks a:hover {
  text-decoration: underline;
}
</style>
