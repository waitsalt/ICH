<script lang="ts" setup>
import { ref } from "vue";
import { useRouter } from "vue-router";
import { userSignup } from "@/api/user";
import { getEmailCaptcha, getImageCaptcha } from "@/api/util/captcha";
import type { UserSignupPayload } from "@/model/user";
import type { AppResponse } from "@/model";

const router = useRouter();

const form = ref<UserSignupPayload>({
    user_name: "",
    user_password: "",
    user_email: "",
    user_avatar_url: "",
    captcha_email: "",
    captcha_image_key: "",
    captcha_image_value: "",
});

const captchaImage = ref("");
const errorMessage = ref("");
const loading = ref(false);
const emailSent = ref(false);
const emailCountdown = ref(0);

// 获取图片验证码
const loadImageCaptcha = async () => {
    const response = await getImageCaptcha();
    captchaImage.value = response.data.captcha_image;
    form.value.captcha_image_key = response.data.captcha_image_key;
};

// 发送邮箱验证码
const sendEmailCaptcha = async () => {
    errorMessage.value = "";
    if (!form.value.user_email) {
        errorMessage.value = "请先输入邮箱";
        return;
    }

    try {
        await getEmailCaptcha(form.value.user_email);
        emailSent.value = true;
        emailCountdown.value = 60;
        const timer = setInterval(() => {
            emailCountdown.value--;
            if (emailCountdown.value <= 0) clearInterval(timer);
        }, 1000);
    } catch (error) {
        errorMessage.value = "发送验证码失败";
    }
};

const handleSubmit = async () => {
    if (loading.value) return;

    loading.value = true;
    errorMessage.value = "";

    try {
        await userSignup(form.value);
        // router.push({ name: "login", query: { registered: "true" } });
    } catch (error: any) {
        errorMessage.value = error.response?.data?.message || "注册失败";
        loadImageCaptcha();
    } finally {
        loading.value = false;
    }
};

// 初始化验证码
loadImageCaptcha();
</script>

<template>
    <div class="register-container">
        <h1>用户注册</h1>

        <form @submit.prevent="handleSubmit" class="register-form">
            <div class="form-group">
                <label>用户名</label>
                <input v-model="form.user_name" type="text" required />
            </div>

            <div class="form-group">
                <label>密码</label>
                <input v-model="form.user_password" type="password" required />
            </div>

            <div class="form-group">
                <label>邮箱</label>
                <input v-model="form.user_email" type="email" required />
            </div>

            <div class="form-group">
                <label>头像URL</label>
                <input v-model="form.user_avatar_url" type="text" />
            </div>

            <div class="form-group">
                <label>邮箱验证码</label>
                <div class="email-captcha-group">
                    <input v-model="form.captcha_email" type="text" required />
                    <button
                        type="button"
                        @click="sendEmailCaptcha"
                        :disabled="emailCountdown > 0"
                    >
                        {{
                            emailCountdown > 0
                                ? `${emailCountdown}秒后重试`
                                : "获取验证码"
                        }}
                    </button>
                </div>
            </div>

            <div class="form-group">
                <label>图片验证码</label>
                <div class="captcha-group">
                    <input
                        v-model="form.captcha_image_value"
                        type="text"
                        required
                    />
                    <img
                        :src="captchaImage"
                        alt="验证码"
                        class="captcha-image"
                        @click="loadImageCaptcha"
                    />
                </div>
            </div>

            <div v-if="errorMessage" class="error-message">
                {{ errorMessage }}
            </div>

            <button type="submit" :disabled="loading">
                {{ loading ? "注册中..." : "注册" }}
            </button>

            <div class="login-link">
                已有账号？<router-link to="/login">立即登录</router-link>
            </div>
        </form>
    </div>
</template>

<style scoped>
.register-container {
    max-width: 400px;
    margin: 0 auto;
    padding: 2rem;
}

h1 {
    text-align: center;
    margin-bottom: 2rem;
}

.register-form {
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

.email-captcha-group,
.captcha-group {
    display: flex;
    gap: 0.5rem;
}

.email-captcha-group input,
.captcha-group input {
    flex: 1;
}

.email-captcha-group button {
    padding: 0 1rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    background: none;
    cursor: pointer;
    white-space: nowrap;
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

button[type="submit"] {
    padding: 0.75rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    background: none;
    cursor: pointer;
    margin-top: 1rem;
}

.login-link {
    text-align: center;
    margin-top: 1rem;
    color: #666;
}

.login-link a {
    color: #333;
    text-decoration: none;
}

.login-link a:hover {
    text-decoration: underline;
}
</style>
