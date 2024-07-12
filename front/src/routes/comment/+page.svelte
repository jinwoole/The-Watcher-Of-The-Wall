<script>
    import { page } from '$app/stores';

    let feedback = '';
  
  const submitFeedback = () => {
    console.log('피드백 제출됨: ', feedback);
    // 여기에 피드백 처리 로직 나중에
  };
  </script>
  
  <main class="flex items-center justify-center min-h-screen bg-blue-100">
    <div class="bg-white p-8 rounded shadow-md w-full max-w-md">
      <h1 class="text-2xl font-bold mb-4 text-center">피드백 감사합니다!</h1>
      <div class="mb-4">
        <label for="feedback" class="block text-gray-700">더 자세한 피드백을 남겨주세요</label>
        <input type="text" id="feedback" bind:value={feedback} placeholder="입력"
          class="input input-bordered w-full mt-2" />
      </div>
      <button on:click={submitFeedback} class="btn btn-primary w-full">제출</button>
    </div>
  </main>
  