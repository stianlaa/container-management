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

export async function tryStartContainer(containerId) {
    try {
        const response = await api.put("/container/start", {container_id: containerId});
        return response.data;
    } catch (error) {
        console.error(error)
    }
    return null;
}

export async function tryStopContainer(containerId): Promise<boolean> {
    try {
        const response = await api.put("/container/stop", {container_id: containerId});
        return response.data;
    } catch (error) {
        console.error(error)
    }
    return false;
}

export async function tryRestartContainer(containerId): Promise<boolean> {
    try {
        const response = await api.put("/container/restart", {container_id: containerId});
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

export async function requestContainerInfo(container_id): Promise<string> {
    try {
        const response = await api.get("/container/info/", {
            params: {
                container_id: container_id,
            }
        });
        return response.data;
    } catch (error) {
        console.error(error)
    }
    return null;
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
            since: Math.floor(since/1000),
            until: Math.floor(until/1000),
        }
    };
    return await requestContainerLogs(request);
}


export async function requestContainerLogsLast(container_id: string, count: Number) {
    let request = {
        params: {
            container_id: container_id,
            tail: count,
        }
    };
    return await requestContainerLogs(request);
}