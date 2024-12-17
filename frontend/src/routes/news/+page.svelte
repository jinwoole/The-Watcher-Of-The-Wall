<script lang="ts">
    import { writable } from 'svelte/store';
    import { fade } from 'svelte/transition';
    import { onMount } from 'svelte';

    // 뉴스 데이터 인터페이스
    interface NewsItem {
        id: number;
        title: string;
        description: string;
        publishedAt: Date;
        link: string;
    }

    // 호버 상태 관리 개선
    let hoveredNewsId = writable<number | null>(null);
    let hoveredMousePos = writable<{ x: number; y: number } | null>(null);
    let openOptionsId = writable<number | null>(null);

    function handleMouseEnter(event: MouseEvent, newsId: number) {
        hoveredNewsId.set(newsId);
        hoveredMousePos.set({ x: event.clientX, y: event.clientY });
    }

    function handleMouseLeave() {
        hoveredNewsId.set(null);
        hoveredMousePos.set(null);
    }

    function toggleOptions(newsId: number) {
        openOptionsId.update(current => current === newsId ? null : newsId);
    }

    // 날짜 포맷팅 함수
    function formatDate(date: Date): string {
        return new Intl.DateTimeFormat('ko-KR', {
            month: '2-digit',
            day: '2-digit',
            hour: '2-digit',
            minute: '2-digit'
        }).format(date);
    }

    // Add a writable store for keywords
    let keywords = writable<string[]>([]);

    // Fetch keywords from the backend API when the component mounts
    onMount(async () => {
        try {
            const response = await fetch('http://localhost:8000/keywords');
            const data = await response.json();
            keywords.set(data.keywords);
        } catch (error) {
            console.error('Failed to fetch keywords:', error);
        }
    });

    // Add selectedKeyword store
    let selectedKeyword = writable<string | null>(null);

    // Updated handleKeywordClick to implement filtering
    function handleKeywordClick(keyword: string) {
        selectedKeyword.set(keyword);
        scrollToTop(); // Scroll to top after keyword is clicked
    }

    // Import the new stores
    import { publicLinks, businessLinks, trendLinks } from './stores';

    function handleOptionClick(news: NewsItem, category: 'public' | 'business' | 'trend') {
        const linkInfo = {
            link: news.link,
            loading: true
        };
        
        switch(category) {
            case 'public':
                publicLinks.update(links => [...links, linkInfo]);
                break;
            case 'business':
                businessLinks.update(links => [...links, linkInfo]);
                break;
            case 'trend':
                trendLinks.update(links => [...links, linkInfo]);
                break;
        }
        openOptionsId.set(null);
    }

    $: isLongTable = $newsItems.length > 18; // Increased the limit from 15 to 20

    // 스크롤 가능한 컨테이너 참조 변수 추가
    let scrollContainer: HTMLElement;

    // Add a function to scroll to the top smoothly
    function scrollToTop() {
        if (scrollContainer) {
            scrollContainer.scrollTo({ top: 0, behavior: 'smooth' });
        }
    }

    async function handleNewsClick(event: MouseEvent, link: string) {
        event.preventDefault();
        
        // 클립보드에 복사
        try {
            await navigator.clipboard.writeText(link);
            console.log('Link copied to clipboard');
        } catch (err) {
            console.error('Failed to copy link:', err);
        }
        
        // 새 창으로 열기 (너비 1024px, 높이 768px)
        window.open(
            link, 
            '_blank',
            'width=1357,height=768,menubar=yes,toolbar=yes,location=yes,status=yes'
        );
    }

    // Add a writable store for news items
    let newsItems = writable<NewsItem[]>([]);

    // Updated sanitize function to allow only <strong> tags
    function sanitize(str: string): string {
        const parser = new DOMParser();
        const doc = parser.parseFromString(str, 'text/html');
        const allowedTags = ['STRONG'];
        
        // Remove disallowed tags
        const elements = doc.body.querySelectorAll('*');
        elements.forEach(el => {
            if (!allowedTags.includes(el.tagName)) {
                el.replaceWith(document.createTextNode(el.textContent || ''));
            }
        });

        return doc.body.innerHTML;
    }

    // Fetch news data from the backend API based on the selected keyword
    $: if ($selectedKeyword) {
        fetch(`http://localhost:8000/news?keyword=${encodeURIComponent($selectedKeyword)}`)
            .then(response => response.json())
            .then(data => {
                // Parse publishedAt to Date objects and sanitize fields
                const parsedItems = data.items.map((item: NewsItem) => ({
                    ...item,
                    title: sanitize(item.title),
                    description: sanitize(item.description),
                    publishedAt: new Date(item.publishedAt)
                }));
                newsItems.set(parsedItems);
            })
            .catch(error => {
                console.error('Failed to fetch news:', error);
                newsItems.set([]);
            });
    }

    // Add a function to get snippet around the keyword
    function getSnippet(description: string, keyword: string): string {
        const index = description.toLowerCase().indexOf(keyword.toLowerCase());
        if (index === -1) return description.slice(0, 100) + '...';
        const start = Math.max(index - 30, 0);
        const end = Math.min(index + keyword.length + 30, description.length);
        return (start > 0 ? '... ' : '') + description.slice(start, end) + (end < description.length ? ' ...' : '');
    }
</script>

<div class="container px-4 mx-auto relative">
    <div class="keywords-container mb-2 flex flex-wrap gap-1">
        {#each $keywords as keyword}
            <button 
                class={`keyword-button px-2 py-1 bg-blue-500 text-white text-sm rounded hover:bg-blue-600 transition duration-200 flex-shrink-0 transform ${$selectedKeyword === keyword ? 'scale-105 border-2 border-yellow-400' : ''}`}
                on:click={() => handleKeywordClick(keyword)}
            >
                {keyword}
            </button>
        {/each}
    </div>
    <div class="flex flex-col">
        <div class="-mx-4 -my-2 sm:-mx-6 lg:-mx-8">
            <div 
                class="inline-block min-w-full py-2 align-middle md:px-6 lg:px-8"
                style="max-height: calc(110vh - 200px);"
                class:overflow-y-auto={isLongTable}
                bind:this={scrollContainer}
            >
                <div class="border border-gray-200 dark:border-gray-700 md:rounded-lg">
                    <table class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
                        <thead class="bg-gray-50 dark:bg-gray-800">
                            <tr>
                                <th scope="col" class="px-4 py-2 text-sm font-normal text-left rtl:text-right text-gray-500 dark:text-gray-400 w-16">
                                    
                                </th>
                                <th scope="col" class="px-4 py-2 text-md font-normal text-left rtl:text-right text-gray-500 dark:text-gray-400 font-sans tracking-wide">
                                    제목
                                </th>
                                <th scope="col" class="px-4 py-2 text-sm font-normal text-left rtl:text-right text-gray-500 dark:text-gray-400 w-16">
                                    날짜
                                </th>
                                <th scope="col" class="px-4 py-2 text-sm font-normal text-left rtl:text-right text-gray-500 dark:text-gray-400 w-32">
                                    리포트
                                </th>
                            </tr>
                        </thead>
                        <tbody class="bg-white divide-y divide-gray-200 dark:divide-gray-700 dark:bg-gray-900">
                            {#each $newsItems as news (news.id)}
                                <tr>
                                    <td class="px-4 py-2 text-sm text-gray-500 dark:text-gray-300 whitespace-nowrap w-16">
                                        {news.id}
                                    </td>
                                    <td 
                                        class="px-4 py-2 text-gray-800 dark:text-gray-200 relative"
                                        on:mouseenter={(e) => handleMouseEnter(e, news.id)}
                                        on:mouseleave={handleMouseLeave}
                                    >
                                        <a href="{news.link}" 
                                           on:click={(e) => handleNewsClick(e, news.link)}
                                           class="font-bold text-lg hover:text-blue-500">
                                            {news.title}
                                        </a>
                                        <p class="text-sm text-gray-600 dark:text-gray-400 mt-1">
                                            {#if $selectedKeyword}
                                                {@html sanitize(getSnippet(news.description, $selectedKeyword).replace(
                                                    new RegExp(`(${$selectedKeyword})`, 'gi'),
                                                    '<span class="highlight"><strong>$1</strong></span>'
                                                ))}
                                            {:else}
                                                {#if news.description.length > 100}
                                                    {news.description.slice(0, 100)}...
                                                {:else}
                                                    {news.description}
                                                {/if}
                                            {/if}
                                        </p>
                                    </td>
                                    <td 
                                        class="px-4 py-2 text-sm text-gray-500 dark:text-gray-300 whitespace-nowrap w-16"
                                    >
                                        {formatDate(news.publishedAt)}
                                    </td>
                                    <td 
                                        class="px-4 py-2 w-32"
                                    >
                                        <button 
                                            class="keyword-button px-1 py-0.5 bg-green-500 text-white text-xs rounded hover:bg-green-600 transition duration-200"
                                            on:click={() => toggleOptions(news.id)}
                                        >
                                            추가
                                        </button>
                                        {#if $openOptionsId === news.id}
                                            <div class="option-card absolute mt-1 p-1.5 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded shadow-lg flex justify-center space-x-2 w-36">
                                                <button 
                                                    class="keyword-button px-1 py-1 bg-blue-500 text-white text-xs rounded hover:bg-blue-600 transition duration-200"
                                                    on:click={() => handleOptionClick(news, 'public')}
                                                >
                                                    공공
                                                </button>
                                                <button 
                                                    class="keyword-button px-1 py-1 bg-blue-500 text-white text-xs rounded hover:bg-blue-600 transition duration-200"
                                                    on:click={() => handleOptionClick(news, 'business')}
                                                >
                                                    기업
                                                </button>
                                                <button 
                                                    class="keyword-button px-1 py-1 bg-blue-500 text-white text-xs rounded hover:bg-blue-600 transition duration-200"
                                                    on:click={() => handleOptionClick(news, 'trend')}
                                                >
                                                    트랜드
                                                </button>
                                            </div>
                                        {/if}
                                    </td>
                                </tr>
                            {/each}
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    </div>

    {#if $hoveredNewsId !== null && $hoveredMousePos}
        <div 
            class="fixed z-50 pointer-events-none"
            style="left: {$hoveredMousePos.x + 10}px; top: {$hoveredMousePos.y}px; width: 28rem;"
            transition:fade
        >
            <div class="bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg shadow-lg p-4">
                {#each $newsItems.filter(n => n.id === $hoveredNewsId) as news}
                    <p class="text-base text-gray-700 dark:text-gray-300 leading-normal">
                        {@html sanitize(news.description.replace(
                            new RegExp(`(${ $selectedKeyword })`, 'gi'),
                            '<strong>$1</strong>'
                        ))}
                    </p>
                {/each}
            </div>
        </div>
    {/if}

    <!-- Add the scroll to top button -->
    <button
        class="fixed bottom-4 right-4 w-12 h-12 rounded-full bg-blue-500 text-white text-xl flex items-center justify-center"
        on:click={scrollToTop}
    >
        ▲
    </button>
</div>

<style>
    .highlight {
        font-family: 'Courier New', Courier, monospace;
        background-color: yellow;
    }
</style>