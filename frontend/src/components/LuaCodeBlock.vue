<script setup>
import { ref, onMounted, watch } from 'vue'
import Prism from 'prismjs'
import '@/assets/prism.css'
import 'prismjs/components/prism-lua'

const props = defineProps({
  defaultCode: {
    type: String,
    default: 'print("Hello, World!")',
  },
})

const code = ref('defaultCode' in props ? props.defaultCode : 'print("Hello, World!")')
const output = ref('This is a component for the Lua computer.')
const hasRun = ref(false)

const updateHighlight = () => {
  const highlighted = Prism.highlight(code.value, Prism.languages.lua, 'lua')
  document.querySelector('#highlighted-code').innerHTML = highlighted
}

const resetCode = () => {
  code.value = props.defaultCode
  output.value = 'Code has been reset.'
}

const runCode = async () => {
  hasRun.value = true
  output.value = 'Running Lua code...'
  try {
    console.log('Sending code:', code.value)
    const response = await fetch('/api/lua/run', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ code: code.value }),
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
  // Initialize content and highlighting
  updateHighlight()
  // Add event listeners to buttons
  document.querySelector('#run-button').addEventListener('click', runCode)
  document.querySelector('#reset-button').addEventListener('click', resetCode)
})

watch(code, () => {
  updateHighlight()
})
</script>
<template>
  <div id="code-container">
    <div id="bar">
      <b>Lua Editor</b>
      <button id="run-button">Run</button>
      <button id="reset-button">Reset</button>
    </div>
    <div id="code-editor">
      <div class="textarea-wrapper">
        <textarea
          id="code-area"
          v-model="code"
          @input="updateHighlight"
          spellcheck="false"
        ></textarea>
        <pre
          id="highlighting"
          aria-hidden="true"
        ><code id="highlighted-code" class="language-lua"></code></pre>
      </div>
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
  flex: 0 0 auto;
  padding: 10px;
  background-color: #111314;
  color: #f5f5f5;
  border: 2px solid #f5f5f5;
  font-size: small;
}

#reset-button,
#run-button {
  padding: 5px 10px;
  background-color: inherit;
  color: #f5f5f5;
  border-radius: 1px;
  border: 1px solid #f5f5f5;
  font-family: inherit;
}

#reset-button:hover {
  background-color: white;
}

#code-container {
  display: flex;
  flex-direction: column;
  width: 100%;
}

#code-editor {
  display: flex;
  flex-direction: column;
  width: 100%;
}

.textarea-wrapper {
  position: relative;
  width: 100%;
  flex: 1 1 auto;
}

#code-area,
#highlighting {
  margin: 0;
  padding: 10px;
  border: 0;
  width: 100%;
  height: fit-content;
  font-family: 'Departure Mono', monospace;
  font-size: 15px;
  line-height: 1.5;
  overflow: auto;
  white-space: pre;
  box-sizing: border-box;
}

#code-area {
  position: absolute;
  top: 0;
  left: 0;
  color: transparent;
  background: transparent;
  caret-color: #f5f5f5;
  resize: none;
  z-index: 1;
  height: 100%;
  min-height: 100%;
  border: 1px solid #f5f5f5;
  scrollbar-width: none;
  -ms-overflow-style: none;
}

#highlighting {
  z-index: 0;
}

#code-output {
  margin: 0;
  padding: 10px;
  width: 100%;
  height: fit-content;
  overflow: auto;
  background-color: #111314;
  box-sizing: border-box;
  border: 1px solid #f5f5f5;
  font-family: 'Departure Mono';
  font-size: 15px;
}
</style>
