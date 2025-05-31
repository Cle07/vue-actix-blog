import { createApp } from 'vue'
import App from './App.vue'
import router from './router'

createApp(App).use(router).mount('#app')

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

  // Process Obsidian image links: ![[image.jpg]] -> <img src="/assets/images/image.jpg" alt="image.jpg" class="obsidian-image">
  processedContent = processedContent.replace(/!\[\[(.*?)\]\]/g, (match, imagePath) => {
    const path = imagePath.trim()
    return `<img src="/images/${path}" alt="${path}" class="obsidian-image">`
  })

  // Process regular Obsidian links: [[Page]] -> <router-link to="/article/Page">Page</router-link>
  // or [[Page|Custom text]] -> <router-link to="/article/Page">Custom text</router-link>
  processedContent = processedContent.replace(
    /\[\[(.*?)(?:\|(.*?))?\]\]/g,
    (match, link, alias) => {
      const displayText = alias ? alias.trim() : link.trim()
      const encodedLink = encodeURIComponent(link.trim())

      if (options.useRouterLinks) {
        return `<a href="#" data-link="${encodedLink}" class="obsidian-link custom-link">${displayText}</a>`
      }
    },
  )

  // Process footnotes: ^[footnote text] -> <sup class="footnote-ref"><a href="#footnote-1" id="footnote-ref-1">[1]</a></sup>
  processedContent = processedContent.replace(/\^\[(.*?)\]/g, (match, footnoteText) => {
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

  return processedContent
}
