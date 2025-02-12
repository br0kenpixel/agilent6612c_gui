export default defineNuxtConfig({
    devtools: { enabled: false },
    ssr: false,
    css: ["~/node_modules/bootstrap/scss/bootstrap.scss"],
    routeRules: {
        '/': {
            prerender: true
        }
    },
    nitro: {
        preset: "static"
    }
});