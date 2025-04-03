<script lang="ts" setup>
import { ref } from "vue";
import type { AiChat, Message } from "@/model/ai";
import { chatStream } from "@/api/ai";

const messages = ref<Message[]>([
    {
        role: "assistant",
        content: "您好！我是非遗文化助手，请问有什么可以帮您的吗？",
    },
]);

const userInput = ref("");
const isLoading = ref(false);
const isStreaming = ref(false);

const handleSend = async () => {
    if (!userInput.value.trim() || isLoading.value) return;

    const userMessage: Message = {
        role: "user",
        content: userInput.value,
    };

    messages.value.push(userMessage);
    const currentInput = userInput.value;
    userInput.value = "";
    isLoading.value = true;
    isStreaming.value = true;

    // 添加临时助手消息占位
    messages.value.push({
        role: "assistant",
        content: "",
    });
    const assistantIndex = messages.value.length - 1;

    try {
        await chatStream(
            {
                prompt: currentInput,
                history: messages.value.slice(0, -1),
            },
            (data, event) => {
                if (event === "error") {
                    messages.value[assistantIndex].content = `错误: ${data}`;
                    isStreaming.value = false;
                } else {
                    messages.value[assistantIndex].content += data;
                }
            },
        );
    } catch (error) {
        // messages.value[assistantIndex].content = `请求失败: ${error.message}`;
        messages.value[assistantIndex].content = `请求失败`;
    } finally {
        isLoading.value = false;
        isStreaming.value = false;
    }
};

const handleKeyPress = (e: KeyboardEvent) => {
    if (e.key === "Enter" && !e.shiftKey) {
        e.preventDefault();
        handleSend();
    }
};
</script>

<template>
    <div class="ai-chat-container">
        <h1>非遗AI助手</h1>

        <div class="chat-window">
            <div
                v-for="(message, index) in messages"
                :key="index"
                :class="['message', message.role]"
            >
                <div class="message-content">
                    {{ message.content }}
                </div>
            </div>

            <div v-if="isLoading" class="message assistant">
                <div class="message-content">
                    {{ messages[messages.length - 1].content || "思考中..." }}
                </div>
            </div>
        </div>

        <div class="input-area">
            <textarea
                v-model="userInput"
                placeholder="输入您的问题..."
                @keydown="handleKeyPress"
                :disabled="isLoading"
            ></textarea>
            <button
                @click="handleSend"
                :disabled="!userInput.trim() || isLoading"
            >
                {{
                    isLoading
                        ? isStreaming
                            ? "生成中..."
                            : "发送中..."
                        : "发送"
                }}
            </button>
        </div>
    </div>
</template>

<style scoped>
/* 保持之前的样式不变 */
.ai-chat-container {
    max-width: 800px;
    margin: 0 auto;
    padding: 2rem;
    display: flex;
    flex-direction: column;
    height: calc(100vh - 120px);
}

h1 {
    text-align: center;
    margin-bottom: 2rem;
    font-size: 1.8rem;
}

.chat-window {
    flex: 1;
    overflow-y: auto;
    padding: 1rem;
    border: 1px solid #eee;
    border-radius: 4px;
    margin-bottom: 1rem;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
}

.message {
    max-width: 80%;
    padding: 0.75rem 1rem;
    border-radius: 4px;
    line-height: 1.5;
}

.message.user {
    align-self: flex-end;
    background-color: #f5f5f5;
    border: 1px solid #eee;
}

.message.assistant {
    align-self: flex-start;
    background-color: white;
    border: 1px solid #eee;
}

.message-content {
    white-space: pre-wrap;
}

.input-area {
    display: flex;
    gap: 0.5rem;
}

.input-area textarea {
    flex: 1;
    padding: 0.75rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    min-height: 60px;
    resize: none;
    font-family: inherit;
}

.input-area button {
    padding: 0 1.5rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    background: none;
    cursor: pointer;
    align-self: flex-end;
    height: 42px;
}

.input-area button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
}

.input-area button:hover:not(:disabled) {
    background-color: #f5f5f5;
}

/* 响应式调整 */
@media (max-width: 768px) {
    .ai-chat-container {
        padding: 1rem;
        height: calc(100vh - 100px);
    }

    .message {
        max-width: 90%;
    }
}
</style>
