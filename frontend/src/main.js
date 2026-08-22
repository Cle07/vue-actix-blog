import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import katex from 'katex'
import 'katex/dist/katex.min.css'
import '@/assets/main.css'

createApp(App).use(router).mount('#app')

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

  // Parse display math: $$...$$ (must be before $...)
  result = result.replace(/\$\$([\s\S]*?)\$\$/g, (_, formula) => {
    try {
      return '<div class="katex-display">' + katex.renderToString(formula.trim(), { displayMode: true, throwOnError: false }) + '</div>'
    } catch (e) {
      return `<span class="katex-error" title="${e.message}">${_.replace(/</g, '&lt;').replace(/>/g, '&gt;')}</span>`
    }
  })

  // Parse inline math: $...$
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

  // Process Obsidian image links: ![[image.jpg]] -> <img src="/images/image.jpg" alt="image.jpg" class="obsidian-image">
  processedContent = processedContent.replace(/!\[\[(.*?)\]\]/g, (_, imagePath) => {
    const path = imagePath.trim()
    return `<img src="/images/${path}" alt="${path}" class="obsidian-image">`
  })

  // Process regular Obsidian links: [[Page]] -> router-link
  // or [[Page|Custom text]] -> router-link with alias
  processedContent = processedContent.replace(/\[\[(.*?)(?:\|(.*?))?\]\]/g, (_, link, alias) => {
    const displayText = alias ? alias.trim() : link.trim()
    const trimmedLink = link.trim()
    const encodedLink = encodeURIComponent(trimmedLink)

    const routePath = `/article/${encodedLink}`

    if (options.useRouterLinks) {
      return `<a href="#" data-link="${encodedLink}" data-route="${routePath}" class="obsidian-link custom-link">${displayText}</a>`
    }
  })

  // Process footnotes: ^[footnote text] -> <sup class="footnote-ref"><a href="#footnote-1" id="footnote-ref-1">[1]</a></sup>
  processedContent = processedContent.replace(/^\[(.*?)\]$/g, (_, footnoteText) => {
    const footnoteId = footnotes.length + 1
    footnotes.push(footnoteText.trim())
    return `<sup class="footnote-ref"><a href="#footnote-${footnoteId}" id="footnote-ref-${footnoteId}">[${footnoteId}]</a></sup>`
  })

  // Add footnotes section if any exist
  if (footnotes.length > 0) {
    let footnoteSection = '<hr><div class="footnotes"><ol>'

    footnotes.forEach((text, index) => {
      const footnoteId = index + 1
      footnoteSection += `<li id="footnote-${footnoteId}">${text} <a href="#footnote-ref-${footnoteId}">↩</a></li>`
    })

    footnoteSection += '</ol></div>'
    processedContent += footnoteSection
  }

  // Parse LaTeX math expressions ($...$, $$...$$)
  processedContent = parseLatex(processedContent)

  return processedContent
}
