<script setup lang="ts">
import { RouterView } from "vue-router";
import NavBar from "@/component/NavBar.vue";
</script>

<template>
    <NavBar />
    <RouterView />
</template>

<style>
/* 全局基础样式 */
body {
    margin: 0;
    font-family:
        -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen, Ubuntu,
        Cantarell, "Open Sans", "Helvetica Neue", sans-serif;
    color: #333;
    line-height: 1.5;
}

/* 全局链接样式 */
a {
    color: inherit;
    text-decoration: none;
}

/* 全局按钮样式 */
button {
    background: none;
    border: none;
    padding: 0;
    margin: 0;
    cursor: pointer;
}
</style>
