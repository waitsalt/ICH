import type { AppResponse } from "@/model";
import type {
  UserAuth,
  UserPublic,
  UserSearchParam,
  UserSigninPayload,
  UserSignupPayload,
} from "@/model/user";
import { useUserStore } from "@/store/user";
import { axiosAuth, axiosAuthRefresh, axiosBase } from "@/util/axios";

/**
 * 用户登录
 * @param payload 登录参数
 * @returns 用户认证信息
 */
async function userSignin(
  payload: UserSigninPayload,
): Promise<AppResponse<UserAuth>> {
  return await axiosBase.post("/api/user/signin", payload);
}

/**
 * 用户登出
 * @returns 空响应
 */
async function userSignout(): Promise<AppResponse<void>> {
  return await axiosAuth.get("/api/user/signout");
}

/**
 * 用户注册
 * @param payload 注册参数
 * @returns 空响应
 */
async function userSignup(
  payload: UserSignupPayload,
): Promise<AppResponse<void>> {
  return await axiosBase.post("/api/user/signup", payload);
}

/**
 * 获取用户公开信息
 * @param userId 用户ID
 * @returns 用户公开信息
 */
async function getUserPublic(userId: number): Promise<AppResponse<UserPublic>> {
  return await axiosBase.get(`/api/user/${userId}`);
}

/**
 * 删除用户
 * @param userId 用户ID
 * @returns 空响应
 */
async function deleteUser(userId: number): Promise<AppResponse<void>> {
  return await axiosAuth.delete(`/api/user/${userId}`);
}

/**
 * 搜索用户
 * @param params 搜索参数
 * @returns 用户公开信息列表
 */
async function searchUser(
  params: UserSearchParam,
): Promise<AppResponse<UserPublic[]>> {
  return await axiosAuth.post("/api/user/search", params);
}

/**
 * 刷新访问令牌
 * @returns 新的访问令牌
 */
async function refreshAccessToken(): Promise<AppResponse<string>> {
  const userStore = useUserStore();
  const refreshToken = userStore.user_auth.refresh_token;
  if (!refreshToken) {
    throw new Error("请先登录");
  }
  const response: AppResponse<string> = await axiosAuthRefresh.post(
    "/api/user/refresh",
    { refresh_token: refreshToken },
  );
  userStore.user_auth.access_token = response.data;
  return response;
}

// async function getUserPublic(user_id: number): Promise<AppResponse<UserPublic>> {
//     return await axiosBase.get(`/api/post/${user_id}`);
// }

export {
  userSignin,
  userSignup,
  userSignout,
  getUserPublic,
  deleteUser,
  searchUser,
  refreshAccessToken,
};
