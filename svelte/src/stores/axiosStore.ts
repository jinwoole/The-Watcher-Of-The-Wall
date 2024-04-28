import { writable } from 'svelte/store';
import type { Writable } from 'svelte/store';
import axios from 'axios';
import type { AxiosInstance } from 'axios';

// Axios 인스턴스 생성
const axiosInstance: AxiosInstance = axios.create({
  baseURL: 'https://iaas.secretary-report.com/', // API 엔드포인트
  timeout: 10000, // 타임아웃 설정
});

// 스토어 생성
export const axiosStore: Writable<AxiosInstance> = writable(axiosInstance);
