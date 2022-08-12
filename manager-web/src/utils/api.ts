import axios from "axios";

const api = axios.create({
    baseURL: "http://localhost:8000",
});

export async function requestDockerCompose(): Promise<object> {
    try {
        const response = await api.get("/compose/docker-compose");
        return response.data;
    } catch (error) {
        console.error(error);
    }
    return null;
}

export async function listContainers(): Promise<object> {
    try {
        const response = await api.get("/container/list");
        return response.data;
    } catch (error) {
        console.error(error);
    }
    return null;
}

export async function tryActivateContainer(activateArgs) {
    try {
        const response = await api.put("/container/start", activateArgs);
        return response.data;
    } catch (error) {
        console.error(error)
    }
    return null;
}

export async function tryDeactivateContainer(deactivateArgs): Promise<boolean> {
    try {
        const response = await api.put("/container/stop", deactivateArgs);
        return response.data;
    } catch (error) {
        console.error(error)
    }
    return false;
}