import { goto } from '$app/navigation';

const BASE_URL = 'https://your-api-base-url.com'; // 여기에 실제 API 기본 URL을 입력하세요

export async function apiRequest(endpoint, method = 'GET', data = null, token = null) {
    const url = `${BASE_URL}${endpoint}`;
    const headers = {
        'Content-Type': 'application/json',
    };

    if (token) {
        headers['Authorization'] = `Bearer ${token}`;
    }

    const options = {
        method,
        headers,
    };

    if (data) {
        options.body = JSON.stringify(data);
    }

    try {
        const response = await fetch(url, options);
        
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        
        return await response.json();
    } catch (error) {
        console.error("API request failed:", error);
        throw error;
    }
}

export async function validateToken(token) {
    if (!token) {
        throw new Error('Token is required');
    }

    try {
        const response = await apiRequest('/validate-token', 'POST', { token });
        
        if (response === 'ok') {
            return true;
        } else if (response === 'no') {
            return false;
        } else {
            goto('/error');
        }
    } catch (error) {
        goto('/error');
    }
}