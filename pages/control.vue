<template>
    <div class="min-vh-100">
        <h4 class="text-center p-2">Control Panel</h4>
        <p class="text-center p-2 force-selectable"><em>{{ deviceModel }}</em>, FW: <em>{{ deviceFirmwareVersion }}</em>
        </p>

        <div class="input-group mb-3" id="refresh-rate-root">
            <label class="input-group-text" for="inputGroupSelect01">Poll rate</label>
            <select class="form-select" id="inputGroupSelect01" v-model="refreshRate">
                <option :value="1000">1s</option>
                <option :value="500">500ms</option>
                <option :value="300">300ms</option>
            </select>
        </div>

        <br>

        <div class="container text-center mx-auto">
            <div class="row">
                <div class="col">
                    <div class="card">
                        <div class="card-body">
                            <h5 class="card-title">Output Settings</h5>
                            <p class="card-text comp-text value-text editable-value" @click="set_output_voltage">
                                {{ setOutputVoltage }}V
                            </p>
                            <p class="card-text comp-text value-text editable-value" @click="set_output_current">
                                {{ setOutputCurrent }}A
                            </p>
                        </div>
                    </div>
                </div>
                <div class="col">
                    <div class="card">
                        <div class="card-body text-center mx-auto">
                            <div class="container text-center">
                                <div class="row">
                                    <div class="col-sm-1">
                                        <div class="form-check">
                                            <input class="form-check-input" type="checkbox" v-model="outputEnabled"
                                                id="flexCheckDefault" @click="output_toggle">
                                        </div>
                                    </div>
                                    <div class="col-sm-auto">
                                        <h5 class="card-title">Current Output</h5>
                                    </div>
                                </div>
                            </div>
                            <p class="card-text comp-text value-text">
                                {{ outputVoltage }}V
                            </p>
                            <p class="card-text comp-text value-text">
                                {{ outputCurrent }}A
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <br>

        <div class="mx-auto">
            <div class="card">
                <div class="card-body">
                    <h5 class="card-title">Settings</h5>

                    <div class="form-check form-switch">
                        <input class="form-check-input" type="checkbox" role="switch" id="flexSwitchCheckDefault"
                            v-model="ocp" @click="ocp_toggle" />
                        <label class="form-check-label" for="flexSwitchCheckDefault">OCP</label>
                    </div>

                    <div class="form-check form-switch">
                        <input class="form-check-input" type="checkbox" role="switch" id="flexSwitchCheckDefault"
                            v-model="stabilize" />
                        <label class="form-check-label" for="flexSwitchCheckDefault">Stabilize measurements</label>
                    </div>
                </div>
            </div>
        </div>

        <br>

        <button @click="disconnect">Disconnect</button>

        <!-- Dialogs -->
        <div>
            <!-- Modal -->
            <div class="modal fade" id="exampleModal" tabindex="-1" aria-labelledby="exampleModalLabel"
                ref="numberModal" aria-hidden="true">
                <div class="modal-dialog">
                    <div class="modal-content">
                        <div class="modal-header">
                            <h1 class="modal-title fs-5" id="exampleModalLabel">Enter number</h1>
                            <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                        </div>
                        <div class="modal-body">
                            <p>Up to 4 decimals allowed.</p>
                            <div data-mdb-input-init class="form-outline">
                                <input step="0.0001" v-model="numberModalValue" type="number" id="typeNumber"
                                    class="form-control" />
                                <label class="form-label" for="typeNumber">Number input</label>
                            </div>
                        </div>
                        <div class="modal-footer">
                            <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Close</button>
                            <button type="button" class="btn btn-primary" data-bs-dismiss="modal"
                                @click="numberModalSubmit">OK</button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
h4 {
    color: white;
}

p {
    color: white;
}

.comp-text {
    font-family: 'Courier New', Courier, monospace;
}

#refresh-rate-root {
    scale: 80%;
}

.value-text {
    font-size: 20px;
}

.editable-value:hover {
    text-decoration: underline;
    cursor: pointer;
}
</style>

<script lang="ts">
import { invoke } from '@tauri-apps/api/tauri';
import type { Modal } from 'bootstrap';

export default {
    data() {
        return {
            deviceModel: "",
            deviceFirmwareVersion: "",
            setOutputVoltage: "",
            setOutputCurrent: "",
            outputVoltage: "",
            outputCurrent: "",
            outputEnabled: false,
            ocp: false,
            stabilize: false,
            refreshRate: 1000,
            numberModal: null as (Modal | null),
            numberModalValue: 1.00,
            numberModalSubmit: () => { },
        }
    },
    mounted() {
        this.fill_device_info();
        this.refresher();
        this.numberModal = new this.$bootstrap.Modal(this.$refs.numberModal as any);
    },
    methods: {
        async disconnect() {
            await invoke("disconnect");
            this.$router.push({ name: "index" });
        },
        async fill_device_info() {
            let result = await invoke<string[]>("get_device_info");
            let hw = result[0];
            let fw_ver = result[1];

            this.deviceModel = hw;
            this.deviceFirmwareVersion = fw_ver;
        },
        async fill_output_settings() {
            let result = await invoke<number[]>("get_output_settings");
            let voltage = result[0];
            let current = result[1];

            this.setOutputVoltage = voltage.toFixed(4);
            this.setOutputCurrent = current.toFixed(4);
        },
        async fill_output_state() {
            let result = await invoke<[number, number, boolean]>("get_output_state");
            let voltage = result[0];
            let current = result[1];
            let enabled = result[2];

            if (this.stabilize) {
                voltage = voltage < 0 ? 0 : voltage;
                current = current < 0 ? 0 : current;
            }

            if (!this.outputEnabled) {
                this.outputVoltage = "0.0000";
                this.outputCurrent = "0.0000";
            } else {
                this.outputVoltage = voltage.toFixed(4);
                this.outputCurrent = current.toFixed(4);
            }

            this.outputEnabled = enabled;
        },
        async fill_ocp() {
            let result = await invoke<boolean>("get_ocp_state");

            this.ocp = result;
        },
        async output_toggle() {
            let state = !this.outputEnabled;

            await invoke("set_output_state", { enabled: state });
        },
        async ocp_toggle() {
            let state = !this.ocp;

            await invoke("set_ocp_state", { enabled: state });
        },
        async set_output_voltage() {
            this.numberModalSubmit = async () => {
                await invoke("set_output_voltage", { value: this.numberModalValue });
                this.numberModalValue = 1;
            };
            this.open_number_dialog();
        },
        async set_output_current() {
            this.numberModalSubmit = async () => {
                await invoke("set_current_limit", { value: this.numberModalValue });
                this.numberModalValue = 1;
            };
            this.open_number_dialog();
        },
        open_number_dialog() {
            this.numberModal!.show();
        },
        refresher() {
            setTimeout(() => {
                this.fill_output_settings();
                this.fill_output_state();
                this.fill_ocp();
                this.refresher();
            }, this.refreshRate);
        }
    }
}
</script>