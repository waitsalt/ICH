import type { AppResponse } from "@/model";
import type { AiChat } from "@/model/ai";

export async function chatStream(
  ai_chat: AiChat,
  onMessage: (data: string, event?: string) => void,
): Promise<void> {
  const eventSource = new EventSource(
    `/api/ai/chat?prompt=${encodeURIComponent(ai_chat.prompt)}&history=${encodeURIComponent(JSON.stringify(ai_chat.history))}`,
  );

  return new Promise((resolve, reject) => {
    eventSource.onmessage = (event) => {
      if (event.type === "end") {
        eventSource.close();
        resolve();
      } else if (event.type === "error") {
        eventSource.close();
        reject(new Error(event.data));
      } else {
        onMessage(event.data);
      }
    };

    eventSource.onerror = () => {
      eventSource.close();
      reject(new Error("EventSource failed"));
    };
  });
}
