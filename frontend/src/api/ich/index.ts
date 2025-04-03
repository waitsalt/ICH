import type { AppResponse } from "@/model";
import type {
  Ich,
  IchCreatePayload,
  IchQueryParam,
  IchUpdatePayload,
} from "@/model/ich";
import { axiosAuth, axiosBase } from "@/util/axios";

/**
 * 创建ICH项目
 * @param payload ICH创建参数
 * @returns 创建的ICH项目ID
 */
async function createIch(
  payload: IchCreatePayload,
): Promise<AppResponse<number>> {
  return await axiosAuth.post("/api/ich/", payload);
}

/**
 * 删除ICH项目
 * @param ichId ICH项目ID
 * @returns 空响应
 */
async function deleteIch(ichId: number): Promise<AppResponse<void>> {
  return await axiosAuth.delete(`/api/ich/${ichId}`);
}

/**
 * 更新ICH项目
 * @param ichId ICH项目ID
 * @param payload ICH更新参数
 * @returns 空响应
 */
async function updateIch(
  ichId: number,
  payload: IchUpdatePayload,
): Promise<AppResponse<void>> {
  return await axiosAuth.put(`/api/ich/${ichId}`, payload);
}

/**
 * 搜索ICH项目
 * @param params 搜索参数
 * @returns ICH项目列表
 */
async function searchIch(params: IchQueryParam): Promise<AppResponse<Ich[]>> {
  return await axiosBase.get("/api/ich/search", { params });
}

/**
 * 获取ICH项目
 * @param params 搜索参数
 * @returns ICH项目列表
 */
async function getIch(ich_id: number): Promise<AppResponse<Ich>> {
  return await axiosBase.get(`/api/ich/${ich_id}`);
}

async function recommendIch(ich_id: number): Promise<AppResponse<Ich>> {
  return await axiosBase.get(`/api/ich/recommend`);
}

export { createIch, deleteIch, updateIch, searchIch, getIch, recommendIch };
