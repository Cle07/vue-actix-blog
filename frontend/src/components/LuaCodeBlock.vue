<script setup>
import { ref, onMounted } from 'vue'
import Prism from 'prismjs'
import '@/assets/prism.css'
import 'prismjs/components/prism-lua'

const props = defineProps({
  defaultCode: {
    type: String,
    default: 'print("Hello, World!")',
  },
  context: {
    type: String,
    default: '',
  },
  id: {
    type: Number,
    default: 1,
  },
})

const output = ref('Ready to run')
const hasRun = ref(false)

const resetCode = () => {
  output.value = 'Code has been reset.'
  hasRun.value = false
}

const runCode = async () => {
  hasRun.value = true
  output.value = 'Running Lua code...'
  try {
    const fullCode = `${props.context}\n${props.defaultCode}`
    console.log('Sending code with context:', fullCode)
    const response = await fetch('/api/lua/run', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ code: fullCode }),
    })

    if (!response.ok) {
      throw new Error(`HTTP error! Status: ${response.status}`)
    }

    const data = await response.json()
    console.log('Response data:', data)

    // Legacy : extrait le contenu entre la 1re paire de parenthèses si présent
    const outputText = data.output || 'No output received'
    const match = outputText.match(/\(([^)]*)\)/)
    output.value = match ? match[1] : outputText
  } catch (error) {
    console.error('Error running Lua code:', error)
    output.value = `Error: ${error.message}`
  }
}

onMounted(() => {
  const codeElement = document.querySelector(`#code-block-${props.id}`)
  if (codeElement) {
    const highlighted = Prism.highlight(props.defaultCode, Prism.languages.lua, 'lua')
    codeElement.innerHTML = highlighted
  }
})
</script>
<template>
  <div id="code-container">
    <div id="bar">
      <b id="cell-id"># {{ props.id }}</b>
      <div id="button-group">
        <button @click="runCode" aria-label="Run Lua code">
          <svg fill="none" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
            <path d="M10 20H8V4h2v2h2v3h2v2h2v2h-2v2h-2v3h-2v2z" fill="currentColor" />
          </svg>
          Run
        </button>
        <button @click="resetCode" aria-label="Reset code">
          <svg
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            width="16"
            height="16"
          >
            <path
              d="M8 4h2v2H8V4zm10 6V8H8V6H6v2H4v2h2v2h2v2h2v-2H8v-2h10zm0 8v-8h2v8h-2zm0 0v2h-6v-2h6z"
              fill="currentColor"
            />
          </svg>
          Reset
        </button>
      </div>
    </div>
    <div id="code-editor">
      <pre
        id="code-display"
      ><code :id="`code-block-${props.id}`" class="language-lua">{{ props.defaultCode }}</code></pre>
      <pre id="code-output" v-show="hasRun">{{ output }}</pre>
    </div>
  </div>
</template>
<style scoped>
#bar {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  align-items: center;
  gap: 10px;
  flex: 0 0 auto;
  padding: 10px 16px;
  background-color: var(--color-lua-soft);
  color: var(--color-fg);
  border: var(--border-width-thin) solid var(--color-lua);
  border-bottom: none;
  font-size: 14px;
  font-weight: bold;
  font-family: var(--font-code);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

#cell-id {
  color: var(--color-fg-subtle);
}

#button-group {
  display: flex;
  gap: 18px;
}

button {
  padding: 6px 6px;
  background-color: var(--color-lua);
  color: var(--color-bg-elevated);
  border-radius: 1px;
  border: none;
  font-family: var(--font-code);
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  gap: 4px;
}

button svg {
  width: 24px;
  height: 24px;
}

button:hover {
  background-color: var(--color-accent);
  color: var(--color-bg-elevated);
  transform: translateY(-1px);
}

#code-container {
  display: flex;
  flex-direction: column;
  width: 100%;
  max-width: 85%;
  margin: 1em auto;
  border: var(--border-width-md) solid var(--color-lua);
  border-radius: 1px;
  overflow: hidden;
}

#code-editor {
  display: flex;
  flex-direction: column;
  width: 100%;
}

#code-display {
  margin: 0;
  padding: 16px;
  width: 100%;
  font-family: var(--font-code);
  font-size: 14px;
  line-height: 1.6;
  background-color: var(--color-bg-elevated);
  border: none;
  box-sizing: border-box;
  overflow-x: auto;
  min-height: 60px;
}

#code-display::-webkit-scrollbar {
  height: 8px;
}

#code-display::-webkit-scrollbar-track {
  background: var(--color-bg-muted);
  border-radius: var(--radius-sm);
}

#code-display::-webkit-scrollbar-thumb {
  background: var(--color-lua);
  border-radius: var(--radius-sm);
}

#code-display::-webkit-scrollbar-thumb:hover {
  background: var(--color-accent);
}

#code-display :deep(*) {
  background: transparent !important;
}

#code-display code {
  font-family: inherit;
  color: var(--color-fg);
  background: transparent !important;
}

#code-output {
  margin: 0;
  padding: 12px 16px;
  width: 100%;
  height: fit-content;
  overflow: auto;
  background-color: var(--color-accent-soft);
  color: var(--color-accent);
  box-sizing: border-box;
  border-top: var(--border-width-thin) solid var(--color-border-muted);
  font-family: var(--font-code);
  font-size: 13px;
  line-height: 1.4;
}

#code-output::-webkit-scrollbar {
  height: 8px;
  width: 8px;
}

#code-output::-webkit-scrollbar-track {
  background: var(--color-accent-soft);
  border-radius: var(--radius-sm);
}

#code-output::-webkit-scrollbar-thumb {
  background: var(--color-accent);
  border-radius: var(--radius-sm);
}

#code-output::-webkit-scrollbar-thumb:hover {
  background: var(--color-lua);
}
</style>
