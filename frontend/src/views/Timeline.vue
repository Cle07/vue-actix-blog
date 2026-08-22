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

// Format timeline : "# date\ncontenu\n# date\ncontenu..."
const timelineEntries = computed(() => {
  if (!article.value?.content) return []

  const raw = article.value.content
  const entries = []
  let currentDate = null
  let currentContent = ''

  for (const line of raw.split('\n')) {
    if (line.startsWith('# ')) {
      if (currentDate !== null) {
        entries.push({
          date: currentDate,
          content: currentContent.trim(),
        })
      }

      currentDate = line.slice(2).trim()
      currentContent = ''
    } else if (currentDate !== null) {
      currentContent += `${line}\n`
    }
  }

  if (currentDate !== null) {
    entries.push({
      date: currentDate,
      content: currentContent.trim(),
    })
  }

  const parseDate = (dateString) => {
    const [day, month, year] = dateString.split('/').map(Number)

    if (!day || !month || !year) {
      console.warn('Date timeline invalide :', dateString)
      return new Date(0)
    }

    return new Date(year, month - 1, day)
  }

  return [...entries].sort((a, b) => {
    const difference = parseDate(a.date) - parseDate(b.date)

    return sortOrder.value === 'desc' ? -difference : difference
  })
})

function parseEntryContent(content) {
  const obsidianProcessed = parseObsidianLinks(content)
  return marked.parse(obsidianProcessed)
}

function handleSortChange(newOrder) {
  sortOrder.value = newOrder
}

onMounted(async () => {
  try {
    const response = await fetch(`/api/article/${encodeURIComponent(props.article_name)}`)
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
  <div class="page-article">
    <div v-if="loading" class="page-status">Loading timeline...</div>
    <div v-else-if="error" class="page-status page-status--error">{{ error }}</div>

    <div v-else-if="article" class="content">
      <TimelineBar
        :entry-count="timelineEntries.length"
        :sort-order="sortOrder"
        @sort-change="handleSortChange"
      />

      <div class="timeline-entries">
        <div v-for="(entry, index) in timelineEntries" :key="index" class="timeline-entry">
          <div class="entry-date">{{ entry.date }}</div>
          <div class="entry-content markdown-content" v-html="parseEntryContent(entry.content)"></div>
        </div>
      </div>

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

<style scoped>
.timeline-entries {
  display: flex;
  flex-direction: column;
  gap: 0;
}

.timeline-entry {
  padding: 1.25rem 0 1.25rem var(--space-lg);
  border-left: 3px solid var(--color-border-muted);
  position: relative;
  transition: border-color 0.2s;
}

.timeline-entry:hover {
  border-left-color: var(--color-accent);
}

.timeline-entry::before {
  content: '';
  position: absolute;
  left: -7px;
  top: 1.5rem;
  width: 11px;
  height: 11px;
  border-radius: 50%;
  background: var(--color-border-muted);
  border: 2px solid var(--color-bg);
  transition: background 0.2s;
}

.timeline-entry:hover::before {
  background: var(--color-accent);
}

.entry-date {
  font-weight: 700;
  color: var(--color-accent);
  font-size: 1.1rem;
  margin-bottom: var(--space-sm);
}

.entry-content {
  line-height: var(--line-height-body);
}
</style>
