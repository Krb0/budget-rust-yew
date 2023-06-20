use yew::{function_component, html, Html};

#[function_component(LayoutHeader)]
pub fn render() -> Html{
    html!{
        <div class="bg-white">
            <div class="border py-3 px-6">
                <div class="flex justify-between">
                <div class="flex items-center">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 text-red-500" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z" />
                    </svg>
                    <span class="ml-2 font-semibold text-[#252C32]">{"BudgetApp"}</span>
                </div>
                <div class="ml-2 flex">
                    <div class="ml-2 flex cursor-pointer items-center gap-x-1 rounded-md border py-2 px-4 hover:bg-gray-100">
                    <span class="text-sm font-medium">{"Sign in"}</span>
                    </div>
                </div>
                </div>
                </div>
        </div>
    }
}
