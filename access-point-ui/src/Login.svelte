<script>
import "./tw.css";
let username = $state("");
let password = $state("");

let login = () => {
	fetch('/user/', {method:'POST', body:JSON.stringify({username:username, password:password})}).then(() => {
		document.cookie = `username=${username}; SameSite=None; Secure; path=/`;
		window.location.href="/";
	});
}
</script>
<div class="h-screen bg-gradient-to-br from-slate-300 to-indigo-600">
<form class="bg-gradient-to-br from-violet-100 to-slate-200 border-violet-600 p-4 m-4 rounded-lg">
	<label class="font-bold text-slate-700" for="username">Username:</label>
	<input class="bg-violet-50 border border-violet-600 text-slate-40 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-4" name="username" type="text" bind:value={username}>

	<br>
	<br>

	<label class="font-bold text-slate-700" for="password">Password:</label>
	<input class="bg-violet-50 border border-violet-600 text-slate-700 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-4" name="password" type="password" bind:value={password}>

	<br>
</form>

<button class="bg-violet-100 border-violet-600 p-4 m-4 rounded-lg font-bold text-slate-700" onclick={login}>Log in!</button>
</div>