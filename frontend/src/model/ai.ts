interface Message {
  role: string;
  content: string;
}

interface AiChat {
  prompt: string;
  history: Message[];
}

export type { Message, AiChat };
