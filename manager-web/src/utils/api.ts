// @ts-ignore
import axios, {AxiosResponse} from "axios";

function getHostname() {
    if (import.meta.env.VITE_NUON_WEBAPI_HOST !== undefined) {
        return import.meta.env.VITE_NUON_WEBAPI_HOST;
    }
    return window.location.hostname;
}

function getPort() {
    if (import.meta.env.VITE_NUON_WEBAPI_PORT !== undefined) {
        return import.meta.env.VITE_NUON_WEBAPI_PORT;
    }
    return "8000";
}

const api = axios.create({
    baseURL: `${window.location.protocol}//${getHostname()}:${getPort()}`,
});

function unwrapResponse(response: AxiosResponse<any, any>) {
    try {
        return response.data;
    } catch (error) {
        console.error(error);
    }
    return null;
}

export async function requestDockerCompose(): Promise<object> {
    return unwrapResponse(await api.get("/compose/docker-compose"));
}

export async function listContainers(): Promise<object> {
    return unwrapResponse(await api.get("/container/list"));
}

export async function tryStartContainer(containerId: string): Promise<object> {
    return unwrapResponse(await api.put("/container/start", {container_id: containerId}));
}

export async function tryStopContainer(containerId: string): Promise<boolean> {
    return unwrapResponse(await api.put("/container/stop", {container_id: containerId}));
}

export async function tryRemoveContainer(containerId: string): Promise<boolean> {
    return unwrapResponse(await api.put("/container/remove", {container_id: containerId}));
}

export async function tryRestartContainer(containerId: string): Promise<boolean> {
    return unwrapResponse(await api.put("/container/restart", {container_id: containerId}));
}

export async function tryCreateContainer(createContainerArgs): Promise<object> {
    return unwrapResponse(await api.put("/container/create", createContainerArgs));
}

export async function requestDefaultContainerOptions(containerName: string): Promise<boolean> {
    return unwrapResponse(await api.get("compose/default_config", {params: {container_name: containerName}}));
}

export async function requestContainerInfo(containerId: string): Promise<string> {
    return unwrapResponse(await api.get("container/info", {params: {container_id: containerId}}));
}

export async function requestContainerInfoByName(containerName): Promise<string> {
    return unwrapResponse(await api.get("container/info_by_name", {params: {container_name: containerName}}));
}

async function requestContainerLogs(request): Promise<string> {
    try {
        const response = await api.get("/container/logs", request);
        return response.data.messages.map(([time, message]) => {
            let date = new Date(time).toLocaleString();
            return `[${date}] ${message}\n`;
        }).join("");
    } catch (error) {
        console.error(error)
    }
    return null;
}

export async function requestContainerLogsSpan(container_id: string, since: number, until: number): Promise<string> {
    let request = {
        params: {
            container_id: container_id,
            since: Math.floor(since / 1000),
            until: Math.floor(until / 1000),
        }
    };
    return await requestContainerLogs(request);
}


export async function requestContainerLogsLast(container_id: string, count: Number): Promise<string> {
    let request = {
        params: {
            container_id: container_id,
            tail: count,
        }
    };
    return await requestContainerLogs(request);
}