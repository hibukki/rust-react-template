import type { Profile } from "../types/bindings";

const API_BASE = "/api";

// Convert bigint fields from JSON (which returns numbers) to actual bigints
function parseProfile(data: unknown): Profile {
  const raw = data as { id: number; user_id: number; display_name: string; bio: string | null; updated_at: string };
  return {
    id: BigInt(raw.id),
    user_id: BigInt(raw.user_id),
    display_name: raw.display_name,
    bio: raw.bio,
    updated_at: raw.updated_at,
  };
}

async function handleResponse<T>(response: Response, parser?: (data: unknown) => T): Promise<T> {
  if (!response.ok) {
    const error = await response.json().catch(() => ({ error: "Unknown error" }));
    throw new Error(error.error || `HTTP ${response.status}`);
  }
  const data = await response.json();
  return parser ? parser(data) : data;
}

export interface AuthResponse {
  user_id: bigint;
  email: string;
  profile: Profile;
}

export interface RegisterRequest {
  email: string;
  password: string;
  display_name: string;
}

export interface LoginRequest {
  email: string;
  password: string;
}

export interface UpdateProfileRequest {
  display_name?: string;
  bio?: string;
}

export const api = {
  auth: {
    async register(req: RegisterRequest): Promise<AuthResponse> {
      const response = await fetch(`${API_BASE}/auth/register`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        credentials: "include",
        body: JSON.stringify(req),
      });
      return handleResponse(response, (data) => {
        const raw = data as { user_id: number; email: string; profile: unknown };
        return {
          user_id: BigInt(raw.user_id),
          email: raw.email,
          profile: parseProfile(raw.profile),
        };
      });
    },

    async login(req: LoginRequest): Promise<AuthResponse> {
      const response = await fetch(`${API_BASE}/auth/login`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        credentials: "include",
        body: JSON.stringify(req),
      });
      return handleResponse(response, (data) => {
        const raw = data as { user_id: number; email: string; profile: unknown };
        return {
          user_id: BigInt(raw.user_id),
          email: raw.email,
          profile: parseProfile(raw.profile),
        };
      });
    },

    async logout(): Promise<void> {
      const response = await fetch(`${API_BASE}/auth/logout`, {
        method: "POST",
        credentials: "include",
      });
      if (!response.ok) {
        const error = await response.json().catch(() => ({ error: "Unknown error" }));
        throw new Error(error.error || `HTTP ${response.status}`);
      }
    },
  },

  profiles: {
    async list(): Promise<Profile[]> {
      const response = await fetch(`${API_BASE}/profiles`, {
        credentials: "include",
      });
      return handleResponse(response, (data) => (data as unknown[]).map(parseProfile));
    },

    async get(id: bigint): Promise<Profile> {
      const response = await fetch(`${API_BASE}/profiles/${id}`, {
        credentials: "include",
      });
      return handleResponse(response, parseProfile);
    },

    async update(id: bigint, req: UpdateProfileRequest): Promise<Profile> {
      const response = await fetch(`${API_BASE}/profiles/${id}`, {
        method: "PATCH",
        headers: { "Content-Type": "application/json" },
        credentials: "include",
        body: JSON.stringify(req),
      });
      return handleResponse(response, parseProfile);
    },
  },
};
