import { refresh_access_token } from "@/api/user/refresh_access_token";
import { useUserStore } from "@/store/user";
import axios from "axios";

// 普通资源
const axiosBase = axios.create({
  baseURL: "http://127.0.0.1:8000",
  timeout: 30000,
});

axiosBase.interceptors.request.use((config) => {
  return config;
});

axiosBase.interceptors.response.use(
  (response) => {
    return response.data;
  },
  (error) => {
    return Promise.reject(error);
  },
);

// 认证资源
const axiosAuth = axios.create({
  baseURL: "http://127.0.0.1:8000",
  timeout: 30000,
});

axiosAuth.interceptors.request.use(async (config) => {
  const user_store = useUserStore();
  const access_token = user_store.user_auth.access_token;
  if (access_token !== null) {
    config.headers.Authorization = `Bearer ${access_token}`;
  } else {
    const access_token = await refresh_access_token();
    user_store.user_auth.access_token = access_token;
    config.headers.Authorization = `Bearer ${access_token}`;
  }
  return config;
});

axiosAuth.interceptors.response.use(
  (response) => {
    return response.data;
  },
  (error) => {
    return Promise.reject(error);
  },
);

// 认证资源
const axiosAuthRefresh = axios.create({
  baseURL: "http://127.0.0.1:8000",
  timeout: 30000,
});

axiosAuthRefresh.interceptors.request.use(async (config) => {
  const user_store = useUserStore();
  const refresh_token = user_store.user_auth.refresh_token;
  if (refresh_token !== null) {
    config.headers.Authorization = `Bearer ${refresh_token}`;
  } else {
    alert("请登陆");
  }
  return config;
});

axiosAuthRefresh.interceptors.response.use(
  (response) => {
    return response.data;
  },
  (error) => {
    return Promise.reject(error);
  },
);

export { axiosAuth, axiosBase, axiosAuthRefresh };
