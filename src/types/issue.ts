// Issue 数据结构
export interface Issue {
  id: string;
  number: number;
  title: string;
  body: string;
  state: 'open' | 'closed';
  locked: boolean;
  user: {
    id: string;
    login: string;
    avatar_url: string;
    html_url: string;
  };
  assignee: {
    id: string;
    login: string;
    avatar_url: string;
    html_url: string;
  } | null;
  labels: Array<{
    id: string;
    name: string;
    color: string;
    description: string;
  }>;
  milestone: {
    id: string;
    title: string;
    description: string;
    state: 'open' | 'closed';
    due_on: string | null;
  } | null;
  created_at: string;
  updated_at: string;
  closed_at: string | null;
  repository: {
    name: string;
    full_name: string;
    html_url: string;
  };
  comments_count: number;
}

// 评论数据结构
export interface Comment {
  id: string;
  body: string;
  user: {
    id: string;
    login: string;
    avatar_url: string;
    html_url: string;
  };
  created_at: string;
  updated_at: string;
}