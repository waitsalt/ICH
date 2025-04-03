<script lang="ts" setup>
import { deleteIch, searchIch } from '@/api/ich'
import type { Ich } from '@/model/ich'
import { ref, onMounted, onUnmounted } from 'vue'

const ichList = ref<Ich[]>([])
const loading = ref(false)
const loadingMore = ref(false)
const hasMore = ref(true)
const searchParams = ref({
    keyword: '',
    page: 1,
    size: 10
})

// 获取非遗列表
const fetchIchList = async (reset = false) => {
    if (reset) {
        searchParams.value.page = 1
        hasMore.value = true
        loading.value = true
    } else {
        loadingMore.value = true
    }

    try {
        const response = await searchIch(searchParams.value)
        if (reset) {
            ichList.value = response.data
        } else {
            ichList.value = [...ichList.value, ...response.data]
        }

        // 检查是否还有更多数据
        hasMore.value = response.data.length >= searchParams.value.size
        if (hasMore.value) {
            searchParams.value.page++
        }
    } finally {
        loading.value = false
        loadingMore.value = false
    }
}

// 处理删除
const handleDelete = async (ichId: number) => {
    if (confirm('确定要删除这个非遗项目吗？')) {
        await deleteIch(ichId)
        fetchIchList(true) // 删除后重新加载第一页
    }
}

// 无限滚动处理
const handleScroll = () => {
    const nearBottom = window.innerHeight + window.scrollY >= document.body.offsetHeight - 500
    if (nearBottom && !loading.value && !loadingMore.value && hasMore.value) {
        fetchIchList()
    }
}

onMounted(() => {
    fetchIchList(true)
    window.addEventListener('scroll', handleScroll)
})

onUnmounted(() => {
    window.removeEventListener('scroll', handleScroll)
})
</script>

<template>
    <div class="ich-management">
        <div class="header">
            <h1>非遗物质文化</h1>
            <div class="controls">
                <div class="search-box">
                    <input v-model="searchParams.keyword" placeholder="搜索非遗项目..." @keyup.enter="fetchIchList(true)">
                    <button class="action-btn" @click="fetchIchList(true)">搜索</button>
                </div>
                <button to="/ich/create" class="create-btn action-btn">新增项目</button>
            </div>
        </div>

        <div v-if="loading" class="loading">加载中...</div>

        <div v-else class="ich-grid">
            <div v-for="item in ichList" :key="item.ich_id" class="ich-card">
                <div class="card-header">
                    <h3>{{ item.ich_name }}</h3>
                    <span class="ich-code">{{ item.ich_code }}</span>
                </div>

                <div class="card-body">
                    <div class="info-item">
                        <label>类别:</label>
                        <span>{{ item.ich_category }}</span>
                    </div>
                    <div class="info-item">
                        <label>地区:</label>
                        <span>{{ item.ich_location }}</span>
                    </div>
                    <div class="info-item">
                        <label>申报地区:</label>
                        <span>{{ item.ich_apply_location }}</span>
                    </div>
                </div>

                <div class="card-actions">
                    <router-link :to="`/ich/${item.ich_id}`" class="action-btn">查看</router-link>
                    <router-link :to="`/ich/${item.ich_id}/edit`" class="action-btn">编辑</router-link>
                    <button @click="handleDelete(item.ich_id)" class="action-btn delete-btn">删除</button>
                </div>
            </div>
        </div>

        <div v-if="loadingMore" class="loading-more">正在加载更多数据...</div>
        <div v-if="!hasMore && ichList.length > 0" class="no-more">没有更多数据了</div>
        <div v-if="!loading && ichList.length === 0" class="no-data">暂无数据</div>
    </div>
</template>

<style scoped>
.ich-management {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem;
}

.header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
    flex-wrap: wrap;
    gap: 1rem;
}

.controls {
    display: flex;
    gap: 1rem;
    align-items: center;
}

.search-box {
    display: flex;
    gap: 0.5rem;
}

.search-box input {
    padding: 0.5rem 1rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    min-width: 250px;
}

.search-box button {
    padding: 0.5rem 1rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    background: none;
    cursor: pointer;
}

.create-btn {
    padding: 0.5rem 1rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    text-decoration: none;
    color: #333;
}

.loading,
.loading-more,
.no-more,
.no-data {
    text-align: center;
    padding: 2rem;
    color: #666;
}

/* 卡片网格布局 */
.ich-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 1.5rem;
    margin-bottom: 2rem;
}

.ich-card {
    border: 1px solid #eee;
    border-radius: 8px;
    padding: 1.5rem;
    transition: all 0.2s ease;
}

.ich-card:hover {
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
    transform: translateY(-2px);
}

.card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
    padding-bottom: 1rem;
    border-bottom: 1px solid #eee;
}

.card-header h3 {
    margin: 0;
    font-size: 1.2rem;
    color: #333;
}

.ich-code {
    font-size: 0.8rem;
    color: #666;
    background: #f5f5f5;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
}

.card-body {
    margin-bottom: 1.5rem;
}

.info-item {
    display: flex;
    margin-bottom: 0.75rem;
    font-size: 0.95rem;
}

.info-item label {
    font-weight: 500;
    min-width: 80px;
    color: #666;
}

.info-item span {
    flex: 1;
}

.card-actions {
    display: flex;
    gap: 0.75rem;
    border-top: 1px solid #eee;
    padding-top: 1rem;
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

.delete-btn {
    color: #f44336;
    border-color: #ffcdd2;
    margin-left: auto;
}

.delete-btn:hover {
    background: #ffebee;
}

/* 响应式调整 */
@media (max-width: 768px) {
    .header {
        flex-direction: column;
        align-items: flex-start;
    }

    .controls {
        width: 100%;
        flex-direction: column;
    }

    .search-box {
        width: 100%;
    }

    .search-box input {
        flex: 1;
        min-width: auto;
    }

    .create-btn {
        width: 100%;
        text-align: center;
    }
}
</style>