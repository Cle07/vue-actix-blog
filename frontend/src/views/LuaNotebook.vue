<script setup>
import { ref, onMounted, nextTick, computed } from 'vue'
import LuaCodeBlock from '../components/LuaCodeBlock.vue'
import { createApp } from 'vue'
import { marked } from 'marked'
import router from '@/router'
import { parseObsidianLinks } from '../main'

const props = defineProps({
  article_name: {
    type: String,
    required: true,
  },
})

const baseCodeContext = ref('')
const articleContent = ref('')
const codeBlocks = ref([])

const renderedContent = computed(() => {
  if (!articleContent.value) return ''

  // Replace lua code blocks with placeholders before markdown processing
  let content = articleContent.value
  let placeholderCount = 0

  content = content.replace(/```lua\s*\n([\s\S]*?)\n\s*```/g, (match, code) => {
    const placeholderId = placeholderCount++
    return `<div data-lua-placeholder="${placeholderId}"></div>`
  })

  // Process Obsidian links
  content = parseObsidianLinks(content)

  return marked.parse(content)
})

function replace_codeblock() {
  nextTick(() => {
    // Find all placeholder divs
    const placeholderDivs = document.querySelectorAll('div[data-lua-placeholder]')

    placeholderDivs.forEach((placeholderDiv, index) => {
      const placeholderId = Number.parseInt(placeholderDiv.getAttribute('data-lua-placeholder'))
      const componentId = `lua-notebook-block-${index}`
      const blockId = placeholderId + 1 // 1-indexed based on placeholder ID

      // Use the pre-extracted code block
      if (placeholderId < codeBlocks.value.length) {
        const codeContent = codeBlocks.value[placeholderId]

        // Create a new div to mount the LuaCodeBlock component
        const componentDiv = document.createElement('div')
        componentDiv.id = componentId

        // Replace the placeholder div
        placeholderDiv.parentNode.replaceChild(componentDiv, placeholderDiv)

        // Build context for this block
        const contextForBlock = buildContextForBlock(blockId)

        // Create and mount the Vue component
        const app = createApp(LuaCodeBlock, {
          defaultCode: codeContent.trim(),
          context: contextForBlock,
          id: blockId,
        })
        app.mount(`#${componentId}`)
      }
    })
  })
}

async function fetchAndProcessArticle() {
  try {
    const response = await fetch(`/api/article/${props.article_name}`)
    if (!response.ok) {
      throw new Error(`Article not found: ${response.status}`)
    }
    const data = await response.json()

    // Split at the first '----'
    const parts = data.content.split('----', 2)

    if (parts.length === 2) {
      baseCodeContext.value = parts[0].trim()
      articleContent.value = parts[1].trim()
    } else {
      baseCodeContext.value = ''
      articleContent.value = data.content
    }

    // Extract lua code blocks for context building
    extractCodeBlocks()
  } catch (error) {
    console.error('LuaNotebook: Error fetching article:', error)
    articleContent.value = `<p>Error loading article: ${error.message}</p>`
  }
}

function extractCodeBlocks() {
  const luaCodeRegex = /```lua\s*\n([\s\S]*?)\n\s*```/g
  const blocks = []
  let match

  while ((match = luaCodeRegex.exec(articleContent.value)) !== null) {
    const codeBlock = match[1].trim()
    blocks.push(codeBlock)
  }

  codeBlocks.value = blocks
}

function buildContextForBlock(blockId) {
  let context = baseCodeContext.value

  // Add all previous blocks (1-indexed, so blockId-1 gives us 0 to blockId-1)
  for (let i = 0; i < blockId - 1; i++) {
    if (codeBlocks.value[i]) {
      context += '\n' + codeBlocks.value[i]
    }
  }

  return context
}

onMounted(async () => {
  // First fetch and process the article
  await fetchAndProcessArticle()

  // Replace code blocks after the Article component has rendered
  setTimeout(() => {
    replace_codeblock()
  }, 100)

  // Add click handler for internal links
  document.addEventListener('click', (e) => {
    if (e.target.classList.contains('custom-link')) {
      e.preventDefault()
      const route = e.target.getAttribute('data-route')
      const link = e.target.getAttribute('data-link')
      if (route) {
        router.push(route)
      } else if (link) {
        // Fallback for backward compatibility
        router.push(`/article/${link}`)
      }
    }
  })
})
</script>
<template>
  <div class="main-container">
    <div v-if="!articleContent" class="loading">Loading Lua notebook...</div>
    <div v-else class="markdown-content" v-html="renderedContent"></div>
  </div>
</template>
<style scoped>
.main-container {
  color: #f5f5f5;
  min-height: 100vh;
  padding: 1.5rem;
  font-family: 'JetBrains Mono', monospace;
}

.loading {
  text-align: center;
  padding: 2rem;
  color: #f5f5f5;
}

.markdown-content {
  display: flex;
  flex-direction: column;
  margin-bottom: 2rem;
  line-height: 1.6;
  gap: 0px;
}

.markdown-content :deep(h1) {
  color: #f5f5f5;
  font-size: 1.8rem;
  margin-bottom: 1rem;
  border-bottom: 4px solid #f5f5f5;
  padding-bottom: 0.5rem;
}

.markdown-content :deep(h2) {
  color: #f5f5f5;
  font-size: 1.4rem;
  margin-top: 2rem;
  margin-bottom: 1rem;
}

.markdown-content :deep(h3) {
  color: #f5f5f5;
  font-size: 1.2rem;
  margin-top: 1.5rem;
  margin-bottom: 0.8rem;
}

.markdown-content :deep(p) {
  margin-bottom: 1rem;
  color: #f5f5f5;
}

.markdown-content :deep(a) {
  color: #5ce2fa;
  text-decoration: none;
}

.markdown-content :deep(a:hover) {
  text-decoration: underline;
}

.markdown-content :deep(pre) {
  background-color: #1a1a1a;
  padding: 1rem;
  border: 1px solid #333;
  border-radius: 4px;
  overflow-x: auto;
  margin: 1rem 0;
}

.markdown-content :deep(code) {
  background-color: #333;
  color: #f5f5f5;
  padding: 0.2rem 0.4rem;
  border-radius: 3px;
  font-family: 'Departure Mono', monospace;
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
</style>
