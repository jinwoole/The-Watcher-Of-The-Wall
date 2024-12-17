<script>
  // ...existing code...
  const mediaOutlets = [
    {
      name: 'ZDNet Korea',
      url: 'https://zdnet.co.kr/'
    },
    {
      name: '전자신문',
      url: 'https://www.etnews.com/'
    },
    {
      name: '디지털데일리',
      url: 'https://www.ddaily.co.kr/'
    },
    {
      name: '바이라인네트워크',
      url: 'https://byline.network/'
    },
    {
      name: 'ITWorld',
      url: 'https://www.itworld.co.kr/'
    },
    {
      name: '디지털투데이',
      url: 'https://www.digitaltoday.co.kr/'
    },
  ];

  function openInNewWindow(url) {
    window.open(url, 'newWindow', 'width=1080,height=720');
  }
</script>

<!-- ...existing code... -->

<ul class="grid grid-cols-2 gap-4">
  {#each mediaOutlets as outlet}
    <li>
      <a href="{outlet.url}" on:click|preventDefault={() => openInNewWindow(outlet.url)} class="flex items-center p-4 border rounded-lg hover:shadow-lg">
        <img
          src="{outlet.name === '바이라인네트워크' ? 'https://byline.network/wp-content/uploads/2023/07/cropped-BN2-32x32.png' : outlet.name === '디지털데일리' ? 'https://www.ddaily.co.kr/assets/images/common/favicon.ico' : outlet.url + 'favicon.ico'}"
          alt="{outlet.name} icon"
          class="w-8 h-8 mr-4"
        />
        <span class="text-xl font-semibold">{outlet.name}</span>
      </a>
    </li>
  {/each}
</ul>

<!-- ...existing code... -->
