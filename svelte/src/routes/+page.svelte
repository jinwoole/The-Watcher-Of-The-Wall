<script lang="ts">
  import { onMount } from 'svelte';
  import { axiosStore } from '../stores/axiosStore';
  import type { AxiosInstance } from 'axios';

  let axiosInstance: AxiosInstance;

  // 스토어 구독
  axiosStore.subscribe((instance) => {
    axiosInstance = instance;
  });

  onMount(() => {
    // GET 요청
    axiosInstance.get('/vms') // API 엔드포인트
      .then((response) => {
        console.log('응답:', response.data); // 응답 데이터 처리
      })
      .catch((error) => {
        console.error('오류:', error); // 오류
      });
  });
</script>