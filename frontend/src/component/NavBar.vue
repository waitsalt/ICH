<script lang="ts" setup>
import { ref, computed } from "vue";
import { useUserStore } from "@/store/user";
import { useRouter } from "vue-router";

const userStore = useUserStore();
const router = useRouter();
const showDropdown = ref(false);
let closeTimer: number | null = null;

const isAuthenticated = computed(() => userStore.isAuthenticated());

const handleLogout = () => {
    userStore.user_auth = {
        access_token: "",
        refresh_token: "",
    };
    router.push("/");
    showDropdown.value = false;
};

const navigateTo = (path: string) => {
    router.push(path);
    showDropdown.value = false;
};

const handleMouseEnter = () => {
    if (closeTimer) {
        clearTimeout(closeTimer);
        closeTimer = null;
    }
    showDropdown.value = true;
};

const handleMouseLeave = () => {
    closeTimer = setTimeout(() => {
        showDropdown.value = false;
    }, 200); // 200ms延迟关闭
};
</script>

<template>
    <nav class="navbar">
        <div class="nav-container">
            <!-- 左侧导航 -->
            <div class="nav-left">
                <router-link to="/" class="logo">
                    <span>非遗文化</span>
                </router-link>
                <router-link to="/recommend" class="nav-item">推荐</router-link>
                <router-link to="/ich" class="nav-item">非遗</router-link>
                <router-link to="/posts" class="nav-item">文章</router-link>
            </div>

            <!-- 右侧导航 -->
            <div class="nav-right">
                <router-link to="/ai" class="nav-item">非遗助手</router-link>
                <div v-if="!isAuthenticated" class="auth-buttons">
                    <router-link to="/login" class="auth-button"
                        >登录</router-link
                    >
                    <router-link to="/register" class="auth-button"
                        >注册</router-link
                    >
                </div>

                <div
                    v-else
                    class="user-menu"
                    @mouseenter="handleMouseEnter"
                    @mouseleave="handleMouseLeave"
                >
                    <div class="avatar-container">
                        <img
                            :src="
                                userStore.user_public?.user_avatar_url ||
                                '/default-avatar.png'
                            "
                            alt="用户头像"
                            class="user-avatar"
                        />

                        <transition name="fade">
                            <div
                                v-if="showDropdown"
                                class="dropdown-menu"
                                @mouseenter="handleMouseEnter"
                                @mouseleave="handleMouseLeave"
                            >
                                <div class="dropdown-section">
                                    <button
                                        @click="navigateTo('/follow')"
                                        class="dropdown-item"
                                    >
                                        关注
                                    </button>
                                    <button
                                        @click="navigateTo('/activity')"
                                        class="dropdown-item"
                                    >
                                        动态
                                    </button>
                                </div>
                                <div class="dropdown-section">
                                    <button
                                        @click="navigateTo('/profile/posts')"
                                        class="dropdown-item"
                                    >
                                        我的发布
                                    </button>
                                </div>
                                <div class="dropdown-section">
                                    <button
                                        @click="navigateTo('/profile')"
                                        class="dropdown-item"
                                    >
                                        账号设置
                                    </button>
                                </div>
                                <div class="dropdown-section">
                                    <button
                                        @click="handleLogout"
                                        class="dropdown-item logout"
                                    >
                                        退出登录
                                    </button>
                                </div>
                            </div>
                        </transition>
                    </div>
                </div>
            </div>
        </div>
    </nav>
</template>

<style scoped>
.navbar {
    position: sticky;
    top: 0;
    z-index: 100;
    background-color: white;
    border-bottom: 1px solid #eee;
    padding: 0.5rem 0;
}

.nav-container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 0 1rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.nav-left {
    display: flex;
    align-items: center;
    gap: 2rem;
}

.logo {
    font-weight: bold;
    font-size: 1.2rem;
    text-decoration: none;
    color: #333;
}

.nav-item {
    text-decoration: none;
    color: #333;
    padding: 0.5rem 0;
    border-bottom: 2px solid transparent;
    transition: all 0.2s;
}

.nav-item:hover {
    border-bottom-color: #333;
}

.nav-right {
    display: flex;
    align-items: center;
    gap: 1.5rem;
}

.search-container {
    display: flex;
    align-items: center;
    border: 1px solid #ddd;
    border-radius: 4px;
    overflow: hidden;
}

.search-type {
    padding: 0.5rem;
    border: none;
    outline: none;
    background: none;
    border-right: 1px solid #eee;
    cursor: pointer;
}

.search-input {
    padding: 0.5rem;
    border: none;
    outline: none;
    min-width: 200px;
}

.auth-buttons {
    display: flex;
    gap: 0.75rem;
}

.auth-button {
    padding: 0.5rem 1rem;
    text-decoration: none;
    color: #333;
    border: 1px solid #ddd;
    border-radius: 4px;
}

.user-menu {
    position: relative;
}

.avatar-container {
    cursor: pointer;
}

.user-avatar {
    width: 36px;
    height: 36px;
    border-radius: 50%;
    object-fit: cover;
    border: 1px solid #eee;
}

.dropdown-menu {
    position: absolute;
    right: 0;
    top: 100%;
    margin-top: 0.5rem;
    min-width: 180px;
    background: white;
    border: 1px solid #eee;
    border-radius: 4px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    z-index: 101;
    padding: 0.5rem 0;
}

.dropdown-section {
    border-bottom: 1px solid #eee;
    padding: 0.25rem 0;
}

.dropdown-section:last-child {
    border-bottom: none;
}

.dropdown-item {
    display: block;
    width: 100%;
    padding: 0.5rem 1rem;
    text-align: left;
    background: none;
    border: none;
    cursor: pointer;
    color: #333;
}

.dropdown-item:hover {
    background-color: #f5f5f5;
}

.dropdown-item.logout {
    color: #f44336;
}

.fade-enter-active,
.fade-leave-active {
    transition:
        opacity 0.2s,
        transform 0.2s;
}

.fade-enter-from,
.fade-leave-to {
    opacity: 0;
    transform: translateY(-10px);
}

/* 响应式调整 */
@media (max-width: 768px) {
    .nav-container {
        flex-direction: column;
        gap: 1rem;
        padding: 1rem;
    }

    .nav-left {
        width: 100%;
        justify-content: space-between;
    }

    .nav-right {
        width: 100%;
        justify-content: space-between;
    }

    .search-container {
        flex: 1;
        margin-right: 1rem;
    }
}
</style>
