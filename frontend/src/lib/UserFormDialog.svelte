<script lang="ts">
    import type { NewUserDetails, UpdatedUserDetails, User } from "./api";

    type DialogMode = "create" | "edit";
    type RoleOption = { value: string; label: string };

    let {
        open = $bindable(false),
        mode,
        user = null,
        roleOptions,
        onSubmit,
        onClose,
    }: {
        open?: boolean;
        mode: DialogMode;
        user?: User | null;
        roleOptions: RoleOption[];
        onSubmit: (
            details: NewUserDetails | UpdatedUserDetails,
        ) => Promise<void>;
        onClose: () => void;
    } = $props();

    let username = $state("");
    let email = $state("");
    let password = $state("");
    let role = $state("user");
    let errorMessage = $state("");

    const isCreateMode = $derived(mode === "create");
    const dialogTitle = $derived(isCreateMode ? "Add User" : "Edit User");
    const submitLabel = $derived(isCreateMode ? "Create" : "Save");

    $effect(() => {
        if (!open) {
            return;
        }

        errorMessage = "";
        username = user?.username ?? "";
        email = user?.email ?? "";
        password = "";
        role = user?.role ?? "user";
    });

    async function submitForm(event: SubmitEvent) {
        event.preventDefault();
        errorMessage = "";

        try {
            if (isCreateMode) {
                await onSubmit({ username, email, password, role });
            } else {
                await onSubmit({ username, email, role });
            }
        } catch (error) {
            errorMessage =
                error instanceof Error ? error.message : "Failed to save user.";
        }
    }

    function closeDialog() {
        open = false;
        onClose();
    }
</script>

{#if open}
    <div class="modal modal-open">
        <div class="modal-box">
            <h3 class="text-lg font-bold">{dialogTitle}</h3>
            <form onsubmit={submitForm} class="grid gap-4">
                {#if errorMessage}
                    <div class="alert alert-error">{errorMessage}</div>
                {/if}
                <label class="form-control">
                    <span class="label-text">Username</span>
                    <input
                        class="input input-bordered"
                        bind:value={username}
                        required
                    />
                </label>
                <label class="form-control">
                    <span class="label-text">Email</span>
                    <input
                        class="input input-bordered"
                        type="email"
                        bind:value={email}
                        required
                    />
                </label>
                {#if isCreateMode}
                    <label class="form-control">
                        <span class="label-text">Password</span>
                        <input
                            class="input input-bordered"
                            type="password"
                            bind:value={password}
                            required
                        />
                    </label>
                {/if}
                <label class="form-control">
                    <span class="label-text">Role</span>
                    <select class="select select-bordered" bind:value={role}>
                        {#each roleOptions as roleOption (roleOption.value)}
                            <option value={roleOption.value}
                                >{roleOption.label}</option
                            >
                        {/each}
                    </select>
                </label>
                <div class="modal-action">
                    <button
                        class="btn btn-outline"
                        type="button"
                        onclick={closeDialog}>Cancel</button
                    >
                    <button class="btn btn-primary" type="submit"
                        >{submitLabel}</button
                    >
                </div>
            </form>
        </div>
    </div>
{/if}
