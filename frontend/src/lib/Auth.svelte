<script lang="ts">
    import { signIn, signUp, type User } from "./api";

    let { onLogin }: { onLogin: (user: User) => void } = $props();

    let activeForm = $state<"login" | "register">("login");

    let loginUsername = $state("");
    let loginPassword = $state("");
    let loginErrorMessage = $state("");
    let isLoginPending = $state(false);

    let registrationUsername = $state("");
    let registrationEmail = $state("");
    let registrationPassword = $state("");
    let registrationErrorMessage = $state("");
    let isRegistrationPending = $state(false);

    async function submitLogin(event: SubmitEvent) {
        event.preventDefault();
        loginErrorMessage = "";
        isLoginPending = true;

        try {
            const authResponse = await signIn(loginUsername, loginPassword);
            onLogin(authResponse.user);
        } catch (error) {
            loginErrorMessage = getErrorMessage(error, "Login failed");
        } finally {
            isLoginPending = false;
        }
    }

    async function submitRegistration(event: SubmitEvent) {
        event.preventDefault();
        registrationErrorMessage = "";
        isRegistrationPending = true;

        try {
            const authResponse = await signUp(
                registrationUsername,
                registrationEmail,
                registrationPassword,
            );
            onLogin(authResponse.user);
        } catch (error) {
            registrationErrorMessage = getErrorMessage(error, "Registration failed");
        } finally {
            isRegistrationPending = false;
        }
    }

    function getErrorMessage(error: unknown, fallbackMessage: string) {
        return error instanceof Error ? error.message : fallbackMessage;
    }
</script>

<div class="hero min-h-screen bg-base-200">
    <div class="card w-full max-w-md bg-base-100 shadow">
        <div class="card-body">
            <h1 class="card-title justify-center text-3xl">CS 348 Portal</h1>
            <div class="tabs tabs-box">
                <button
                    type="button"
                    class="tab flex-1 {activeForm === 'login' ? 'tab-active' : ''}"
                    onclick={() => (activeForm = "login")}
                >
                    Sign In
                </button>
                <button
                    type="button"
                    class="tab flex-1 {activeForm === 'register' ? 'tab-active' : ''}"
                    onclick={() => (activeForm = "register")}
                >
                    Create Account
                </button>
            </div>

            {#if activeForm === "login"}
                <form onsubmit={submitLogin} class="grid gap-4">
                    {#if loginErrorMessage}
                        <div class="alert alert-error">{loginErrorMessage}</div>
                    {/if}
                    <label class="form-control">
                        <span class="label-text">Username</span>
                        <input class="input input-bordered" bind:value={loginUsername} required />
                    </label>
                    <label class="form-control">
                        <span class="label-text">Password</span>
                        <input
                            class="input input-bordered"
                            type="password"
                            bind:value={loginPassword}
                            required
                        />
                    </label>
                    <button class="btn btn-primary" type="submit" disabled={isLoginPending}>
                        {isLoginPending ? "Signing in..." : "Sign In"}
                    </button>
                    <p class="text-center text-sm">
                        Default admin: <strong>admin</strong> / <strong>admin123</strong>
                    </p>
                </form>
            {:else}
                <form onsubmit={submitRegistration} class="grid gap-4">
                    {#if registrationErrorMessage}
                        <div class="alert alert-error">{registrationErrorMessage}</div>
                    {/if}
                    <label class="form-control">
                        <span class="label-text">Username</span>
                        <input
                            class="input input-bordered"
                            bind:value={registrationUsername}
                            required
                        />
                    </label>
                    <label class="form-control">
                        <span class="label-text">Email</span>
                        <input
                            class="input input-bordered"
                            type="email"
                            bind:value={registrationEmail}
                            required
                        />
                    </label>
                    <label class="form-control">
                        <span class="label-text">Password</span>
                        <input
                            class="input input-bordered"
                            type="password"
                            bind:value={registrationPassword}
                            required
                        />
                    </label>
                    <button class="btn btn-primary" type="submit" disabled={isRegistrationPending}>
                        {isRegistrationPending ? "Creating account..." : "Create Account"}
                    </button>
                </form>
            {/if}
        </div>
    </div>
</div>
