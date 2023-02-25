import { invoke } from "@tauri-apps/api/tauri";

async function get_minecraft() {
    try {
        const users = await invoke("get_minecraft", {});
    } catch (error) {
        console.error(error);
    }
}