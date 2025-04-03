interface Post {
    post_id: number;                       // 文章ID（主键）
    user_id: number;                       // 关联的用户ID（外键）
    ich_id: number;                        // 关联的非遗项目ID（外键）
    post_title: string;                    // 文章标题
    post_content: string;                  // 文章内容（富文本或Markdown格式）
    post_media_urls?: string[];            // 文章图片/视频URL
    post_type: number;                     // 文章类型（1=传承文章 2=研究记录 3=体验分享 4=其他）
    post_status: number;                   // 文章状态（0=待审核 1=已发布 2=已下架 3=违规删除）
    post_likes: number;                    // 点赞数
    post_favorites: number;                // 收藏数
    post_create_time: Date;                // 创建时间
    post_update_time: Date;                // 更新时间
}

interface PostCreatePayload {
    ich_id: number;
    post_type: number;
    post_title: string;
    post_content: string;
    post_media_urls?: string[];
}

interface PostSearchParam {
    keyword: string;
    page: number;
    size: number;
}

interface PostUpdatePayload {
    post_title: string,
    post_content: string,
    post_media_urls: string[] | null,
    post_type: number,
}

export type {
    Post,
    PostCreatePayload,
    PostSearchParam,
    PostUpdatePayload
};