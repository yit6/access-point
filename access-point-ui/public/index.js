const registerServiceWorker = async () => {
	const swRegistration = await navigator.serviceWorker.register("service.js");
	return swRegistration;
}

const main = async () => {
	const swRegistration = await registerServiceWorker();
}

main();