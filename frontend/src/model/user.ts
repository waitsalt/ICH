interface User {
    user_id: number;
    user_name: string;
    user_desc: string;
    user_password: string;
    user_email: string;
    user_avatar_url: string;  // 头像 url
    user_level: number;       // 0
    user_status: number;      // 0. 正常 1. 被封禁 2. 删除
    user_identity: number;    // 0. 超级管理 1. 管理员 2. 用户 3. 继承人
    user_create_time: string;
    user_update_time: string;
}

interface UserPublic {
    user_id: number;
    user_name: string;
    user_desc: string;
    user_email: string;
    user_avatar_url: string;
    user_level: number;
    user_status: number;
    user_identity: number;
    user_create_time: string;
    user_update_time: string;
}

interface UserSigninPayload {
    user_name: string;
    user_password: string;
    captcha_image_key: string;
    captcha_image_value: string;
}

interface UserSignupPayload {
    user_name: string;
    user_password: string;
    user_email: string;
    user_avatar_url: string;
    captcha_email: string;
    captcha_image_key: string;
    captcha_image_value: string;
}

interface UserSearchParam {
    keyword: string;
    page: number;
    size: number;
}

interface UserClaim {
    iat: number;  // 开始毫秒
    exp: number;  // 到期毫秒
    data: UserPublic;
}

interface UserRefreshClaim {
    iat: number;
    exp: number;
    data: string;
}

interface UserAuth {
    access_token: string;
    refresh_token: string;
}

export type {
    User,
    UserPublic,
    UserSigninPayload,
    UserSignupPayload,
    UserSearchParam,
    UserClaim,
    UserRefreshClaim,
    UserAuth,
};