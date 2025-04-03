<script lang="ts" setup>
import { deletePost, searchPost } from "@/api/post";
import type { Post } from "@/model/post";
import { ref, onMounted } from "vue";

const posts = ref<Post[]>([]);
const loading = ref(false);
const searchParams = ref({
    keyword: "",
    page: 1,
    size: 10,
});

const fetchPosts = async () => {
    loading.value = true;
    try {
        const response = await searchPost(searchParams.value);
        posts.value = response.data;
    } finally {
        loading.value = false;
    }
};

const handleDelete = async (postId: number) => {
    if (confirm("确定要删除这篇帖子吗？")) {
        await deletePost(postId);
        fetchPosts();
    }
};

onMounted(fetchPosts);
</script>

<template>
    <div class="post-list-container">
        <div class="header">
            <h1>文章</h1>
            <div class="search-box">
                <input
                    v-model="searchParams.keyword"
                    placeholder="搜索..."
                    @keyup.enter="fetchPosts"
                />
                <button @click="fetchPosts" class="action-btn">搜索</button>
            </div>
        </div>

        <div v-if="loading" class="loading">加载中...</div>

        <div v-else class="post-grid">
            <div v-for="post in posts" :key="post.post_id" class="post-card">
                <div class="post-header">
                    <h3>{{ post.post_title }}</h3>
                    <div class="post-meta">
                        <span>{{
                            new Date(post.post_create_time).toLocaleDateString()
                        }}</span>
                        <span>点赞: {{ post.post_likes }}</span>
                    </div>
                </div>
                <div class="post-content">
                    {{ post.post_content.substring(0, 100) }}...
                </div>
                <div class="post-actions">
                    <router-link :to="`/posts/${post.post_id}`"
                        >查看详情</router-link
                    >
                    <router-link :to="`/posts/${post.post_id}/edit`"
                        >编辑</router-link
                    >
                    <button @click="handleDelete(post.post_id)">删除</button>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
.post-list-container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem;
}

.header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
}

.search-box {
    display: flex;
    gap: 0.5rem;
}

.search-box input {
    padding: 0.5rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    min-width: 300px;
}

.loading {
    text-align: center;
    padding: 2rem;
}

.post-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 1.5rem;
}

.post-card {
    border: 1px solid #eee;
    border-radius: 4px;
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
    height: 100%;
}

.post-header {
    margin-bottom: 1rem;
}

.post-header h3 {
    margin: 0 0 0.5rem 0;
    font-size: 1.2rem;
}

.post-meta {
    display: flex;
    gap: 1rem;
    font-size: 0.8rem;
    color: #666;
}

.post-content {
    flex: 1;
    margin-bottom: 1rem;
    color: #444;
    line-height: 1.5;
}

.post-actions {
    display: flex;
    gap: 1rem;
    border-top: 1px solid #eee;
    padding-top: 1rem;
}

.post-actions a,
.post-actions button {
    color: #333;
    text-decoration: none;
    padding: 0.25rem 0.5rem;
    border: none;
    background: none;
    cursor: pointer;
}

.post-actions a:hover,
.post-actions button:hover {
    text-decoration: underline;
}

.action-btn {
    padding: 0.5rem 1rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    background: none;
    cursor: pointer;
    text-decoration: none;
    color: #333;
    font-size: 0.9rem;
    transition: all 0.2s;
}

.action-btn:hover {
    background: #f5f5f5;
}
</style>
