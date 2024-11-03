<script>
let username = $state("");
let password = $state("");

let login = () => {
	fetch('/user/', {method:'POST', body:JSON.stringify({username:username, password:password})}).then(() => {
		document.cookie = `username=${username}; SameSite=None; Secure; path=/`;
		window.location.href="/";
	});
}
</script>

<form>
	<label for="username">Username:</label>
	<input name="username" type="text" bind:value={username}>

	<br>

	<label for="password">Password:</label>
	<input name="password" type="password" bind:value={password}>

	<br>
</form>

<button onclick={login}>Log in!</button>
