<script lang="ts" setup>
import { ref, onMounted } from 'vue'
import { useUserStore } from '@/store/user'
import type { UserPublic } from '@/model/user'
import { deleteUser, getUserPublic } from '@/api/user'

const userStore = useUserStore()
const userInfo = ref<UserPublic | null>(null)
const editMode = ref(false)
const tempUser = ref({
    user_name: '',
    user_desc: '',
    user_email: ''
})

onMounted(async () => {
    const response = await getUserPublic(userStore.user_public.user_id)
    userInfo.value = response.data
    tempUser.value = { ...response.data }
})

const handleSave = async () => {
    // await updateUser(userStore.user_id, tempUser.value)
    // userInfo.value = { ...tempUser.value }
    // editMode.value = false
}

const handleDelete = async () => {
    if (confirm('确定要删除账号吗？此操作不可恢复')) {
        await deleteUser(userStore.user_public.user_id)
        userStore.user_auth = {
            "access_token": '',
            "refresh_token": ''
        }
        // 跳转到登录页
    }
}
</script>

<template>
    <div class="profile-container">
        <h1>个人中心</h1>

        <div v-if="userInfo" class="profile-content">
            <div class="avatar-section">
                <img :src="userInfo.user_avatar_url" alt="头像" class="avatar">
            </div>

            <div class="info-section">
                <div class="info-item" v-if="!editMode">
                    <label>用户名</label>
                    <div>{{ userInfo.user_name }}</div>
                </div>
                <div class="info-item" v-else>
                    <label>用户名</label>
                    <input v-model="tempUser.user_name" type="text">
                </div>

                <div class="info-item" v-if="!editMode">
                    <label>简介</label>
                    <div>{{ userInfo.user_desc || '暂无简介' }}</div>
                </div>
                <div class="info-item" v-else>
                    <label>简介</label>
                    <textarea v-model="tempUser.user_desc"></textarea>
                </div>

                <div class="info-item">
                    <label>邮箱</label>
                    <div>{{ userInfo.user_email }}</div>
                </div>

                <div class="info-item">
                    <label>注册时间</label>
                    <div>{{ new Date(userInfo.user_create_time).toLocaleString() }}</div>
                </div>

                <div class="actions">
                    <button v-if="!editMode" @click="editMode = true">编辑资料</button>
                    <template v-else>
                        <button @click="handleSave">保存</button>
                        <button @click="editMode = false">取消</button>
                    </template>
                    <button class="delete-btn" @click="handleDelete">删除账号</button>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
.profile-container {
    max-width: 800px;
    margin: 0 auto;
    padding: 2rem;
}

.profile-content {
    display: flex;
    gap: 2rem;
    margin-top: 2rem;
}

.avatar-section {
    flex: 0 0 150px;
}

.avatar {
    width: 150px;
    height: 150px;
    border-radius: 50%;
    object-fit: cover;
    border: 1px solid #e0e0e0;
}

.info-section {
    flex: 1;
}

.info-item {
    margin-bottom: 1.5rem;
}

.info-item label {
    display: block;
    font-weight: 500;
    margin-bottom: 0.5rem;
    color: #333;
}

.info-item div {
    padding: 0.5rem 0;
    border-bottom: 1px solid #eee;
}

.info-item input,
.info-item textarea {
    width: 100%;
    padding: 0.5rem;
    border: 1px solid #ddd;
    border-radius: 4px;
}

.info-item textarea {
    min-height: 100px;
    resize: vertical;
}

.actions {
    display: flex;
    gap: 1rem;
    margin-top: 2rem;
}

button {
    padding: 0.5rem 1rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    background: none;
    cursor: pointer;
    transition: all 0.2s;
}

button:hover {
    background: #f5f5f5;
}

.delete-btn {
    margin-left: auto;
    color: #f44336;
    border-color: #f44336;
}

.delete-btn:hover {
    background: #ffebee;
}
</style>