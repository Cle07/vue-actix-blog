<script setup>
import router from '@/router'
import { ref, onMounted, computed } from 'vue'
import { marked } from 'marked'
import { parseObsidianLinks } from '../main'
import TimelineBar from '../components/TimelineBar.vue'

const props = defineProps({
  article_name: {
    type: String,
    required: true,
  },
})

const article = ref(null)
const loading = ref(true)
const error = ref(null)
const sortOrder = ref('desc')

// 🎯 Parsing du format timeline : "# date\ncontenu\n# date\ncontenu..."
const timelineEntries = computed(() => {
  if (!article.value?.content) return []

  const raw = article.value.content
  const entries = []
  let currentDate = null
  let currentContent = ''

  for (const line of raw.split('\n')) {
    if (line.startsWith('# ')) {
      // Sauvegarder l'entrée précédente
      if (currentDate !== null) {
        entries.push({
          date: currentDate,
          content: currentContent.trim(),
        })
      }
      currentDate = line.slice(2).trim()
      currentContent = ''
    } else if (currentDate !== null) {
      currentContent += line + '\n'
    }
  }

  // Dernière entrée
  if (currentDate !== null) {
    entries.push({
      date: currentDate,
      content: currentContent.trim(),
    })
  }

  // Tri par date
  return [...entries].sort((a, b) => {
    return sortOrder.value === 'desc'
      ? b.date.localeCompare(a.date)
      : a.date.localeCompare(b.date)
  })
})

// Parser le markdown de chaque entrée individuellement
function parseEntryContent(content) {
  const obsidianProcessed = parseObsidianLinks(content)
  return marked.parse(obsidianProcessed)
}

function handleSortChange(newOrder) {
  sortOrder.value = newOrder
}

onMounted(async () => {
  console.log('🔎 article_name reçu :', props.article_name)
  console.log('🔎 URL appelée :', `/api/article/${props.article_name}`)
  
  try {
    const response = await fetch(`/api/article/${encodeURIComponent(props.article_name)}`)
    // ...
    if (!response.ok) {
      throw new Error(`Article not found: ${props.article_name}`)
    }
    article.value = await response.json()
    loading.value = false
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
  <div class="article">
    <div v-if="loading" class="loading">Loading timeline...</div>
    <div v-else-if="error" class="error">{{ error }}</div>

    <div v-else-if="article" class="content">
      <!-- 🎯 Widget TimelineBar -->
      <TimelineBar
        :entry-count="timelineEntries.length"
        :sort-order="sortOrder"
        @sort-change="handleSortChange"
      />

      <!-- Entrées de la timeline -->
      <div class="timeline-entries">
        <div
          v-for="(entry, index) in timelineEntries"
          :key="index"
          class="timeline-entry"
        >
          <div class="entry-date">{{ entry.date }}</div>
          <div
            class="entry-content markdown-content"
            v-html="parseEntryContent(entry.content)"
          ></div>
        </div>
      </div>

      <!-- Backlinks (conservé de Article.vue) -->
      <div
        v-if="article.backlinks && article.backlinks.length > 0"
        class="backlinks-section"
      >
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
  border: 2px solid #ff6b6b;
  border-radius: 4px;
  padding: 1rem;
}

/* 🎯 Styles Timeline */
.timeline-entries {
  display: flex;
  flex-direction: column;
  gap: 0;
}

.timeline-entry {
  padding: 1.25rem 0 1.25rem 1.5rem;
  border-left: 3px solid #333;
  position: relative;
  transition: border-color 0.2s;
}

.timeline-entry:hover {
  border-left-color: #5ce2fa;
}

.timeline-entry::before {
  content: '';
  position: absolute;
  left: -7px;
  top: 1.5rem;
  width: 11px;
  height: 11px;
  border-radius: 50%;
  background: #333;
  border: 2px solid #1a1a1a;
  transition: background 0.2s;
}

.timeline-entry:hover::before {
  background: #5ce2fa;
}

.entry-date {
  font-weight: 700;
  color: #5ce2fa;
  font-size: 1.1rem;
  margin-bottom: 0.75rem;
}

.entry-content {
  line-height: 1.6;
}

/* Styles markdown conservés */
.markdown-content :deep(a) {
  color: #5ce2fa;
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

.markdown-content :deep(.obsidian-image) {
  max-width: 100%;
  height: auto;
  margin: 1rem 0;
  border-radius: 4px;
}

/* Backlinks */
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