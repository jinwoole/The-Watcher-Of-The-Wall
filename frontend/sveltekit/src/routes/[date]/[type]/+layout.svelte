<script>
    import { onMount } from 'svelte';
    import { page } from '$app/stores';
    import { goto } from '$app/navigation';
    import { token } from '../../../utils/store.js';
    import { requestToken } from '../../../utils/utils.js';

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
        // 토큰 유무 검증
        if ($token) {
          rsp = validateToken($token);
          if (rsp === true) {
            goto("/comment/" + date);
          } 
          else if (rsp === false) {
            //유효기간 경과 : 발급
            // 여기부터 작업해야함, 그런데 하려면 백엔드를 만들어야 함
            res = requestToken(date, type);
            console.log(res);
          } 
          else {
            goto('/error');
          }
        } 
        else {
            //토큰 없는 경우: 발급
            requestToken(date, type)
            .then(res => {
              console.log(res.token);
            })
            .catch(error => {
              console.error(error);
            });
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
    });

    





</script>


<!-- svelte-ignore non-top-level-reactive-declaration -->
<slot />