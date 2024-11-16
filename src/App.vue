<template>
    <main class="container">
        <div class="title">{{ updateMessage }}</div>
        <div class="progress">
            <div class="progress-bar" :class="{ 'completed': isCompleted }"></div>
        </div>
    </main>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'

declare global {
    interface Window {
        updateTitle: (title: string) => void
        updateCompleted: () => void
    }
}

const updateMessage = ref('Installing updates, please wait...')
const isCompleted = ref(false)

onMounted(() => {
    window.updateTitle = (title: string) => {
        if (title) {
            updateMessage.value = title
        }
    }
    window.updateCompleted = () => {
        const progressBar = document.querySelector('.progress-bar') as HTMLElement
        const currentWidth = getComputedStyle(progressBar).width
        progressBar.style.animation = 'none'
        progressBar.style.width = currentWidth
        progressBar.offsetHeight
        isCompleted.value = true
    }
})

// 监听系统主题变化
const darkModeMediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
const updateTheme = (e: MediaQueryListEvent | MediaQueryList) => {
    document.documentElement.classList.toggle('dark', e.matches);
};

// 初始化主题
updateTheme(darkModeMediaQuery);
// 监听主题变化
darkModeMediaQuery.addEventListener('change', updateTheme);
</script>

<style scoped>
.container {
    position: fixed;
    top: 0;
    left: 10%;
    right: 10%;
    bottom: 0;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
}

.title {
    font-size: 15px;
    color: var(--text-color);
    margin-bottom: 20px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 100%;
}

.progress {
    width: 260px;
    max-width: 100%;
    height: 6px;
    background: var(--progress-bg);
    border-radius: 2px;
    margin: 0 auto;
    overflow: hidden;
}

.progress-bar {
    width: 0%;
    height: 100%;
    background: var(--progress-bar-color);
    border-radius: 2px;
    animation: progress 30s cubic-bezier(0.1, 0.5, 0.2, 1) forwards;
}

.progress-bar.completed {
    animation: none;
    transition: width 0.3s ease-out;
    width: 100% !important;
}

@keyframes progress {
    0% {
        width: 0%;
    }
    50% {
        width: 75%;
    }
    100% {
        width: 99%;
    }
}
</style>

<style>
:root {
    /* 亮色模式变量 */
    --text-color: #333;
    --bg-color: #f6f6f6;
    --progress-bg: #f0f0f0;
    --progress-bar-color: #1890ff;

    font-family: "Helvetica Neue", Helvetica, "PingFang SC", "Hiragino Sans GB", "Microsoft YaHei", 微软雅黑, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: var(--text-color);
    background-color: var(--bg-color);

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;

    overscroll-behavior: none;
}

/* 深色模式变量 */
:root.dark {
    --text-color: #ffffff;
    --bg-color: #1a1a1a;
    --progress-bg: #333333;
    --progress-bar-color: #1890ff;
}
</style>
