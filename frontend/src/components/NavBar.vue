<script setup>
import { ref, onMounted, onBeforeUnmount, nextTick } from 'vue'

const time = ref(new Date().toLocaleTimeString('en-US', {hour12: false, hour: '2-digit', minute: '2-digit', second: '2-digit', fractionalSecondDigits: 3}))
const timer = ref(null)

onMounted(() => {
    // Update time every 100ms (0.1 seconds)
    timer.value = setInterval(() => {
        const now = new Date()
        const timeString = now.toLocaleTimeString('en-US', {hour12: false, hour: '2-digit', minute: '2-digit', second: '2-digit'})
        const ms = Math.floor(now.getMilliseconds() / 100)
        time.value = `${timeString}.${ms}`
    }, 100)
})

onBeforeUnmount(() => {
    // Clean up interval when component is unmounted
    clearInterval(timer)
    
})
</script>
<template>
<nav ref="navbarRef">
    <div id="outer-container">
        <div class="inner-container">
            <div id="#twitter-cell" class="grid-item">
                <p>
                    <router-link to="/">
                        <img src="@/assets/home_pixel.svg" alt="Home" height="24" width="24" style="filter: invert(100%);">
                    </router-link>
                </p>
            </div>
            <div id="#route-cell" class="grid-item"><p style="white-space: nowrap;"><router-link to="/">{{  $route.path }}</router-link></p></div>
            <div id="#time-cell" class="grid-item"><p>{{ time }}</p></div>
            <div id="#twitter-cell" class="grid-item">
                <p>
                    <a href="https://twitter.com" target="_blank" rel="noopener noreferrer">
                        <img src="@/assets/twitter_pixel.svg" alt="Twitter" height="24" width="24" style="filter: brightness(0) saturate(100%) invert(48%) sepia(90%) saturate(2299%) hue-rotate(194deg) brightness(101%) contrast(101%);">
                    </a>
                </p>
            </div>
        </div>
        <div class="inner-container">
            <div class="grid-item"><p><router-link to="/about">About</router-link></p></div>
            <div class="grid-item"><p><router-link to="/timeline">Timeline</router-link></p></div>

            <div class="grid-item"><p><router-link to="/lua">Lua Stuff</router-link></p></div>
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
    width: 100% ;
    max-width: 900px;
    height: auto;
    z-index: 1000;
    background-color: #111314;
    display: flex;
    align-items: center;
    border: 3px solid #f5f5f5;
}

#outer-container {
    display: flex;
    flex-direction: column;
    width: 100%;
    background-color: #111314;
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
        font-size: 0.9rem;
    }
}

.grid-item {
    flex: 1;
    display: flex;
}

p {
    border: 3px solid #f5f5f5;
    padding: 0.7rem;
    margin: 0;
    text-align: center;
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-family: 'Departure Mono', monospace;
    font-size: 0.95rem;
    background-color: #111314;

}

a {
    color: #f5f5f5;
    text-decoration: none;
    width: 100%;
    display: block;
    
}
</style>