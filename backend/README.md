# axum-server-template
A simple template for web-server via axum.

# API 接口规范

## 用户接口 (User)

1. 路由: `/api/user/signin` 请求方法: `POST` 参数: UserSigninPayload 返回值: UserAuth
2. 路由: `/api/user/signout` 请求方法: `GET` 参数: 无 返回值: ()
3. 路由: `/api/user/signup` 请求方法: `POST` 参数: UserSignupPayload 返回值: ()
4. 路由: `/api/user/{user_id}` 请求方法: `GET` 参数: user_id (路径参数) 返回值: UserPublic
5. 路由: `/api/user/{user_id}` 请求方法: `DELETE` 参数: user_id (路径参数) 返回值: ()
6. 路由: `/api/user/search` 请求方法: `POST` 参数: UserSearchParam 返回值: Vec<UserPublic>
7. 路由: `/api/user/refresh` 请求方法: `POST` 参数: refresh_token 返回值: String

## 帖子接口 (Post)

1. 路由: `/api/post/` 请求方法: `POST` 参数: PostCreatePayload 返回值: i64
2. 路由: `/api/post/{post_id}` 请求方法: `DELETE` 参数: post_id (路径参数) 返回值: i64
3. 路由: `/api/post/{post_id}` 请求方法: `PUT` 参数: post_id (路径参数), PostUpdatePayload 返回值: ()
4. 路由: `/api/post/search` 请求方法: `GET` 参数: PostSearchParam 返回值: Vec<Post>

## ICH接口 (非物质文化遗产)

1. 路由: `/api/ich/` 请求方法: `POST` 参数: IchCreatePayload 返回值: i64
2. 路由: `/api/ich/{ich_id}` 请求方法: `DELETE` 参数: ich_id (路径参数) 返回值: ()
3. 路由: `/api/ich/{ich_id}` 请求方法: `PUT` 参数: ich_id (路径参数), IchUpdatePayload 返回值: ()
4. 路由: `/api/ich/search` 请求方法: `GET` 参数: IchQueryParam 返回值: Vec<Ich>

## 工具接口 (Util)

1. 路由: `/api/util/captcha_email/{user_email}` 请求方法: `GET` 参数: user_email (路径参数) 返回值: ()
2. 路由: `/api/util/captcha_image` 请求方法: `GET` 参数: 无 返回值: CaptchaImageResponse
