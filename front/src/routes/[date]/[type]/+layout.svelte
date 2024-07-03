<script>
    import { onMount } from 'svelte';
    import { page } from '$app/stores';
    import { goto } from '$app/navigation';
  
    onMount(() => {
      const { date, type } = $page.params;
  
      // 누가 분명 개떡같은걸 넣겠지
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
  
    let feedback = '';
  
    const submitFeedback = () => {
      console.log('피드백 제출됨: ', feedback);
      // 여기에 피드백 처리 로직 나중에
    };
  </script>
  
<slot />