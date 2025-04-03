<script lang="ts" setup>
import { getIch } from '@/api/ich'
import type { Ich } from '@/model/ich'
import { ref, onMounted } from 'vue'
import { useRoute } from 'vue-router'

const route = useRoute()
const ichId = parseInt(route.params.id as string)
const ichData = ref<Ich | null>(null)
const loading = ref(false)

onMounted(async () => {
    loading.value = true
    try {
        const response = await getIch(ichId)
        ichData.value = response.data
    } finally {
        loading.value = false
    }
})
</script>

<template>
    <div class="ich-detail">
        <div v-if="loading" class="loading">加载中...</div>

        <div v-else-if="ichData" class="detail-content">
            <h1>{{ ichData.ich_name }}</h1>

            <div class="detail-grid">
                <div class="detail-item">
                    <label>项目编号</label>
                    <div>{{ ichData.ich_code }}</div>
                </div>

                <div class="detail-item">
                    <label>公布时间</label>
                    <div>{{ ichData.ich_publish }}</div>
                </div>

                <div class="detail-item">
                    <label>类别</label>
                    <div>{{ ichData.ich_category }}</div>
                </div>

                <div class="detail-item">
                    <label>所在地</label>
                    <div>{{ ichData.ich_location }}</div>
                </div>

                <div class="detail-item">
                    <label>申报地区</label>
                    <div>{{ ichData.ich_apply_location }}</div>
                </div>

                <div class="detail-item">
                    <label>保护单位</label>
                    <div>{{ ichData.ich_protect_unit }}</div>
                </div>
            </div>

            <div class="content-section">
                <h3>项目内容</h3>
                <div class="content-text">{{ ichData.ich_content }}</div>
            </div>

            <div class="actions">
                <router-link :to="`/ich/${ichId}/edit`">编辑</router-link>
                <router-link to="/ich">返回列表</router-link>
            </div>
        </div>
    </div>
</template>

<style scoped>
.ich-detail {
    max-width: 900px;
    margin: 0 auto;
    padding: 2rem;
}

.loading {
    text-align: center;
    padding: 2rem;
}

.detail-content h1 {
    margin-bottom: 2rem;
    font-size: 1.8rem;
}

.detail-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 1.5rem;
    margin-bottom: 2rem;
}

.detail-item {
    padding: 1rem;
    border: 1px solid #eee;
    border-radius: 4px;
}

.detail-item label {
    display: block;
    font-weight: 500;
    margin-bottom: 0.5rem;
    color: #666;
}

.content-section {
    margin: 2rem 0;
}

.content-section h3 {
    margin-bottom: 1rem;
    font-size: 1.2rem;
}

.content-text {
    line-height: 1.6;
    white-space: pre-line;
}

.actions {
    display: flex;
    gap: 1rem;
    margin-top: 2rem;
}

.actions a {
    padding: 0.5rem 1rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    text-decoration: none;
    color: #333;
}

.actions a:hover {
    background: #f5f5f5;
}
</style>