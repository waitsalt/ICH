import type { AppResponse } from "@/model";
import type { Post, PostCreatePayload, PostSearchParam, PostUpdatePayload } from "@/model/post";
import { axiosAuth, axiosBase } from "@/util/axios";


/**
 * 创建帖子
 * @param payload 帖子创建参数
 * @returns 创建的帖子ID
 */
async function createPost(payload: PostCreatePayload): Promise<AppResponse<number>> {
    return await axiosAuth.post('/api/post/', payload);
}

/**
 * 删除帖子
 * @param postId 帖子ID
 * @returns 删除的帖子ID
 */
async function deletePost(postId: number): Promise<AppResponse<number>> {
    return await axiosAuth.delete(`/api/post/${postId}`);
}

/**
 * 更新帖子
 * @param postId 帖子ID
 * @param payload 帖子更新参数
 * @returns 空响应
 */
async function updatePost(postId: number, payload: PostUpdatePayload): Promise<AppResponse<void>> {
    return await axiosAuth.put(`/api/post/${postId}`, payload);
}

/**
 * 搜索帖子
 * @param params 搜索参数
 * @returns 帖子列表
 */
async function searchPost(params: PostSearchParam): Promise<AppResponse<Post[]>> {
    return await axiosBase.get('/api/post/search', { params });
}


async function getPost(post_id: number): Promise<AppResponse<Post>> {
    return await axiosBase.get(`/api/post/${post_id}`);
}

export {
    createPost,
    deletePost,
    updatePost,
    searchPost,
    getPost,
}