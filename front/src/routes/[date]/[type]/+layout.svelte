<script>
    import { onMount } from 'svelte';
    import { page } from '$app/stores';
    import { goto } from '$app/navigation';
  
    onMount(() => {
      const { date, type } = $page.params;
  
      // 제발 이상한걸 입력하지 말아주세요
      const dateRegex = /^\d{8}$/;
      const isValidDate = dateRegex.test(date);
  
      if (isValidDate) {
        const year = parseInt(date.substring(0, 4));
        const month = parseInt(date.substring(4, 6)) - 1;
        const day = parseInt(date.substring(6, 8));
        const dateObject = new Date(year, month, day);
  
        const isValidDateObject =
          dateObject.getFullYear() === year &&
          dateObject.getMonth() === month &&
          dateObject.getDate() === day;
  
        if (!isValidDateObject) {
          goto('/error');
          return;
        }
      } else {
        goto('/error');
        return;
      }
  
      // type 검증
      const validTypes = ['good', 'soso', 'bad'];
      const isValidType = validTypes.includes(type);
  
      if (!isValidType) {
        goto('/error');
        return;
      }
    });
  

  </script>
  
<slot />