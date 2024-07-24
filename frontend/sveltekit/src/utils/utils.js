const BASE_URL = 'https://backendfeedback.jinwoolee.info';

export async function requestToken(date, type) {
    try {
        const endpoint = `/api/check/${date}/${type}`;
        const response = await apiRequest(endpoint, 'GET');
        return response;
    } catch (error) {
        console.error("Failed to request token:", error);
        throw error;
    }
}

export async function validateToken(token) {
    try {
        const endpoint = `/api/validate/${token}`;
        const response = await apiRequest(endpoint, 'GET');
        return response;
    } catch (error) {
        console.error("Failed to validate token:", error);
        throw error;
    }
}

export async function apiRequest(endpoint, method, body) {
    try {
        const url = `${BASE_URL}${endpoint}`;
        const options = {
            method,
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(body)
        };
        const response = await fetch(url, options);
        if (!response.ok) {
            throw new Error('API request failed');
        }
        const data = await response.json();
        return data;
    } catch (error) {
        console.error("API request failed:", error);
        throw error;
    }
}