<template>
    <div class="min-vh-100">
        <h4 class="text-center p-2">Setup</h4>

        <p v-if="loading" class="p-2">Please wait...</p>

        <div class="alert alert-danger p-2" role="alert" v-else-if="error !== null">
            ❌ Error: {{ error }}
        </div>

        <div v-show="!loading" class="p-2">
            <select class="form-select" aria-label="Serial ports" v-model="selected_port">
                <option selected :value="null">Select serial port</option>
                <option v-for="port in ports" :value="port">{{ port }}</option>
            </select>
        </div>

        <button type="button" class="btn btn-primary mx-auto text-center" @click="connect">Connect</button>
    </div>
</template>

<style scoped>
h4 {
    color: white;
}

p {
    color: white;
}
</style>

<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri';

export default {
    data() {
        return {
            loading: true,
            error: null as (string | null),
            ports: [] as (string[]),
            selected_port: useState("port", () => null as (string | null)),
        }
    },
    mounted() {
        invoke<string[]>("get_serial_ports")
            .then((result) => {
                this.ports = result;
            }
            ).catch(error => {
                this.error = error;
            }).finally(() => {
                this.loading = false;
            });
    },
    methods: {
        async connect() {
            try {
                this.loading = true;
                await invoke("connect", { port: this.selected_port });
                this.$router.push({ name: "control" });
            } catch (error) {
                this.error = error as any;
            } finally {
                this.loading = false;
            }
        }
    }
}
</script>