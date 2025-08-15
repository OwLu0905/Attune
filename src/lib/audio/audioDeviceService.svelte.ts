import { getContext, setContext } from "svelte";

interface AudioDeviceContext {
    audioInputs: MediaDeviceInfo[] | null;
    audioOutputs: MediaDeviceInfo[] | null;
    selectedDeviceId: string;
    initializeDevices: () => Promise<void>;
    setSelectedDevice: (deviceId: string) => void;
}

class AudioDeviceService {
    audioInputs: MediaDeviceInfo[] | null = $state.raw(null);
    audioOutputs: MediaDeviceInfo[] | null = $state.raw(null);
    selectedDeviceId: string = $state("");

    async initializeDevices() {
        if (this.audioInputs && this.audioOutputs) {
            // Devices already initialized, no need to re-initialize
            return;
        }

        try {
            // Request permission first (required for device labels)
            await navigator.mediaDevices.getUserMedia({ audio: true });

            // Get all media devices
            const devices = await navigator.mediaDevices.enumerateDevices();

            // Filter for audio input devices (microphones)
            this.audioInputs = devices.filter(
                (device) => device.kind === "audioinput",
            );

            // Filter for audio output devices (speakers/headphones)
            this.audioOutputs = devices.filter(
                (device) => device.kind === "audiooutput",
            );

            // Set default selected device if none selected
            if (!this.selectedDeviceId && this.audioInputs.length > 0) {
                this.selectedDeviceId = this.audioInputs[0].deviceId;
            }
        } catch (error) {
            console.error("Error accessing media devices:", error);
        }
    }

    setSelectedDevice(deviceId: string) {
        this.selectedDeviceId = deviceId;
    }
}

const AUDIO_DEVICE_CONTEXT_KEY = Symbol("audioDevice");

export function createAudioDeviceContext(): AudioDeviceContext {
    const service = new AudioDeviceService();

    return {
        get audioInputs() {
            return service.audioInputs;
        },
        get audioOutputs() {
            return service.audioOutputs;
        },
        get selectedDeviceId() {
            return service.selectedDeviceId;
        },
        initializeDevices: () => service.initializeDevices(),
        setSelectedDevice: (deviceId: string) =>
            service.setSelectedDevice(deviceId),
    };
}

export function setAudioDeviceContext(context: AudioDeviceContext) {
    setContext(AUDIO_DEVICE_CONTEXT_KEY, context);
}

export function getAudioDeviceContext(): AudioDeviceContext {
    return getContext(AUDIO_DEVICE_CONTEXT_KEY);
}

