<script lang="ts" setup>
import { useRouter } from 'vue-router'

const router = useRouter()

const goHome = () => {
    router.push('/')
}
</script>

<template>
    <div class="not-found-container">
        <h1>404</h1>
        <p>抱歉，您访问的页面不存在</p>
        <button @click="goHome">返回首页</button>
    </div>
</template>

<style scoped>
.not-found-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100vh;
    text-align: center;
}

h1 {
    font-size: 5rem;
    margin-bottom: 1rem;
    color: #333;
}

p {
    font-size: 1.2rem;
    margin-bottom: 2rem;
    color: #666;
}

button {
    padding: 0.75rem 1.5rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    background: none;
    cursor: pointer;
    font-size: 1rem;
}

button:hover {
    background: #f5f5f5;
}
</style>