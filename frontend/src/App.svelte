<script lang="ts">
    import Auth from "./lib/Auth.svelte";
    import Users from "./lib/Users.svelte";
    import { clearStoredSession, getStoredUser, type User } from "./lib/api";

    let authenticatedUser = $state<User | null>(getStoredUser());

    function storeAuthenticatedUser(user: User) {
        authenticatedUser = user;
    }

    function signOut() {
        clearStoredSession();
        authenticatedUser = null;
    }
</script>

{#if !authenticatedUser}
    <Auth onLogin={storeAuthenticatedUser} />
{:else}
    <div class="navbar bg-base-100 shadow-sm">
        <div class="flex-1">
            <span class="text-xl font-semibold">CS 348 Portal</span>
        </div>
        <div class="flex-none gap-3">
            <div class="badge {authenticatedUser.role === 'admin' ? 'badge-error' : 'badge-primary'}">
                {authenticatedUser.role}
            </div>
            <span>{authenticatedUser.username}</span>
            <button class="btn btn-outline btn-sm" onclick={signOut}>Logout</button>
        </div>
    </div>

    <main class="p-4">
        <Users />
    </main>
{/if}
