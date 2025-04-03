<script lang="ts" setup>
import { searchIch } from '@/api/ich'
import { searchPost } from '@/api/post'
import type { Ich } from '@/model/ich'
import type { Post } from '@/model/post'
import { ref, onMounted } from 'vue'
// 精选非遗项目
const featuredIch = ref<Ich[]>([])
// 最新帖子
const latestPosts = ref<Post[]>([])

// 加载数据
const loadData = async () => {
    try {
        // 获取3个精选非遗项目
        const ichRes = await searchIch({
            keyword: '',
            page: 1,
            size: 3
        })
        featuredIch.value = ichRes.data

        // 获取4篇最新帖子
        const postRes = await searchPost({
            keyword: '',
            page: 1,
            size: 4
        })
        latestPosts.value = postRes.data
    } catch (error) {
        console.error('加载首页数据失败:', error)
    }
}

onMounted(() => {
    loadData()
})
</script>

<template>
    <div class="home-view">
        <!-- 欢迎横幅 -->
        <section class="welcome-banner">
            <h1>传承非遗文化</h1>
            <p>探索、记录、分享非物质文化遗产的独特魅力</p>
            <div class="action-buttons">
                <router-link to="/ich" class="banner-button">浏览非遗</router-link>
                <router-link to="/posts" class="banner-button">查看分享</router-link>
            </div>
        </section>

        <!-- 精选非遗项目 -->
        <section class="featured-section">
            <h2>精选非遗项目</h2>
            <div class="ich-grid">
                <div v-for="item in featuredIch" :key="item.ich_id" class="ich-card">
                    <router-link :to="`/ich/${item.ich_id}`">
                        <h3>{{ item.ich_name }}</h3>
                        <div class="ich-meta">
                            <span>{{ item.ich_category }}</span>
                            <span>{{ item.ich_location }}</span>
                        </div>
                        <p class="ich-desc">{{ item.ich_content.substring(0, 60) }}...</p>
                    </router-link>
                </div>
            </div>
            <router-link to="/ich" class="view-all">查看全部非遗项目 →</router-link>
        </section>

        <!-- 最新分享 -->
        <section class="posts-section">
            <h2>最新分享</h2>
            <div class="posts-grid">
                <div v-for="post in latestPosts" :key="post.post_id" class="post-card">
                    <router-link :to="`/posts/${post.post_id}`">
                        <h3>{{ post.post_title }}</h3>
                        <div class="post-meta">
                            <span>{{ new Date(post.post_create_time).toLocaleDateString() }}</span>
                            <span>❤️ {{ post.post_likes }}</span>
                        </div>
                        <p class="post-excerpt">{{ post.post_content.substring(0, 80) }}...</p>
                    </router-link>
                </div>
            </div>
            <router-link to="/posts" class="view-all">查看全部帖子 →</router-link>
        </section>

        <!-- 平台特色 -->
        <section class="features-section">
            <h2>平台特色</h2>
            <div class="features-grid">
                <div class="feature-card">
                    <h3>全面记录</h3>
                    <p>系统化记录非物质文化遗产的详细信息</p>
                </div>
                <div class="feature-card">
                    <h3>社区分享</h3>
                    <p>传承人、研究者与爱好者共同交流</p>
                </div>
                <div class="feature-card">
                    <h3>数字保护</h3>
                    <p>通过数字化手段保存珍贵文化遗产</p>
                </div>
            </div>
        </section>
    </div>
</template>

<style scoped>
.home-view {
    max-width: 1200px;
    margin: 0 auto;
    padding: 0 1rem;
}

/* 欢迎横幅 */
.welcome-banner {
    text-align: center;
    padding: 4rem 1rem;
    margin-bottom: 3rem;
    border-bottom: 1px solid #eee;
}

.welcome-banner h1 {
    font-size: 2.5rem;
    margin-bottom: 1rem;
}

.welcome-banner p {
    font-size: 1.2rem;
    color: #666;
    margin-bottom: 2rem;
}

.action-buttons {
    display: flex;
    justify-content: center;
    gap: 1rem;
}

.banner-button {
    padding: 0.75rem 1.5rem;
    border: 1px solid #333;
    border-radius: 4px;
    text-decoration: none;
    color: #333;
    transition: all 0.2s;
}

.banner-button:hover {
    background: #f5f5f5;
}

/* 内容区块通用样式 */
section {
    margin-bottom: 4rem;
}

h2 {
    font-size: 1.8rem;
    margin-bottom: 2rem;
    padding-bottom: 0.5rem;
    border-bottom: 1px solid #eee;
}

.view-all {
    display: block;
    text-align: right;
    margin-top: 1rem;
    text-decoration: none;
    color: #666;
}

.view-all:hover {
    color: #333;
    text-decoration: underline;
}

/* 非遗项目网格 */
.ich-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 1.5rem;
    margin-bottom: 1rem;
}

.ich-card {
    border: 1px solid #eee;
    border-radius: 4px;
    padding: 1.5rem;
    transition: all 0.2s;
}

.ich-card:hover {
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.ich-card h3 {
    margin: 0 0 0.5rem 0;
    font-size: 1.2rem;
}

.ich-meta {
    display: flex;
    gap: 1rem;
    font-size: 0.9rem;
    color: #666;
    margin-bottom: 1rem;
}

.ich-desc {
    color: #444;
    line-height: 1.5;
}

/* 帖子网格 */
.posts-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
    gap: 1.5rem;
}

.post-card {
    border: 1px solid #eee;
    border-radius: 4px;
    padding: 1.5rem;
    transition: all 0.2s;
}

.post-card:hover {
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.post-card h3 {
    margin: 0 0 0.5rem 0;
    font-size: 1.1rem;
}

.post-meta {
    display: flex;
    gap: 1rem;
    font-size: 0.8rem;
    color: #666;
    margin-bottom: 1rem;
}

.post-excerpt {
    color: #444;
    line-height: 1.5;
    font-size: 0.9rem;
}

/* 特色区块 */
.features-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 2rem;
}

.feature-card {
    text-align: center;
    padding: 2rem 1rem;
    border: 1px solid #eee;
    border-radius: 4px;
}

.feature-card h3 {
    font-size: 1.3rem;
    margin-bottom: 1rem;
}

.feature-card p {
    color: #666;
    line-height: 1.5;
}

/* 响应式调整 */
@media (max-width: 768px) {
    .welcome-banner {
        padding: 2rem 1rem;
    }

    .action-buttons {
        flex-direction: column;
        align-items: center;
    }

    .banner-button {
        width: 100%;
        max-width: 300px;
    }
}
</style>