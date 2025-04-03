import type { AppResponse } from "@/model";
import { useUserStore } from "@/store/user"
import { axiosAuthRefresh } from "@/util/axios";

async function refresh_access_token() {
    const user_store = useUserStore();
    const refresh_token = user_store.user_auth.refresh_token;
    if (refresh_token === '') {
        alert("请先登陆")
    }
    const response: AppResponse<string> = await axiosAuthRefresh.get('/api/user/refresh')
    user_store.user_auth.refresh_token = response.data;
    alert("登陆成功")
    return response.data;
}

export {
    refresh_access_token,
}