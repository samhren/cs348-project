const backendBaseUrl = import.meta.env.VITE_BACKEND_URL ?? "http://localhost:3000";

export interface User {
    id: string;
    username: string;
    email: string;
    role: "admin" | "user";
    created_at: string;
}

export interface AuthResponse {
    token: string;
    user: User;
}

export interface UserFilters {
    role?: string;
    username?: string;
}

export interface NewUserDetails {
    username: string;
    email: string;
    password: string;
    role: string;
}

export interface UpdatedUserDetails {
    username: string;
    email: string;
    role: string;
}

export function getStoredToken(): string | null {
    return localStorage.getItem("token");
}

export function getStoredUser(): User | null {
    const storedUser = localStorage.getItem("user");
    return storedUser ? JSON.parse(storedUser) : null;
}

export function storeSession(authResponse: AuthResponse) {
    localStorage.setItem("token", authResponse.token);
    localStorage.setItem("user", JSON.stringify(authResponse.user));
}

export function clearStoredSession() {
    localStorage.removeItem("token");
    localStorage.removeItem("user");
}

function createAuthHeaders(): Record<string, string> {
    const storedToken = getStoredToken();
    return storedToken ? { Authorization: `Bearer ${storedToken}` } : {};
}

async function parseResponse<T>(response: Response, fallbackMessage: string): Promise<T> {
    const responseBody = await response.text();
    const parsedBody = responseBody ? JSON.parse(responseBody) : null;

    if (response.status === 401) {
        clearStoredSession();
        if (typeof window !== "undefined") {
            window.location.reload();
        }
    }

    if (!response.ok) {
        throw new Error(parsedBody?.error ?? fallbackMessage);
    }

    return parsedBody as T;
}

export async function signIn(username: string, password: string): Promise<AuthResponse> {
    const response = await fetch(`${backendBaseUrl}/auth/login`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ username, password }),
    });
    const authResponse = await parseResponse<AuthResponse>(response, "Invalid credentials");
    storeSession(authResponse);
    return authResponse;
}

export async function signUp(
    username: string,
    email: string,
    password: string,
): Promise<AuthResponse> {
    const response = await fetch(`${backendBaseUrl}/auth/register`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ username, email, password }),
    });
    const authResponse = await parseResponse<AuthResponse>(response, "Registration failed");
    storeSession(authResponse);
    return authResponse;
}

export async function fetchUsers(filters: UserFilters = {}): Promise<User[]> {
    const searchParams = new URLSearchParams();

    if (filters.role) {
        searchParams.set("role", filters.role);
    }

    if (filters.username) {
        searchParams.set("username", filters.username);
    }

    const response = await fetch(`${backendBaseUrl}/users?${searchParams}`, {
        headers: createAuthHeaders(),
    });

    return parseResponse<User[]>(response, "Failed to load users");
}

export async function createUserAccount(newUserDetails: NewUserDetails): Promise<User> {
    const response = await fetch(`${backendBaseUrl}/users`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
            ...createAuthHeaders(),
        },
        body: JSON.stringify(newUserDetails),
    });

    return parseResponse<User>(response, "Failed to create user");
}

export async function updateUserAccount(
    userId: string,
    updatedUserDetails: UpdatedUserDetails,
): Promise<User> {
    const response = await fetch(`${backendBaseUrl}/users/${userId}`, {
        method: "PUT",
        headers: {
            "Content-Type": "application/json",
            ...createAuthHeaders(),
        },
        body: JSON.stringify(updatedUserDetails),
    });

    return parseResponse<User>(response, "Failed to update user");
}

export async function deleteUserAccount(userId: string): Promise<void> {
    const response = await fetch(`${backendBaseUrl}/users/${userId}`, {
        method: "DELETE",
        headers: createAuthHeaders(),
    });

    await parseResponse<null>(response, "Failed to delete user");
}

export async function fetchAvailableRoles(): Promise<string[]> {
    const response = await fetch(`${backendBaseUrl}/users/roles`, {
        headers: createAuthHeaders(),
    });

    if (!response.ok) {
        return [];
    }

    return parseResponse<string[]>(response, "Failed to load roles");
}
