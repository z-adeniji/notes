<script lang="ts">
    import { tick } from "svelte";
    import { elements , paraCount, component_no} from "$lib/store";

    export let id = "";
    let paraText = "Click to Edit";
    let isHovered = false;
    let timeoutId: number | null = null;
    let specificCompNo: number;
    let isTyping = false;

    function handleMouseEnter() {
        timeoutId = window.setTimeout(() => {
            isHovered = true
        }, 500);
    }

    function handleMouseLeave() {
        if (timeoutId !== null) {
            window.clearTimeout(timeoutId);
            timeoutId = null;
        }
        isHovered = false;
    }

    async function handleKeyPress(event: KeyboardEvent) {
        isTyping = true;

        let wrapper = document.getElementById(id)
        await tick()
        let textBox = wrapper?.querySelector("p");

        if (event.key === "Enter") {
            event.preventDefault();

            if (document.activeElement === textBox) {
                if (textBox) {
                    textBox.blur();
                }
            }
        } else if (event.key === "Backspace") {
            if (textBox?.textContent?.trim() === "") {
                textBox.blur();
                paraCount.update(n => n-1);
                component_no.update(n => n-1);

                //remove the element
                elements.update(currentElements => {
                    const row = currentElements.find(row => row.find(e => e.id === id));
                    const element = row?.find(e => e.id === id);
                    console.log(element);

                    if(element) {
                        specificCompNo = element.component_no;
                        console.log(`specificCompNo: ${specificCompNo}`)
                    }
                    return [
                    ...currentElements.slice(0, specificCompNo),
                    ...currentElements.slice(specificCompNo + 1)   
                    ]
                })
            }
        }

        if (timeoutId !== null) {
            clearTimeout(timeoutId);
        }
    }

    function onTypingStop() {
        timeoutId = window.setTimeout(() => {
            isTyping = false
            console.log(`typing: ${isTyping}`)

            let wrapper = document.getElementById(id)
            let textBox = wrapper?.querySelector("p");

            //change the elements array
            elements.update(currentElements => {
                const row = currentElements.find(row => row.find(e => e.id === id));
                const element = row?.find(e => e.id === id);
                if (element) {
                    console.log(wrapper);
                    element.content = textBox?.textContent || '';
                }
                return currentElements
            })
        }, 1000)
    }
</script>

<div id = {id}>
    <p
    contenteditable=true
    on:mouseenter={handleMouseEnter}
    on:mouseleave={handleMouseLeave}
    on:keydown={handleKeyPress}
    on:keyup={onTypingStop}
    class="outline-none text-base transition-bg duration-400 custom-ease p-1 {isHovered ? 'bg-gray-400 text-white' : 'bg-white text-black'}"
    >{paraText}</p>
</div>