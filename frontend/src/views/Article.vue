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

  // D'abord markdown → HTML, puis syntaxe Obsidian / LaTeX
  const rawHtml = marked.parse(article.value.content)
  return parseObsidianLinks(rawHtml)
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
      const route = e.target.getAttribute('data-route')
      const link = e.target.getAttribute('data-link')
      if (route) {
        router.push(route)
      } else if (link) {
        router.push(`/article/${link}`)
      }
    }
  })
})
</script>

<template>
  <div class="page-article">
    <div v-if="loading" class="page-status">Loading article...</div>
    <div v-else-if="error" class="page-status page-status--error">
      {{ error }}
    </div>
    <div v-else-if="article" class="content">
      <div class="markdown-content" v-html="parsedContent"></div>

      <div v-if="article.backlinks && article.backlinks.length > 0" class="backlinks-section">
        <h2>Backlinks:</h2>
        <ul class="backlinks">
          <li v-for="backlink in article.backlinks" :key="backlink">
            <a
              href="#"
              :data-link="backlink"
              :data-route="`/article/${backlink}`"
              class="obsidian-link custom-link"
              >{{ backlink }}</a
            >
          </li>
        </ul>
      </div>
    </div>
  </div>
</template>
