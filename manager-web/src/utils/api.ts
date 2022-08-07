import axios from "axios";

const api = axios.create({
    baseURL: "http://localhost:8000",
});

export async function requestServiceStatus(): Promise<object> {
    try {
        const response = await api.get("/container/list");
        return response.data;
    }
    catch (error) {
        console.error(error);
    }
    return null;
}

export async function tryActivateService(serviceName: string) {
    try {
        const response = await api.put("/container/start", {}, {
            params: {
                services: [serviceName]
            }
        });
    }
    catch (error) {
        console.error(error)
    }
    return null;
}

export async function tryDeactivateService(serviceName: string): Promise<boolean> {
    try {
        const response = await api.put("/service/stop", {}, {
            params: {
                services: [serviceName]
            }
        });
        return true;
    }
    catch (error) {
        console.error(error)
    }
    return false;
}