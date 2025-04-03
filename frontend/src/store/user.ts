import { ref } from "vue";
import { defineStore } from "pinia";
import type { UserAuth, UserPublic } from "@/model/user";

const useUserStore = defineStore("user", () => {
  const user_auth = ref<UserAuth>({
    access_token: "",
    refresh_token: "",
  });
  const user_public = ref<UserPublic>({
    user_id: 0,
    user_name: "",
    user_desc: "",
    user_email: "",
    user_avatar_url: "",
    user_level: 0,
    user_status: 0,
    user_identity: 0,
    user_create_time: "",
    user_update_time: "string",
  });

  const isAuthenticated = () => {
    if (
      user_auth.value.access_token === "" &&
      user_auth.value.refresh_token === ""
    ) {
      return false;
    } else {
      return true;
    }
  };

  return {
    user_auth,
    user_public,
    isAuthenticated,
  };
});

export { useUserStore };
