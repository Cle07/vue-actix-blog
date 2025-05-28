<script setup>
import { ref, onMounted, watch } from 'vue'
import Prism from 'prismjs'
import '@/assets/prism.css'
import 'prismjs/components/prism-lua'

// Default code for reset function
const props = defineProps({
  defaultCode: {
    type: String,
    default: 'print("Hello, World!")'
  }
})
const code = ref('defaultCode' in props ? props.defaultCode : 'print("Hello, World!")')
const output = ref('This is a component for the Lua computer.')

// Function to update the highlighted code
const updateHighlight = () => {
  const highlighted = Prism.highlight(code.value, Prism.languages.lua, 'lua')
  document.querySelector('#highlighted-code').innerHTML = highlighted
}

onMounted(() => {
  // Initialize content and highlighting
  updateHighlight()
});

// Re-highlight whenever code changes
watch(code, () => {
  updateHighlight()
})

// Function to run Lua code
const runCode = async () => {
  try {
    console.log('Sending code:', code.value);
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
    console.log('Response data:', data);
    
    // Extract content between first parentheses
    const outputText = data.output || 'No output received'
    const match = outputText.match(/\(([^)]*)\)/)
    output.value = match ? match[1] : outputText
  } catch (error) {
    console.error('Error running Lua code:', error);
    output.value = `Error: ${error.message}`
  }
}

// Function to reset the code
const resetCode = () => {
  code.value = defaultCode
  output.value = 'Code has been reset.'
}

// Add event listeners when mounted
onMounted(() => {
  // Initialize content and highlighting
  updateHighlight()
  
  // Add event listeners to buttons
  document.querySelector('#run-button').addEventListener('click', runCode)
  document.querySelector('#reset-button').addEventListener('click', resetCode)
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
        <pre id="highlighting" aria-hidden="true"><code id="highlighted-code" class="language-lua"></code></pre>
      </div>
      <pre id="code-output">{{ output }}</pre>
    </div>
  </div>
</template>

<style scoped>
  #bar {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    flex: 0 0 auto;
    padding: 10px;
    background-color: #222;
    color: #f5f5f5;
    border-radius: 5px;
  }

  button {
    padding: 5px 10px;
    background-color: #222;
    color: #f5f5f5;
    border-radius: 5px;
  }

  #code-container {
    display: flex;
    flex-direction: column;
    width: 100%;
  }

  #code-editor {
    display: flex;
    flex-direction: row;
    width: 100%;
  }

  .textarea-wrapper {
    position: relative;
    width: 50%;
    flex: 1 1 50%;
    max-width: 50%;
  }

  #code-area, #highlighting {
    margin: 0;
    padding: 10px;
    border: 0;
    width: 100%;
    height: 300px;
    font-family: monospace;
    font-size: 14px;
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
  }
  
  #highlighting {
    z-index: 0;
  }
  
  #code-output {
    margin: 0;
    padding: 10px;
    width: 50%;
    flex: 1 1 50%;
    max-width: 50%;
    height: 300px;
    overflow: auto;
    background-color: #000000;
    box-sizing: border-box;
  }
</style>