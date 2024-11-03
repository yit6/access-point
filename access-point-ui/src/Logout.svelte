<script>
import "./tw.css";
let logout = () => {
	document.cookie = "username=; SameSite=None; Secure; path=/";
	window.location.href="/login/index.html";
}
let back = () => {
	window.location.href="/";
}
</script>

<div class="h-screen bg-gradient-to-b from-white to-indigo-600">

<button class="bg-violet-100 border-violet-600 p-4 m-4 rounded-lg font-bold text-slate-700" onclick={back}>Back</button>
<button class="bg-violet-100 border-violet-600 p-4 m-4 rounded-lg font-bold text-slate-700" onclick={logout}>Log Out</button>

</div>