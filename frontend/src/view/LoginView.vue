<script lang="ts" setup>
import { ref } from "vue";
import { useRouter } from "vue-router";
import { useUserStore } from "@/store/user";
import { userSignin } from "@/api/user";
import { getImageCaptcha } from "@/api/util/captcha";
import type { UserClaim } from "@/model/user";
import { jwtDecode } from "jwt-decode";

const router = useRouter();
const userStore = useUserStore();

const form = ref({
    username: "",
    password: "",
    captcha_key: "",
    captcha_value: "",
});

const captchaImage = ref("");
const errorMessage = ref("");
const loading = ref(false);

// 获取图片验证码
const loadCaptcha = async () => {
    const response = await getImageCaptcha();
    captchaImage.value = response.data.captcha_image;
    form.value.captcha_key = response.data.captcha_image_key;
};

const handleSubmit = async () => {
    if (loading.value) return;

    loading.value = true;
    errorMessage.value = "";

    try {
        const response = await userSignin({
            user_name: form.value.username,
            user_password: form.value.password,
            captcha_image_key: form.value.captcha_key,
            captcha_image_value: form.value.captcha_value,
        });

        userStore.user_auth = response.data;
        const user_claim: UserClaim = jwtDecode(
            userStore.user_auth.access_token,
        );
        userStore.user_public = user_claim.data;
        console.log(user_claim);
        router.push(
            router.currentRoute.value.query.redirect?.toString() || "/",
        );
    } catch (error: any) {
        errorMessage.value = error.response?.data?.message || "登录失败";
        loadCaptcha(); // 刷新验证码
    } finally {
        loading.value = false;
    }
};

// 初始化验证码
loadCaptcha();
</script>

<template>
    <div class="login-container">
        <h1>用户登录</h1>

        <form @submit.prevent="handleSubmit" class="login-form">
            <div class="form-group">
                <label>用户名</label>
                <input v-model="form.username" type="text" required />
            </div>

            <div class="form-group">
                <label>密码</label>
                <input v-model="form.password" type="password" required />
            </div>

            <div class="form-group">
                <label>验证码</label>
                <div class="captcha-group">
                    <input v-model="form.captcha_value" type="text" required />
                    <img
                        :src="captchaImage"
                        alt="验证码"
                        class="captcha-image"
                        @click="loadCaptcha"
                    />
                </div>
            </div>

            <div v-if="errorMessage" class="error-message">
                {{ errorMessage }}
            </div>

            <button type="submit" :disabled="loading">
                {{ loading ? "登录中..." : "登录" }}
            </button>

            <div class="register-link">
                没有账号？<router-link to="/register">立即注册</router-link>
            </div>
        </form>
    </div>
</template>

<style scoped>
.login-container {
    max-width: 400px;
    margin: 0 auto;
    padding: 2rem;
}

h1 {
    text-align: center;
    margin-bottom: 2rem;
}

.login-form {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
}

.form-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
}

.form-group label {
    font-weight: 500;
}

.form-group input {
    padding: 0.75rem;
    border: 1px solid #ddd;
    border-radius: 4px;
}

.captcha-group {
    display: flex;
    gap: 0.5rem;
}

.captcha-group input {
    flex: 1;
}

.captcha-image {
    height: 46px;
    border: 1px solid #eee;
    cursor: pointer;
}

.error-message {
    color: #f44336;
    text-align: center;
}

button {
    padding: 0.75rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    background: none;
    cursor: pointer;
    margin-top: 1rem;
}

.register-link {
    text-align: center;
    margin-top: 1rem;
    color: #666;
}

.register-link a {
    color: #333;
    text-decoration: none;
}

.register-link a:hover {
    text-decoration: underline;
}
</style>
