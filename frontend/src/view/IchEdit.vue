<script lang="ts" setup>
import { createIch, getIch, updateIch } from '@/api/ich'
import type { IchCreatePayload, IchUpdatePayload } from '@/model/ich'
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'

const route = useRoute()
const router = useRouter()
const isEditMode = ref(false)
const loading = ref(false)
const ichId = ref(0)

const formData = ref<IchCreatePayload | IchUpdatePayload>({
    ich_code: '',
    ich_name: '',
    ich_publish: '',
    ich_category: '',
    ich_location: '',
    ich_apply_location: '',
    ich_protect_unit: '',
    ich_content: ''
})

const fetchIchData = async (id: number) => {
    loading.value = true
    try {
        const response = await getIch(id)
        Object.assign(formData.value, response.data)
    } finally {
        loading.value = false
    }
}

const handleSubmit = async () => {
    loading.value = true
    try {
        if (isEditMode.value) {
            await updateIch(ichId.value, formData.value as IchUpdatePayload)
        } else {
            await createIch(formData.value)
        }
        router.push('/ich')
    } finally {
        loading.value = false
    }
}

onMounted(() => {
    if (route.params.id) {
        isEditMode.value = true
        ichId.value = parseInt(route.params.id as string)
        fetchIchData(ichId.value)
    }
})
</script>

<template>
    <div class="ich-edit">
        <h1>{{ isEditMode ? '编辑非遗项目' : '新增非遗项目' }}</h1>

        <form @submit.prevent="handleSubmit" class="form-container">
            <div class="form-group">
                <label>项目编号</label>
                <input v-model="formData.ich_code" required>
            </div>

            <div class="form-group">
                <label>项目名称</label>
                <input v-model="formData.ich_name" required>
            </div>

            <div class="form-group">
                <label>公布时间</label>
                <input v-model="formData.ich_publish" type="date">
            </div>

            <div class="form-group">
                <label>类别</label>
                <input v-model="formData.ich_category">
            </div>

            <div class="form-group">
                <label>所在地</label>
                <input v-model="formData.ich_location">
            </div>

            <div class="form-group">
                <label>申报地区</label>
                <input v-model="formData.ich_apply_location">
            </div>

            <div class="form-group">
                <label>保护单位</label>
                <input v-model="formData.ich_protect_unit">
            </div>

            <div class="form-group">
                <label>项目内容</label>
                <textarea v-model="formData.ich_content" rows="6"></textarea>
            </div>

            <div class="form-actions">
                <button type="submit" :disabled="loading">
                    {{ loading ? '提交中...' : '提交' }}
                </button>
                <router-link to="/ich" class="cancel-btn">取消</router-link>
            </div>
        </form>
    </div>
</template>

<style scoped>
.ich-edit {
    max-width: 800px;
    margin: 0 auto;
    padding: 2rem;
}

.form-container {
    margin-top: 2rem;
}

.form-group {
    margin-bottom: 1.5rem;
}

.form-group label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: 500;
}

.form-group input,
.form-group textarea {
    width: 100%;
    padding: 0.5rem;
    border: 1px solid #ddd;
    border-radius: 4px;
}

.form-group textarea {
    resize: vertical;
}

.form-actions {
    display: flex;
    gap: 1rem;
    margin-top: 2rem;
}

.form-actions button {
    padding: 0.5rem 1.5rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    background: none;
    cursor: pointer;
}

.cancel-btn {
    padding: 0.5rem 1.5rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    text-decoration: none;
    color: #333;
}
</style>