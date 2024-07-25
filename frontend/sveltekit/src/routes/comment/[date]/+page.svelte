<script>
import { goto } from '$app/navigation';
import { page } from '$app/stores';
import { onMount } from 'svelte';
import { validateToken } from '../../../utils/utils';

onMount(() => {
      const { date } = $page.params;

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

        const today = new Date();
        today.setHours(0, 0, 0, 0);
        
        const yesterday = new Date(today);
        yesterday.setDate(today.getDate() - 1);
        
        const tomorrow = new Date(today);
        tomorrow.setDate(today.getDate() + 1);

        dateObject.setHours(0, 0, 0, 0);

        if (dateObject.getTime() >= tomorrow.getTime()) {
          goto('/error');
          return;
        } else if (dateObject.getTime() < yesterday.getTime()) {
          goto('/error/too_late');
          return;
        } 
      }
      let currentToken = localStorage.getItem('token');
      let res = validateToken(currentToken);
      console.log("res: ", res);


    }
);

</script>

<div class="flex items-center justify-center min-h-screen bg-sky-100">
    <div class="card w-96 bg-base-100 shadow-xl">
        <div class="card-body">
        <h2 class="card-title text-center">Daily News 피드백 시스템</h2>
        <p>피드백 감사합니다</p>
        </div>
    </div>
</div>
