import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import katex from 'katex'
import 'katex/dist/katex.min.css'
import '@/assets/main.css'

createApp(App).use(router).mount('#app')

/* Thème : dark par défaut, persisté en localStorage, togglé via data-theme sur <html>. */
const THEME_KEY = 'theme'
const savedTheme = localStorage.getItem(THEME_KEY)
document.documentElement.dataset.theme = savedTheme === 'light' ? 'light' : 'dark'

export function toggleTheme() {
  const next = document.documentElement.dataset.theme === 'dark' ? 'light' : 'dark'
  document.documentElement.dataset.theme = next
  localStorage.setItem(THEME_KEY, next)
}

/**
 * Parse LaTeX math expressions using KaTeX
 *
 * Supports:
 * - Inline: $formula$ ... or \(formula\)
 * - Display: $$formula$$ ... or \[formula\]
 *
 * @param {string} text - Text potentially containing LaTeX
 * @returns {string} - Text with LaTeX replaced by rendered KaTeX HTML
 */
function parseLatex(text) {
  if (!text || typeof katex === 'undefined') return text

  let result = text

  // Display math d'abord : $$...$$
  result = result.replace(/\$\$([\s\S]*?)\$\$/g, (_, formula) => {
    try {
      return '<div class="katex-display">' + katex.renderToString(formula.trim(), { displayMode: true, throwOnError: false }) + '</div>'
    } catch (e) {
      return `<span class="katex-error" title="${e.message}">${_.replace(/</g, '&lt;').replace(/>/g, '&gt;')}</span>`
    }
  })

  // Inline math : $...$
  result = result.replace(/\$((?!\$)((?:[^$\\]|\\.)+))\$/g, (_, formula) => {
    try {
      return katex.renderToString(formula.trim(), { displayMode: false, throwOnError: false })
    } catch (e) {
      return `<span class="katex-error" title="${e.message}">${_.replace(/</g, '&lt;').replace(/>/g, '&gt;')}</span>`
    }
  })

  return result
}

/**
 * Parse Obsidian-style syntax in Markdown content and convert it to HTML
 *
 * @param {string} content - Markdown content with Obsidian syntax
 * @param {Object} options - Parsing options
 * @param {boolean} options.useRouterLinks - Whether to use router-link instead of standard links
 * @returns {string} - HTML content with processed Obsidian syntax
 */
export function parseObsidianLinks(content, options = { useRouterLinks: true }) {
  if (!content) return ''

  let processedContent = content
  const footnotes = []

  // Images : ![[image.jpg]] -> <img src="/images/...">
  processedContent = processedContent.replace(/!\[\[(.*?)\]\]/g, (_, imagePath) => {
    const path = imagePath.trim()
    return `<img src="/images/${path}" alt="${path}" class="obsidian-image">`
  })

  // Liens : [[Page]] ou [[Page|alias]] -> <a> routé
  processedContent = processedContent.replace(/\[\[(.*?)(?:\|(.*?))?\]\]/g, (_, link, alias) => {
    const displayText = alias ? alias.trim() : link.trim()
    const trimmedLink = link.trim()
    const encodedLink = encodeURIComponent(trimmedLink)

    const routePath = `/article/${encodedLink}`

    if (options.useRouterLinks) {
      return `<a href="#" data-link="${encodedLink}" data-route="${routePath}" class="obsidian-link custom-link">${displayText}</a>`
    }
  })

  // Footnotes : ^[texte] -> référence numérotée + section en fin
  processedContent = processedContent.replace(/\^\[(.*?)\]/g, (_, footnoteText) => {
    const footnoteId = footnotes.length + 1
    footnotes.push(footnoteText.trim())
    return `<sup class="footnote-ref"><a href="#footnote-${footnoteId}" id="footnote-ref-${footnoteId}">[${footnoteId}]</a></sup>`
  })

  if (footnotes.length > 0) {
    let footnoteSection = '<hr><div class="footnotes"><ol>'

    footnotes.forEach((text, index) => {
      const footnoteId = index + 1
      footnoteSection += `<li id="footnote-${footnoteId}">${text} <a href="#footnote-ref-${footnoteId}">↩</a></li>`
    })

    footnoteSection += '</ol></div>'
    processedContent += footnoteSection
  }

  // LaTeX ($...$, $$...$$)
  processedContent = parseLatex(processedContent)

  return processedContent
}
