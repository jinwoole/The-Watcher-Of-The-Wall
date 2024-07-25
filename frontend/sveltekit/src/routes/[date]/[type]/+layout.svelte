<script>
    import { onMount } from 'svelte';
    import { page } from '$app/stores';
    import { goto } from '$app/navigation';
    import { requestToken, validateToken } from '../../../utils/utils.js';


        // 토큰 검증 및 관리 함수
    export async function manageToken(date, type) {
      let currentToken = localStorage.getItem('token') || '';
      console.log(currentToken)
      
      if (!currentToken) {
        try {
          const newToken = await requestToken(date, type);
          localStorage.setItem('token', newToken.token);
          goto("/comment/" + date)
        } catch (error) {
          goto("/error")
        }
      } else {
        try {
          console.log("there is a token");
          const isValid = await validateToken(currentToken);
          if (!isValid) {
            // 토큰이 유효하지 않으면 새 토큰 요청
            const newToken = await requestToken();
            token.set(newToken);
          }
        } catch (error) {
          goto("/comment/" + date)
          // 에러 처리 로직
        }
      }
    }


    onMount(() => {
      const { date, type } = $page.params;

      // 누가 분명 이상한거 입력할테니
      const dateRegex = /^\d{8}$/;
      const isValidDate = dateRegex.test(date);

      if (isValidDate) {
        const year = parseInt(date.substring(0, 4));
        const month = parseInt(date.substring(4, 6)) - 1; //그지같은 자바스크립트 0월부터 시작하는거 실화냐
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

        // 현재 날짜만! 기준으로 어제, 오늘, 내일 계산
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
      } else {
        goto('/error');
        return;
      }
      // 타입 검증
      const validTypes = ['good', 'soso', 'bad'];
      const isValidType = validTypes.includes(type);

      if (!isValidType) {
        goto('/error');
        return;
      }

      // 토큰 검증
      manageToken(date, type);



    });
</script>

<slot />