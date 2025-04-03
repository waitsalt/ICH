interface Ich {
    ich_id: number;
    ich_code: string;
    ich_name: string;
    ich_publish: string;
    ich_category: string;
    ich_location: string;
    ich_apply_location: string;
    ich_protect_unit: string;
    ich_content: string;
    ich_create: Date;
    ich_uploader: number;
}

interface IchCreatePayload {
    ich_code: string;
    ich_name: string;
    ich_publish: string;
    ich_category: string;
    ich_location: string;
    ich_apply_location: string;
    ich_protect_unit: string;
    ich_content: string;
}

interface IchQueryParam {
    keyword: string;
    page: number;
    size: number;
}

interface IchUpdatePayload {
    ich_code: string;
    ich_name: string;
    ich_publish: string;
    ich_category: string;
    ich_location: string;
    ich_apply_location: string;
    ich_protect_unit: string;
    ich_content: string;
}

// 统一导出
export type {
    Ich,
    IchCreatePayload,
    IchQueryParam,
    IchUpdatePayload
};