<script>
let logout = () => {
	document.cookie = "username=; SameSite=None; Secure; path=/";
	window.location.href="/login/index.html";
}
</script>

<button onclick={logout}>Logout!</button>
