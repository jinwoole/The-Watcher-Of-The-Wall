<script lang="ts">
    import { writable } from 'svelte/store';
    import { onMount } from 'svelte';

    // Stores
    let keywords = writable<string[]>([]);
    let selectedKeyword = writable<string | null>(null);
    let newKeyword = writable<string>('');

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

    // Handle keyword deletion
    async function handleDeleteClick(keyword: string) {
        try {
            const response = await fetch(`http://localhost:8000/db/items/${encodeURIComponent(keyword)}`, {
                method: 'DELETE',
            });
            if (response.ok) {
                keywords.update(current => current.filter(k => k !== keyword));
            } else {
                console.error('Failed to delete keyword');
            }
        } catch (error) {
            console.error('Error deleting keyword:', error);
        }
    }

    // Handle adding new keyword
    async function handleAddKeyword() {
        if ($newKeyword.trim()) {
            try {
                const response = await fetch('http://localhost:8000/db/items', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify({ value: $newKeyword.trim() }),
                });
                
                if (response.ok) {
                    keywords.update(current => [...current, $newKeyword.trim()]);
                    newKeyword.set('');
                } else {
                    console.error('Failed to add keyword');
                }
            } catch (error) {
                console.error('Error adding keyword:', error);
            }
        }
    }
</script>

<div class="container">
    <h1>키워드 설정</h1>
    
    <div class="keyword-form">
        <input
            type="text"
            bind:value={$newKeyword}
            placeholder="새로운 키워드를 입력하세요"
            class="keyword-input"
        />
        <button on:click={handleAddKeyword} class="add-button">
            추가하기
        </button>
    </div>

    <div class="keywords-container">
        {#each $keywords as keyword}
            <div class="keyword-item">
                <span class="keyword-text">{keyword}</span>
                <button 
                    class="delete-button"
                    on:click={() => handleDeleteClick(keyword)}
                    aria-label="키워드 삭제"
                >
                    ✕
                </button>
            </div>
        {/each}
    </div>
</div>

<style>
    .container {
        max-width: 800px;
        margin: 0 auto;
        padding: 2.618rem 1.618rem;
    }

    h1 {
        font-size: 2.618rem;
        font-weight: 700;
        color: #333;
        margin-bottom: 2.618rem;
    }

    .keyword-form {
        display: flex;
        gap: 1rem;
        margin-bottom: 2.618rem;
    }

    .keyword-input {
        flex: 1;
        padding: 0.618rem 1rem;
        font-size: 1rem;
        border: none;
        border-radius: 0.618rem;
        background-color: #f5f5f7;
        transition: all 0.3s ease;
    }

    .keyword-input:focus {
        outline: none;
        background-color: #fff;
        box-shadow: 0 0 0 2px #0066cc;
    }

    .add-button {
        padding: 0.618rem 1.618rem;
        font-size: 1rem;
        color: white;
        background-color: #0066cc;
        border: none;
        border-radius: 0.618rem;
        cursor: pointer;
        transition: all 0.3s ease;
    }

    .add-button:hover {
        background-color: #0055aa;
    }

    .keywords-container {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
        gap: 1rem;
    }

    .keyword-item {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 0.618rem 1rem;
        background-color: #fff;
        border-radius: 0.618rem;
        box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
        transition: all 0.3s ease;
    }

    .keyword-item:hover {
        transform: translateY(-2px);
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    }

    .keyword-text {
        font-size: 1rem;
        color: #333;
    }

    .delete-button {
        width: 1.618rem;
        height: 1.618rem;
        display: flex;
        align-items: center;
        justify-content: center;
        border: none;
        background: none;
        color: #666;
        cursor: pointer;
        border-radius: 50%;
        transition: all 0.3s ease;
    }

    .delete-button:hover {
        background-color: #ff3b30;
        color: white;
    }
</style>
