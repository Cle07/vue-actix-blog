<script setup>
import { ref, onMounted, onBeforeUnmount } from 'vue'
import { toggleTheme } from '@/main'

const timer = ref(null)
const time = ref(
  new Date().toLocaleTimeString('en-US', {
    hour12: false,
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
    fractionalSecondDigits: 3,
  }),
)

const themeIcon = () =>
  document.documentElement.dataset.theme === 'dark' ? '☀' : '☾'

const currentThemeIcon = ref(themeIcon())

const switchTheme = () => {
  toggleTheme()
  currentThemeIcon.value = themeIcon()
}

onMounted(() => {
  timer.value = setInterval(() => {
    const now = new Date()
    const timeString = now.toLocaleTimeString('en-US', {
      hour12: false,
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
    })
    const ms = Math.floor(now.getMilliseconds() / 100)
    time.value = `${timeString}.${ms}`
  }, 100)
})

onBeforeUnmount(() => {
  clearInterval(timer.value)
})
</script>
<template>
  <nav ref="navbarRef">
    <div id="outer-container">
      <div class="inner-container">
        <div id="#home" class="grid-item">
          <p class="home-cell">
            <router-link to="/">
              <img
                src="@/assets/home_pixel.svg"
                alt="Home"
                height="24"
                width="24"
              />
            </router-link>
          </p>
        </div>
        <div id="#route-cell" class="grid-item">
          <p style="white-space: nowrap">
            <router-link to="/">{{
              decodeURI($route.path).split('.').slice(0, -1).join('.') || decodeURI($route.path)
            }}</router-link>
          </p>
        </div>
        <div id="#time-cell" class="grid-item">
          <p>{{ time }}</p>
        </div>
      </div>
      <div class="inner-container">
        <div class="grid-item">
          <p><router-link to="/about">About this site</router-link></p>
        </div>
        <div class="grid-item">
          <p><router-link to="/lua_playground">Lua Playground</router-link></p>
        </div>
        <div class="grid-item">
          <button id="theme-switch" @click="switchTheme" aria-label="Switch theme">
            {{ currentThemeIcon }} theme
          </button>
        </div>
      </div>
    </div>
  </nav>
</template>
<style scoped>
nav {
  position: fixed;
  top: 0;
  left: 50%;
  transform: translateX(-50%);
  width: 100%;
  max-width: var(--content-max-width);
  height: auto;
  z-index: 1000;
  background-color: var(--color-bg);
  display: flex;
  align-items: center;
  border: var(--border-width) solid var(--color-border);
}

#outer-container {
  display: flex;
  flex-direction: column;
  width: 100%;
  background-color: var(--color-bg);
  gap: 0;
  align-items: center;
  justify-content: center;
}

.inner-container {
  display: flex;
  flex-direction: row;
  align-items: stretch;
  justify-content: space-between;
  width: 100%;
}

@media (max-width: 768px) {
  .inner-container {
    flex-direction: row;
    gap: 0;
  }

  nav {
    height: auto;
  }

  p {
    padding: 0.3rem;
    font-size: var(--font-size-xs);
  }
}

.grid-item {
  flex: 1;
  display: flex;
}

p {
  border: var(--border-width) solid var(--color-border);
  padding: 0.7rem;
  margin: 0;
  text-align: center;
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-family: var(--font-body);
  font-size: var(--font-size-base);
  background-color: var(--color-bg);
}

a {
  color: var(--color-fg);
  text-decoration: none;
  width: 100%;
  display: block;
}

/* Cellule home — icône teintée par thème */
.home-cell a {
  display: flex;
  align-items: center;
  justify-content: center;
}

.home-cell img {
  filter: var(--home-icon-filter);
}

p:hover,
.home-cell:hover {
  background-color: var(--color-bg-soft);
}

/* Bouton thème — même chrome que les cellules <p> */
#theme-switch {
  border: var(--border-width) solid var(--color-border);
  padding: 0.7rem;
  margin: 0;
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.4em;
  font-family: var(--font-body);
  font-size: var(--font-size-base);
  background-color: var(--color-bg);
  color: var(--color-fg);
  cursor: pointer;
  transition:
    background-color 0.2s,
    color 0.2s;
}

#theme-switch:hover {
  background-color: var(--color-accent-soft);
  color: var(--color-accent);
}
</style>
