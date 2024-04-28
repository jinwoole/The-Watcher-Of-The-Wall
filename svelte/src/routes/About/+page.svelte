<script lang="ts">
    import { onMount } from 'svelte';
    import { initFlowbite } from 'flowbite';

    onMount(() => {
        initFlowbite();
    });
</script>

<main class="flex flex-col items-center justify-center px-6 py-8 bg-gray-100 min-h-screen">
    <div class="mx-auto max-w-3xl"> <!-- 폭을 확장 -->
        <!-- 로고 및 제목 -->
        <div class="text-center mb-10">
            <a href="/" class="text-3xl font-bold text-gray-800">Infra Pricing</a>
        </div>

        <!-- 프라이싱 카드 -->
        <div class="bg-white shadow-lg rounded-lg p-6 space-y-8">
            <div class="text-center">
                <h2 class="text-2xl font-bold text-gray-900">VM 요금제</h2>
                <p class="text-gray-500">모든 옵션은 95% 이상의 가용성을 보장하기 위해 노력할 예정입니다</p>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                <!-- Free 플랜 -->
                <div class="bg-gray-50 p-6 rounded-lg shadow text-center">
                    <h3 class="text-xl font-semibold">Free</h3>
                    <ul class="text-left mt-4 space-y-2 list-inside list-disc">
                        <li>0.5 vcpu</li>
                        <li>1GB ram</li>
                        <li>32GB Storage</li>
                    </ul>
                </div>

                <!-- Basic 플랜 -->
                <div class="bg-gray-50 p-6 rounded-lg shadow text-center">
                    <h3 class="text-xl font-semibold">Basic</h3>
                    <p class="text-gray-600">$2.99/month</p>
                    <ul class="text-left mt-4 space-y-2 list-inside list-disc">
                        <li>2vcpu</li>
                        <li>4GB ram</li>
                        <li>128GB Storage</li>
                        <li>Tech Support</li>
                        <p class="text-gray-600">Tech Support is not reliable</p>
                    </ul>
                    <button class="mt-6 px-5 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700">Get Started</button>
                </div>

                <!-- Dev 플랜 -->
                <div class="bg-gray-50 p-6 rounded-lg shadow text-center">
                    <h3 class="text-xl font-semibold">Dev</h3>
                    <p class="text-gray-600">Thanks</p>
                    <ul class="text-left mt-4 space-y-2 list-inside list-disc">
                        <li>Basic Plan</li>
                        <li>Custom Offers</li>
                    </ul>
                </div>
            </div>
        </div>
    </div>
</main>
