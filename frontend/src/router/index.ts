import { createRouter, createWebHistory } from "vue-router";
import { useUserStore } from "@/store/user";

import HomeView from "@/view/HomeView.vue";
import MyProfile from "@/view/MyProfile.vue";
import PostList from "@/view/PostList.vue";
import IchManagement from "@/view/IchManagement.vue";
import IchEdit from "@/view/IchEdit.vue";
import NotFound from "@/view/NotFound.vue";
import RegisterView from "@/view/RegisterView.vue";
import LoginView from "@/view/LoginView.vue";
import IchDetail from "@/view/IchDetail.vue";
import Recommend from "@/view/Recommend.vue";
import AiView from "@/view/AiView.vue";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "home",
      component: HomeView,
      meta: { title: "首页" },
    },
    {
      path: "/ai",
      name: "ai",
      component: AiView,
      meta: { title: "非遗助手" },
    },
    {
      path: "/profile",
      name: "profile",
      component: MyProfile,
      meta: {
        title: "个人中心",
        requiresAuth: true,
      },
    },
    {
      path: "/posts",
      name: "posts",
      component: PostList,
      meta: {
        title: "文章列表",
        requiresAuth: true,
      },
    },
    {
      path: "/recommend",
      name: "recommend",
      component: Recommend,
      meta: {
        title: "推荐",
        requiresAuth: false,
      },
    },
    {
      path: "/ich",
      name: "ich-management",
      component: IchManagement,
      meta: {
        title: "非遗项目管理",
        requiresAuth: true,
      },
    },
    {
      path: "/ich/create",
      name: "ich-create",
      component: IchEdit,
      meta: {
        title: "创建非遗项目",
        requiresAuth: true,
      },
    },
    {
      path: "/ich/:id/edit",
      name: "ich-edit",
      component: IchEdit,
      props: true,
      meta: {
        title: "编辑非遗项目",
        requiresAuth: true,
      },
    },
    {
      path: "/ich/:id",
      name: "ich-detail",
      component: IchDetail,
      props: true,
      meta: {
        title: "非遗项目详情",
        requiresAuth: true,
      },
    },
    {
      path: "/login",
      name: "login",
      component: LoginView,
      meta: {
        title: "登录",
        requiresGuest: true,
      },
    },
    {
      path: "/register",
      name: "register",
      component: RegisterView,
      meta: {
        title: "注册",
        requiresGuest: true,
      },
    },
    {
      path: "/:pathMatch(.*)*",
      name: "not-found",
      component: NotFound,
    },
  ],
});

// 路由守卫 - 身份验证检查
router.beforeEach(async (to, from, next) => {
  const userStore = useUserStore();

  // 设置页面标题
  if (to.meta.title) {
    document.title = `${to.meta.title} | 非遗文化平台`;
  }

  // 检查是否需要认证
  if (to.meta.requiresAuth && !userStore.isAuthenticated) {
    next({ name: "login", query: { redirect: to.fullPath } });
    return;
  }

  // 检查是否要求未登录状态
  if (to.meta.requiresGuest && userStore.isAuthenticated()) {
    next({ name: "home" });
    return;
  }

  next();
});

export default router;
