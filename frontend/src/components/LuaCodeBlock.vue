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
    // Combine context with current code
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

    // Extract content between first parentheses
    const outputText = data.output || 'No output received'
    const match = outputText.match(/\(([^)]*)\)/)
    output.value = match ? match[1] : outputText
  } catch (error) {
    console.error('Error running Lua code:', error)
    output.value = `Error: ${error.message}`
  }
}

onMounted(() => {
  // Highlight the code
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
  background-color: rgba(66, 184, 131, 0.1);
  color: #f5f5f5;
  border: 1px solid #42b883;
  border-bottom: none;
  font-size: 14px;
  font-weight: bold;
  font-family: 'Departure Mono';
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

#cell-id {
  color: grey;
}

#button-group {
  display: flex;
  gap: 18px;
}

button {
  padding: 6px 6px;
  background-color: #42b883;
  color: #1a1a1a;
  border-radius: 1px;
  border: none;
  font-family: 'Departure Mono';
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
  background-color: #5ce2fa;
  color: #1a1a1a;
  transform: translateY(-1px);
}

/* Change max-width and margin here for
changing the whole block without breaking */
#code-container {
  display: flex;
  flex-direction: column;
  width: 100%;
  max-width: 85%;
  margin: 1em auto;
  border: 2px solid #42b883;
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
  font-family: 'Departure Mono', monospace;
  font-size: 14px;
  line-height: 1.6;
  background-color: #1a1a1a;
  border: none;
  box-sizing: border-box;
  overflow-x: auto;
  min-height: 60px;
}

#code-display::-webkit-scrollbar {
  height: 8px;
}

#code-display::-webkit-scrollbar-track {
  background: #2a2a2a;
  border-radius: 4px;
}

#code-display::-webkit-scrollbar-thumb {
  background: #42b883;
  border-radius: 4px;
}

#code-display::-webkit-scrollbar-thumb:hover {
  background: #5ce2fa;
}

#code-display :deep(*) {
  background: transparent !important;
}

#code-display code {
  font-family: inherit;
  color: #f5f5f5;
  background: transparent !important;
}

#code-output {
  margin: 0;
  padding: 12px 16px;
  width: 100%;
  height: fit-content;
  overflow: auto;
  background-color: rgba(92, 226, 250, 0.1);
  color: #5ce2fa;
  box-sizing: border-box;
  border-top: 1px solid #333;
  font-family: 'Departure Mono';
  font-size: 13px;
  line-height: 1.4;
}

#code-output::-webkit-scrollbar {
  height: 8px;
  width: 8px;
}

#code-output::-webkit-scrollbar-track {
  background: rgba(92, 226, 250, 0.1);
  border-radius: 4px;
}

#code-output::-webkit-scrollbar-thumb {
  background: #5ce2fa;
  border-radius: 4px;
}

#code-output::-webkit-scrollbar-thumb:hover {
  background: #42b883;
}
</style>
