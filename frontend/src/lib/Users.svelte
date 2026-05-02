<script lang="ts">
    import { onMount } from "svelte";
    import UserFormDialog from "./UserFormDialog.svelte";
    import {
        createUserAccount,
        deleteUserAccount,
        fetchAvailableRoles,
        fetchUsers,
        updateUserAccount,
        type NewUserDetails,
        type UpdatedUserDetails,
        type User,
    } from "./api";

    const selectableRoles = [
        { value: "user", label: "User" },
        { value: "admin", label: "Admin" },
    ];

    let users = $state<User[]>([]);
    let listErrorMessage = $state("");
    let userBeingEdited = $state<User | null>(null);
    let isCreateDialogOpen = $state(false);
    let isEditDialogOpen = $state(false);

    let selectedRoleFilter = $state("");
    let usernameFilter = $state("");
    let availableRoleFilters = $state<string[]>([]);

    onMount(async () => {
        availableRoleFilters = await fetchAvailableRoles();
        await refreshUsers();
    });

    async function refreshUsers() {
        listErrorMessage = "";

        try {
            users = await fetchUsers({
                role: selectedRoleFilter || undefined,
                username: usernameFilter || undefined,
            });
        } catch {
            listErrorMessage = "Failed to load users.";
        }
    }

    async function applyFilters(event: SubmitEvent) {
        event.preventDefault();
        await refreshUsers();
    }

    async function resetFilters() {
        selectedRoleFilter = "";
        usernameFilter = "";
        await refreshUsers();
    }

    function openCreateDialog() {
        userBeingEdited = null;
        isCreateDialogOpen = true;
    }

    async function createUser(details: NewUserDetails | UpdatedUserDetails) {
        if (!("password" in details)) {
            return;
        }

        await createUserAccount(details);
        isCreateDialogOpen = false;
        await refreshUsers();
    }

    function openEditDialog(user: User) {
        userBeingEdited = user;
        isEditDialogOpen = true;
    }

    async function saveUserChanges(
        details: NewUserDetails | UpdatedUserDetails,
    ) {
        if (!userBeingEdited) {
            return;
        }

        await updateUserAccount(userBeingEdited.id, {
            username: details.username,
            email: details.email,
            role: details.role,
        });
        isEditDialogOpen = false;
        userBeingEdited = null;
        await refreshUsers();
    }

    async function removeUser(userId: string) {
        if (!confirm("Delete this user?")) {
            return;
        }

        try {
            await deleteUserAccount(userId);
            await refreshUsers();
        } catch (error) {
            listErrorMessage = getErrorMessage(error, "Failed to delete user.");
        }
    }

    function closeEditDialog() {
        isEditDialogOpen = false;
        userBeingEdited = null;
    }

    function formatTimestamp(timestamp: string) {
        return new Date(timestamp).toLocaleString();
    }

    function getErrorMessage(error: unknown, fallbackMessage: string) {
        return error instanceof Error ? error.message : fallbackMessage;
    }
</script>

<section class="grid gap-4 p-4">
    <div class="flex justify-between">
        <h2 class="text-2xl font-bold">User Management</h2>
        <button class="btn btn-primary" onclick={openCreateDialog}
            >Add User</button
        >
    </div>

    <div class="card bg-base-100 shadow">
        <div class="card-body">
            <h3 class="card-title">Filter Report</h3>
            <form
                onsubmit={applyFilters}
                class="flex flex-wrap items-end gap-4"
            >
                <label class="form-control">
                    <span class="label-text">Role</span>
                    <select
                        class="select select-bordered"
                        bind:value={selectedRoleFilter}
                    >
                        <option value="">All Roles</option>
                        {#each availableRoleFilters as role (role)}
                            <option value={role}>{role}</option>
                        {/each}
                    </select>
                </label>
                <label class="form-control">
                    <span class="label-text">Username contains</span>
                    <input
                        class="input input-bordered"
                        placeholder="search..."
                        bind:value={usernameFilter}
                    />
                </label>
                <button class="btn btn-primary" type="submit">Apply</button>
                <button
                    class="btn btn-outline"
                    type="button"
                    onclick={resetFilters}>Clear</button
                >
            </form>
            <p>
                Showing <strong>{users.length}</strong> user{users.length !== 1
                    ? "s"
                    : ""}
                {#if selectedRoleFilter}
                    with role <strong>{selectedRoleFilter}</strong>{/if}
                {#if usernameFilter}
                    matching <strong>"{usernameFilter}"</strong>{/if}
            </p>
        </div>
    </div>

    {#if listErrorMessage}
        <div class="alert alert-error">{listErrorMessage}</div>
    {/if}

    <div class="overflow-x-auto">
        <table class="table">
            <thead>
                <tr>
                    <th>Username</th>
                    <th>Email</th>
                    <th>Role</th>
                    <th>Created</th>
                    <th>Actions</th>
                </tr>
            </thead>
            <tbody>
                {#each users as user (user.id)}
                    <tr>
                        <td>{user.username}</td>
                        <td>{user.email}</td>
                        <td>
                            <span
                                class="badge {user.role === 'admin'
                                    ? 'badge-error'
                                    : 'badge-primary'}"
                            >
                                {user.role}
                            </span>
                        </td>
                        <td>{formatTimestamp(user.created_at)}</td>
                        <td>
                            <div class="flex gap-2">
                                <button
                                    class="btn btn-outline btn-sm"
                                    onclick={() => openEditDialog(user)}
                                >
                                    Edit
                                </button>
                                <button
                                    class="btn btn-error btn-sm"
                                    onclick={() => removeUser(user.id)}
                                >
                                    Delete
                                </button>
                            </div>
                        </td>
                    </tr>
                {:else}
                    <tr>
                        <td colspan="5">No users found.</td>
                    </tr>
                {/each}
            </tbody>
        </table>
    </div>
</section>

<UserFormDialog
    bind:open={isCreateDialogOpen}
    mode="create"
    roleOptions={selectableRoles}
    onSubmit={createUser}
    onClose={() => (isCreateDialogOpen = false)}
/>

<UserFormDialog
    bind:open={isEditDialogOpen}
    mode="edit"
    user={userBeingEdited}
    roleOptions={selectableRoles}
    onSubmit={saveUserChanges}
    onClose={closeEditDialog}
/>
