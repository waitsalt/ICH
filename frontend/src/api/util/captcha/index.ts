import type { AppResponse } from "@/model";
import type { CaptchaImageResponse } from "@/model/util";
import { axiosBase } from "@/util/axios";

/**
 * 获取邮箱验证码
 * @param userEmail 用户邮箱
 * @returns 空响应
 */
async function getEmailCaptcha(userEmail: string): Promise<AppResponse<void>> {
    return await axiosBase.get(`/api/util/captcha_email/${userEmail}`);
}

/**
 * 获取图片验证码
 * @returns 图片验证码响应
 */
async function getImageCaptcha(): Promise<AppResponse<CaptchaImageResponse>> {
    return await axiosBase.get('/api/util/captcha_image');
}

export {
    getEmailCaptcha,
    getImageCaptcha,
}