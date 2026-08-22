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

  // Placeholders avant le passage markdown pour monter les cellules Lua
  let content = articleContent.value
  let placeholderCount = 0

  content = content.replace(/```lua\s*\n([\s\S]*?)\n\s*```/g, () => {
    const placeholderId = placeholderCount++
    return `<div data-lua-placeholder="${placeholderId}"></div>`
  })

  content = parseObsidianLinks(content)

  return marked.parse(content)
})

function replace_codeblock() {
  nextTick(() => {
    const placeholderDivs = document.querySelectorAll('div[data-lua-placeholder]')

    placeholderDivs.forEach((placeholderDiv, index) => {
      const placeholderId = Number.parseInt(placeholderDiv.getAttribute('data-lua-placeholder'))
      const componentId = `lua-notebook-block-${index}`
      const blockId = placeholderId + 1

      if (placeholderId < codeBlocks.value.length) {
        const codeContent = codeBlocks.value[placeholderId]

        const componentDiv = document.createElement('div')
        componentDiv.id = componentId

        placeholderDiv.parentNode.replaceChild(componentDiv, placeholderDiv)

        const contextForBlock = buildContextForBlock(blockId)

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

    // Split au premier '----' : contexte global | corps notebook
    const parts = data.content.split('----', 2)

    if (parts.length === 2) {
      baseCodeContext.value = parts[0].trim()
      articleContent.value = parts[1].trim()
    } else {
      baseCodeContext.value = ''
      articleContent.value = data.content
    }

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

  for (let i = 0; i < blockId - 1; i++) {
    if (codeBlocks.value[i]) {
      context += '\n' + codeBlocks.value[i]
    }
  }

  return context
}

onMounted(async () => {
  await fetchAndProcessArticle()

  setTimeout(() => {
    replace_codeblock()
  }, 100)

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
  <div class="main-container">
    <div v-if="!articleContent" class="page-status">Loading Lua notebook...</div>
    <div v-else class="markdown-content" v-html="renderedContent"></div>
  </div>
</template>
<style scoped>
.main-container {
  color: var(--color-fg);
  min-height: 100vh;
  padding: var(--space-lg);
  padding-top: calc(var(--space-lg) + var(--space-md));
  font-family: var(--font-body);
}
</style>
