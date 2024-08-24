<script lang="ts">
    import { get } from "svelte/store";
    import { Checkbox } from "$lib/components/ui/checkbox/index";
    import { tick } from "svelte";
    import { elements, toDoListCount, component_no, lineFocus } from "$lib/store";
    
    export let id = ""
    let isHovered = false;
    let timeoutId: number | null = null;
    let specificCompNo: number;
    let isChecked = false;
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
        let wrapper = document.getElementById(id);
        let toDoListElement = wrapper?.querySelector(`#line${get(lineFocus)}`);
        let textBox = toDoListElement?.querySelector("p");

        if (event.key === "Enter") {
            event.preventDefault();
            
            if (document.activeElement === textBox) {
                lineFocus.update(n => n + 1);
                textBox?.blur();
                nextLineFocus(get(lineFocus));
            }
        } else if (event.key === "Backspace") {
            //check if the current text is empty
            if (textBox?.textContent?.trim() === "") {
                if (document.activeElement === textBox) {
                     if (get(lineFocus) === 0) {
                        toDoListCount.update(n => n-1);
                        component_no.update(n => n-1);
                     }
                    lineFocus.update(n => n -1);
                    textBox?.blur();
                    previousLineFocus(get(lineFocus));
                }
            }
        }

        if (timeoutId !== null) {
            clearTimeout(timeoutId);
        }
    }

    async function nextLineFocus (lineNum: number) {
        //create new element then change the focus
        elements.update(currentElements => {
            const row = currentElements.find(row => row.find(e => e.id === id));
            if (row) {
                const previousComponent = row[row.length - 1].component;
                row.push({ component: previousComponent, component_lineno: get(lineFocus), content: "", id, component_no: get(component_no) });
            }
            return currentElements
        });
        
        await tick();

        let currentLine = document.getElementById(`line${lineNum}`)
        let textBox = currentLine?.querySelector("p");
        textBox?.focus()
    }

    async function previousLineFocus(lineNum: number) {
        //remove current line from list
        elements.update(currentElements => {
            const row = currentElements.find(row => row.find(e => e.id === id));
            if (row) {
                if (get(lineFocus) !== -1) {
                    row.splice(get(lineFocus) -1, 1);
                }
            }

            //filet empty arrays out of it
            const filteredElements = currentElements.filter(subArr => subArr.length > 0);
            return filteredElements
        });

        await tick()

        if (get(lineFocus) < 0) {
            //slice from elements array
            elements.update(currentElements => {
                //find specific compno
                const row = currentElements.find(row => row.find(e => e.id === id));
                const element = row?.find(e => e.id === id);
                if (element) {
                    specificCompNo = element.component_no;
                    console.log(`specificCompNo: ${specificCompNo}`)
                }
                return [
                    ...currentElements.slice(0, specificCompNo),
                    ...currentElements.slice(specificCompNo + 1)
                ]
            })
        } else {
            let currentLine = document.getElementById(`line${lineNum}`)
            let textBox = currentLine?.querySelector("p");
            textBox?.focus()
        }
    }

    function onTypingStop() {
        timeoutId = window.setTimeout(() => {
            isTyping = false
            console.log(`line focus: ${get(lineFocus)}`)

            let wrapper = document.getElementById(id);
            let toDoListElement = wrapper?.querySelector(`#line${get(lineFocus)}`);
            let textBox = toDoListElement?.querySelector("p");

            //change the elements array
            elements.update(currentElements => {
                const row = currentElements.find(row => row.find(e => e.id === id));
                const element = row?.find(e => e.component_lineno === get(lineFocus));
                if (element) {
                    element.content = textBox?.textContent || '';
                }
                return currentElements
            })
        }, 1000)
    }

    function updateLineFocus(event: MouseEvent) {
        const target = event.target as HTMLElement

        if (target && target.parentElement?.id) {
            const parentId = target.parentElement.id;
            lineFocus.update(n => Number(parentId.slice(4)));
            console.log(`line focus: ${get(lineFocus)}`)
        }
    }
</script>


<div id={id}>
    {#each $elements as row}
        {#each row as element}
            {#if element.id.startsWith("toDoList")}
                <div class="items-center flex space-x-2
                            transition-bg duration-400 custom-ease
                            {isHovered ? 'bg-gray-700 text-white' : 'bg-white text-black'}"
                            id={`line${element.component_lineno}`}>
                    <Checkbox bind:checked={isChecked} id="box"/>
                    <p
                    contenteditable=true
                    on:mouseenter={handleMouseEnter}
                    on:mouseleave={handleMouseLeave}
                    on:keydown={handleKeyPress}
                    on:keyup={onTypingStop}
                    on:click={updateLineFocus}
                    class="outline-none p-1 text-sm font-medium
                            {isChecked ? 'line-through text-slate-400' : 'text-black'}"
                    >{element.content}</p>
                </div>
            {/if}
        {/each}
    {/each}
</div>