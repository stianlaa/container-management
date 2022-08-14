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

export async function tryActivateContainer(containerId) {
    try {
        const response = await api.put("/container/start", {container_id: containerId});
        return response.data;
    } catch (error) {
        console.error(error)
    }
    return null;
}

export async function tryDeactivateContainer(containerId): Promise<boolean> {
    try {
        const response = await api.put("/container/stop", {container_id: containerId});
        return response.data;
    } catch (error) {
        console.error(error)
    }
    return false;
}

export async function tryCreateContainer(createContainerArgs) {
    try {
        const response = await api.put("/container/create", createContainerArgs);
        return response.data;
    } catch (error) {
        console.error(error)
    }
    return null;
}

export async function requestContainerInfo(createContainerArgs) {
    return {}
}

export async function requestContainerLogsLast(createContainerArgs) {
    return {}
}

export async function requestContainerLogsSpan(createContainerArgs) {
    return {}
}