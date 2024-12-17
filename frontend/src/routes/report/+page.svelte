<script>
    import { publicLinks, businessLinks, trendLinks } from '../news/stores';
    import { onMount } from 'svelte';
    import { get } from 'svelte/store';
    
    async function fetchMetadata(link) {
        try {
            const proxyUrl = `https://api.allorigins.win/get?url=${encodeURIComponent(link)}`;
            const response = await fetch(proxyUrl);
            if (!response.ok) {
                throw new Error('Network response was not ok');
            }
            const data = await response.json();
            const text = data.contents;
            const parser = new DOMParser();
            const doc = parser.parseFromString(text, 'text/html');
            console.log(doc)

            // 일반적인 메타 태그들을 확인
            let title = doc.querySelector('meta[property="og:title"]')?.content ||
                         doc.querySelector('meta[name="title"]')?.content ||
                         doc.querySelector('title')?.textContent ||
                         '제목 없음';
        
            if (title)
                title = title.includes('| 영남일보') ? title.split('| 영남일보')[0].trim() : title;


            let publisher = doc.querySelector('meta[property="og:site_name"]')?.content ||
                            doc.querySelector('meta[name="publisher"]')?.content ||
                            doc.querySelector('meta[name="twitter:creator"]')?.content ||
                            '출처 없음';
            
            if (publisher == 'zdnet.co.kr')
                publisher = "지디넷코리아";
            if (publisher == '미래를 보는 창 - 전자신문')
                publisher = '전자신문'


            return { title, publisher };
        } catch {
            return {
                title: '제목 불러오기 실패',
                publisher: '출처 불러오기 실패'
            };
        }
    }

    function updateLinkInfo(store, linkInfo) {
        store.update(links => links.map(item => {
            if (item.link === linkInfo.link) {
                return { ...item, ...linkInfo };
            }
            return item;
        }));
    }

    function openLink(link) {
        window.open(link, '_blank', 'width=1024,height=768,menubar=yes,toolbar=yes,location=yes,status=yes');
    }

    function removeLink(store, linkToRemove) {
        store.update(links => links.filter(link => link.link !== linkToRemove));
    }

    // Add functions to move links up and down
    function moveLinkUp(store, link) {
        store.update(links => {
            const index = links.findIndex(item => item.link === link.link);
            if (index > 0) {
                [links[index - 1], links[index]] = [links[index], links[index - 1]];
            }
            return links;
        });
    }

    function moveLinkDown(store, link) {
        store.update(links => {
            const index = links.findIndex(item => item.link === link.link);
            if (index < links.length - 1) {
                [links[index], links[index + 1]] = [links[index + 1], links[index]];
            }
            return links;
        });
    }

    onMount(() => {
        const fetchAllMetadata = async () => {
            const stores = [publicLinks, businessLinks, trendLinks];
            for (const store of stores) {
                const currentLinks = get(store);
                for (const linkInfo of currentLinks) {
                    if (!linkInfo.title || !linkInfo.publisher) {
                        const metadata = await fetchMetadata(linkInfo.link);
                        updateLinkInfo(store, { ...metadata, loading: false, link: linkInfo.link });
                    }
                }
            }
        };
        fetchAllMetadata();
    });

    let editingTitle = null;
    let editedTitle = '';
    let editingPublisher = null;
    let editedPublisher = '';

    function startEditingTitle(link) {
        editingTitle = link;
        editedTitle = link.title;
    }

    function saveTitleEdits(link, store) {
        updateLinkInfo(store, { ...link, title: editedTitle });
        editingTitle = null;
    }

    function cancelTitleEdits() {
        editingTitle = null;
    }

    function startEditingPublisher(link) {
        editingPublisher = link;
        editedPublisher = link.publisher;
    }

    function savePublisherEdits(link, store) {
        updateLinkInfo(store, { ...link, publisher: editedPublisher });
        editingPublisher = null;
    }

    function cancelPublisherEdits() {
        editingPublisher = null;
    }

    let authorName = '';
    let authorTeam = '';
    
    // Load author info from cookies on mount
    onMount(() => {
        authorName = getCookie('authorName') || '';
        authorTeam = getCookie('authorTeam') || '';
    });

    // Cookie helper functions
    function setCookie(name, value, days = 365) {
        const d = new Date();
        d.setTime(d.getTime() + (days * 24 * 60 * 60 * 1000));
        document.cookie = `${name}=${value};expires=${d.toUTCString()};path=/`;
    }

    function getCookie(name) {
        const value = `; ${document.cookie}`;
        const parts = value.split(`; ${name}=`);
        if (parts.length === 2) return parts.pop().split(';').shift();
    }

    function updateAuthorInfo() {
        setCookie('authorName', authorName);
        setCookie('authorTeam', authorTeam);
    }

    function sendNewsletter() {
        const today = new Date();
        const year = today.getFullYear();
        const month = String(today.getMonth() + 1).padStart(2, '0');
        const day = String(today.getDate()).padStart(2, '0');
        const dayOfWeekIndex = today.getDay();
        const daysOfWeek = ['일', '월', '화', '수', '목', '금', '토'];
        const dayOfWeek = daysOfWeek[dayOfWeekIndex];

        const subject = `Daily News - ${year}.${month}.${day} (${dayOfWeek})`;

        let body = `<!DOCTYPE html>
<html>
<head>
<meta charset="UTF-8">
<style>
body {font-family: 'Malgun Gothic', sans-serif; font-size: 11pt;}
</style>
</head>
<body>
안녕하세요<br>${authorTeam} ${authorName}입니다.<br><br>
금일 자 데일리 뉴스를 공유 드립니다.<br><br>`;

        // 각 섹션 추가
        if ($publicLinks.length > 0) {
            body += '<b><br>[정책 / 제안 관련]</b><br>';
            $publicLinks.forEach(link => {
                body += `<a href="${link.link}">${link.title}</a>    &nbsp;&nbsp;&nbsp;&nbsp;${link.publisher}<br>\n`;
            });
        }

        if ($businessLinks.length > 0) {
            body += '<b><br>[기업 / 제품 동향]</b><br>';
            $businessLinks.forEach(link => {
                body += `<a href="${link.link}">${link.title}</a>    &nbsp;&nbsp;&nbsp;&nbsp;${link.publisher}<br>\n`;
            });
        }

        if ($trendLinks.length > 0) {
            body += '<b><br>[트렌드 / 인사이트]</b><br>';
            $trendLinks.forEach(link => {
                body += `<a href="${link.link}">${link.title}</a>    &nbsp;&nbsp;&nbsp;&nbsp;${link.publisher}<br>\n`;
            });
        }

        body += `<br>감사합니다.<br><br>${authorName} 드림.<br>`

        const mailtoLink = `mailto:?subject=${encodeURIComponent(subject)}`;
        window.location.href = mailtoLink;

        const blob = new Blob([body], { type: 'text/html' });
        const url = URL.createObjectURL(blob);
        window.open(url, '_blank');
    }

    let showAddPublic = false;
    let newPublicLink = '';
    
    let showAddBusiness = false;
    let newBusinessLink = '';
    
    let showAddTrend = false;
    let newTrendLink = '';
    
    async function addLink(store, linkInput, setShowAdd) {
        if (linkInput.trim() === '') return;
        const metadata = await fetchMetadata(linkInput);
        store.update(links => [...links, { link: linkInput, ...metadata, loading: false }]);
        linkInput = '';
        setShowAdd(false);
    }
    
    function handleAddPublic() {
        showAddPublic = true;
    }
    
    function handleAddBusiness() {
        showAddBusiness = true;
    }
    
    function handleAddTrend() {
        showAddTrend = true;
    }
    
    function cancelAddPublic() {
        showAddPublic = false;
        newPublicLink = '';
    }
    
    function cancelAddBusiness() {
        showAddBusiness = false;
        newBusinessLink = '';
    }
    
    function cancelAddTrend() {
        showAddTrend = false;
        newTrendLink = '';
    }

</script>

<div class="container mx-auto px-4 py-8">
    <!-- Outlook Send Button -->
    <div class="mb-4 flex items-center space-x-4">
        <button on:click={sendNewsletter} class="bg-blue-500 text-white px-4 py-2 rounded">
            뉴스레터 생성
        </button>
        <!-- Input Fields for Name and Team -->
        <div class="flex items-center space-x-2">
            <input 
                type="text" 
                bind:value={authorTeam} 
                placeholder="부서명" 
                class="border rounded px-2 py-1 w-24"
                on:change={updateAuthorInfo}
            />
            <input 
                type="text" 
                bind:value={authorName} 
                placeholder="이름" 
                class="border rounded px-2 py-1 w-24"
                on:change={updateAuthorInfo}
            />
        </div>
    </div>

    <!-- 공공 섹션 -->
    <div class="mb-8">
        <div class="flex justify-between items-center mb-4">
            <h2 class="text-2xl font-bold text-gray-800 border-b pb-2">
                공공 뉴스 ({$publicLinks.length})
            </h2>
            <button on:click={handleAddPublic} class="bg-green-500 text-white px-3 py-1 rounded">
                추가
            </button>
        </div>
        {#if showAddPublic}
            <div class="mb-4 flex space-x-2">
                <input 
                    type="text" 
                    bind:value={newPublicLink} 
                    placeholder="링크 입력" 
                    class="border rounded px-2 py-1 flex-grow"
                />
                <button on:click={() => addLink(publicLinks, newPublicLink, () => showAddPublic = false)} class="bg-blue-500 text-white px-3 py-1 rounded">
                    추가
                </button>
                <button on:click={cancelAddPublic} class="bg-gray-300 text-gray-700 px-3 py-1 rounded">
                    취소
                </button>
            </div>
        {/if}
        <div class="space-y-3">
            {#if $publicLinks.length === 0}
                <p class="text-gray-500 italic">저장된 링크가 없습니다.</p>
            {/if}
            {#each $publicLinks as link}
                <div class="flex justify-between items-start p-3 bg-gray-50 rounded-lg hover:bg-gray-100">
                    <div class="flex-grow mr-4">
                        <!-- Title Section -->
                        <div class="flex items-center mb-2">
                            {#if editingTitle === link}
                                <input type="text" bind:value={editedTitle} placeholder="Title" class="mr-2"/>
                                <button on:click={() => saveTitleEdits(link, publicLinks)} class="text-green-500">Save</button>
                                <button on:click={cancelTitleEdits} class="text-gray-500 ml-2">Cancel</button>
                            {:else}
                                <span class="font-semibold">{link.title}</span>
                                <button on:click={() => startEditingTitle(link)} class="text-blue-500 ml-2">Edit</button>
                            {/if}
                        </div>
                        <!-- Publisher Section -->
                        <div class="flex items-center mb-2">
                            {#if editingPublisher === link}
                                <input type="text" bind:value={editedPublisher} placeholder="Publisher" class="mr-2"/>
                                <button on:click={() => savePublisherEdits(link, publicLinks)} class="text-green-500">Save</button>
                                <button on:click={cancelPublisherEdits} class="text-gray-500 ml-2">Cancel</button>
                            {:else}
                                {#if link.loading}
                                    <span class="animate-pulse text-sm text-gray-600">로딩 중...</span>
                                {:else}
                                    <span class="text-sm text-gray-600">{link.publisher}</span>
                                    <button on:click={() => startEditingPublisher(link)} class="text-blue-500 ml-2">Edit</button>
                                {/if}
                            {/if}
                        </div>
                        <!-- Link Section -->
                        <div class="flex items-center">
                            <a href={link.link} target="_blank" class="text-blue-600 hover:underline truncate">
                                {link.link}
                            </a>
                        </div>
                    </div>
                    <div class="flex items-center space-x-2">
                        <!-- Move Up Button -->
                        <button
                            class="text-yellow-500 hover:text-yellow-700"
                            on:click={() => moveLinkUp(publicLinks, link)}
                            disabled={$publicLinks[0] === link}
                            title="Move Up"
                        >
                            ↑
                        </button>
                        <!-- Move Down Button -->
                        <button
                            class="text-yellow-500 hover:text-yellow-700"
                            on:click={() => moveLinkDown(publicLinks, link)}
                            disabled={$publicLinks[$publicLinks.length - 1] === link}
                            title="Move Down"
                        >
                            ↓
                        </button>
                        <!-- Close Button -->
                        <button
                            class="text-red-500 hover:text-red-700"
                            on:click={() => removeLink(publicLinks, link.link)}
                            title="Remove"
                        >
                            ✕
                        </button>
                    </div>
                </div>
            {/each}
        </div>
    </div>

    <!-- 기업 섹션 -->
    <div class="mb-8">
        <div class="flex justify-between items-center mb-4">
            <h2 class="text-2xl font-bold text-gray-800 border-b pb-2">
                기업 뉴스 ({$businessLinks.length})
            </h2>
            <button on:click={handleAddBusiness} class="bg-green-500 text-white px-3 py-1 rounded">
                추가
            </button>
        </div>
        {#if showAddBusiness}
            <div class="mb-4 flex space-x-2">
                <input 
                    type="text" 
                    bind:value={newBusinessLink} 
                    placeholder="링크 입력" 
                    class="border rounded px-2 py-1 flex-grow"
                />
                <button on:click={() => addLink(businessLinks, newBusinessLink, () => showAddBusiness = false)} class="bg-blue-500 text-white px-3 py-1 rounded">
                    추가
                </button>
                <button on:click={cancelAddBusiness} class="bg-gray-300 text-gray-700 px-3 py-1 rounded">
                    취소
                </button>
            </div>
        {/if}
        <div class="space-y-3">
            {#if $businessLinks.length === 0}
                <p class="text-gray-500 italic">저장된 링크가 없습니다.</p>
            {/if}
            {#each $businessLinks as link}
                <div class="flex justify-between items-start p-3 bg-gray-50 rounded-lg hover:bg-gray-100">
                    <div class="flex-grow mr-4">
                        <!-- Title Section -->
                        <div class="flex items-center mb-2">
                            {#if editingTitle === link}
                                <input type="text" bind:value={editedTitle} placeholder="Title" class="mr-2"/>
                                <button on:click={() => saveTitleEdits(link, businessLinks)} class="text-green-500">Save</button>
                                <button on:click={cancelTitleEdits} class="text-gray-500 ml-2">Cancel</button>
                            {:else}
                                <span class="font-semibold">{link.title}</span>
                                <button on:click={() => startEditingTitle(link)} class="text-blue-500 ml-2">Edit</button>
                            {/if}
                        </div>
                        <!-- Publisher Section -->
                        <div class="flex items-center mb-2">
                            {#if editingPublisher === link}
                                <input type="text" bind:value={editedPublisher} placeholder="Publisher" class="mr-2"/>
                                <button on:click={() => savePublisherEdits(link, businessLinks)} class="text-green-500">Save</button>
                                <button on:click={cancelPublisherEdits} class="text-gray-500 ml-2">Cancel</button>
                            {:else}
                                {#if link.loading}
                                    <span class="animate-pulse text-sm text-gray-600">로딩 중...</span>
                                {:else}
                                    <span class="text-sm text-gray-600">{link.publisher}</span>
                                    <button on:click={() => startEditingPublisher(link)} class="text-blue-500 ml-2">Edit</button>
                                {/if}
                            {/if}
                        </div>
                        <!-- Link Section -->
                        <div class="flex items-center">
                            <a href={link.link} target="_blank" class="text-blue-600 hover:underline truncate">
                                {link.link}
                            </a>
                        </div>
                    </div>
                    <div class="flex items-center space-x-2">
                        <!-- Move Up Button -->
                        <button
                            class="text-yellow-500 hover:text-yellow-700"
                            on:click={() => moveLinkUp(businessLinks, link)}
                            disabled={$businessLinks[0] === link}
                            title="Move Up"
                        >
                            ↑
                        </button>
                        <!-- Move Down Button -->
                        <button
                            class="text-yellow-500 hover:text-yellow-700"
                            on:click={() => moveLinkDown(businessLinks, link)}
                            disabled={$businessLinks[$businessLinks.length - 1] === link}
                            title="Move Down"
                        >
                            ↓
                        </button>
                        <!-- Close Button -->
                        <button
                            class="text-red-500 hover:text-red-700"
                            on:click={() => removeLink(businessLinks, link.link)}
                            title="Remove"
                        >
                            ✕
                        </button>
                    </div>
                </div>
            {/each}
        </div>
    </div>

    <!-- 트렌드 섹션 -->
    <div class="mb-8">
        <div class="flex justify-between items-center mb-4">
            <h2 class="text-2xl font-bold text-gray-800 border-b pb-2">
                트렌드 뉴스 ({$trendLinks.length})
            </h2>
            <button on:click={handleAddTrend} class="bg-green-500 text-white px-3 py-1 rounded">
                추가
            </button>
        </div>
        {#if showAddTrend}
            <div class="mb-4 flex space-x-2">
                <input 
                    type="text" 
                    bind:value={newTrendLink} 
                    placeholder="링크 입력" 
                    class="border rounded px-2 py-1 flex-grow"
                />
                <button on:click={() => addLink(trendLinks, newTrendLink, () => showAddTrend = false)} class="bg-blue-500 text-white px-3 py-1 rounded">
                    추가
                </button>
                <button on:click={cancelAddTrend} class="bg-gray-300 text-gray-700 px-3 py-1 rounded">
                    취소
                </button>
            </div>
        {/if}
        <div class="space-y-3">
            {#if $trendLinks.length === 0}
                <p class="text-gray-500 italic">저장된 링크가 없습니다.</p>
            {/if}
            {#each $trendLinks as link}
                <div class="flex justify-between items-start p-3 bg-gray-50 rounded-lg hover:bg-gray-100">
                    <div class="flex-grow mr-4">
                        <!-- Title Section -->
                        <div class="flex items-center mb-2">
                            {#if editingTitle === link}
                                <input type="text" bind:value={editedTitle} placeholder="Title" class="mr-2"/>
                                <button on:click={() => saveTitleEdits(link, trendLinks)} class="text-green-500">Save</button>
                                <button on:click={cancelTitleEdits} class="text-gray-500 ml-2">Cancel</button>
                            {:else}
                                <span class="font-semibold">{link.title}</span>
                                <button on:click={() => startEditingTitle(link)} class="text-blue-500 ml-2">Edit</button>
                            {/if}
                        </div>
                        <!-- Publisher Section -->
                        <div class="flex items-center mb-2">
                            {#if editingPublisher === link}
                                <input type="text" bind:value={editedPublisher} placeholder="Publisher" class="mr-2"/>
                                <button on:click={() => savePublisherEdits(link, trendLinks)} class="text-green-500">Save</button>
                                <button on:click={cancelPublisherEdits} class="text-gray-500 ml-2">Cancel</button>
                            {:else}
                                {#if link.loading}
                                    <span class="animate-pulse text-sm text-gray-600">로딩 중...</span>
                                {:else}
                                    <span class="text-sm text-gray-600">{link.publisher}</span>
                                    <button on:click={() => startEditingPublisher(link)} class="text-blue-500 ml-2">Edit</button>
                                {/if}
                            {/if}
                        </div>
                        <!-- Link Section -->
                        <div class="flex items-center">
                            <a href={link.link} target="_blank" class="text-blue-600 hover:underline truncate">
                                {link.link}
                            </a>
                        </div>
                    </div>
                    <div class="flex items-center space-x-2">
                        <!-- Move Up Button -->
                        <button
                            class="text-yellow-500 hover:text-yellow-700"
                            on:click={() => moveLinkUp(trendLinks, link)}
                            disabled={$trendLinks[0] === link}
                            title="Move Up"
                        >
                            ↑
                        </button>
                        <!-- Move Down Button -->
                        <button
                            class="text-yellow-500 hover:text-yellow-700"
                            on:click={() => moveLinkDown(trendLinks, link)}
                            disabled={$trendLinks[$trendLinks.length - 1] === link}
                            title="Move Down"
                        >
                            ↓
                        </button>
                        <!-- Close Button -->
                        <button
                            class="text-red-500 hover:text-red-700"
                            on:click={() => removeLink(trendLinks, link.link)}
                            title="Remove"
                        >
                            ✕
                        </button>
                    </div>
                </div>
            {/each}
        </div>
    </div>
</div>

<style>
    .animate-pulse {
        animation: pulse 2s infinite;
    }
    @keyframes pulse {
        0% {
            opacity: 1;
        }
        50% {
            opacity: 0.5;
        }
        100% {
            opacity: 1;
        }
    }
    .bg-green-500 {
        background-color: #38a169;
    }
    .bg-blue-500 {
        background-color: #4299e1;    }    .bg-gray-300 {        background-color: #e2e8f0;    }
    .text-yellow-500 {
        color: #ecc94b;
    }
</style>