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

            //update the text box content
            elements.update(currentElements => {
                const row = currentElements.find(row => row.find(e => e.id === id));
                const element = row?.find(e => e.component_lineno === get(lineFocus));
                if (element) {
                    element.content = textBox?.textContent || '';
                }
                return currentElements
            })
            
            if (document.activeElement === textBox) {
                console.log('enter key pressed')
                lineFocus.update(n => n + 1);
                textBox?.blur();
                nextLineFocus(get(lineFocus));
            }
        } else if (event.key === "Backspace") {
            //check if the current text is empty
            if (textBox?.textContent?.trim() === "") {
                if (document.activeElement === textBox) {
                    console.log("hi");
                    
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

    function previousLineFocus(lineNum: number) {
        //remove current line from list
        if (get(lineFocus) < 0) {
            //slice from elements array
            elements.update(currentElements => {
                //find specific compno
                const row = currentElements.find(row => row.find(e => e.id === id));
                const element = row?.find(e => e.id === id);

                if (row) {
                    if (row.length === 1) {
                        if (element) {
                            specificCompNo = element.component_no;
                            console.log(`specificCompNo: ${specificCompNo}`)
                        }
                        return [
                            ...currentElements.splice(specificCompNo + 1, 1)
                        ]
                    } else {
                        row.splice(get(lineFocus) +1, 1);
                    }
                }
                return currentElements    
            })
        } else {
            let wrapper = document.getElementById(`line${lineNum}`)
            let textBox = wrapper?.querySelector("p");

            //update the array
            elements.update(currentElements => {
                const row = currentElements.find(row => row.find(e => e.id === id));
                const element = row?.findIndex(e => e.id === id)

                if (row) {
                    if (element !== -1) {
                        row.splice(get(lineFocus) + 1, 1);
                    }
                }

                //go through the array and update the component_lineno of each object
                currentElements.forEach((row, rowIndex) => {
                    row.forEach((item, itemIndex) => {
                        item.component_lineno = itemIndex + rowIndex;
                    });
                })

                //filet empty arrays out of it
                const filteredElements = currentElements.filter(subArr => subArr.length > 0);
                filteredElements.forEach((row, rowIndex) => {
                    row.forEach((item, itemIndex) => {
                        item.component_lineno = itemIndex + rowIndex;
                    });
                })
                textBox?.focus();
                return filteredElements
            });
        }
    }

    function onTypingStop() {
        timeoutId = window.setTimeout(() => {
            isTyping = false

            let wrapper = document.getElementById(id);
            let toDoListElement = wrapper?.querySelector(`#line${get(lineFocus)}`);
            let textBox = toDoListElement?.querySelector("p");

            console.log(`linefocus: ${get(lineFocus)}`)
            console.log(`textbox content: ${textBox?.textContent}`)

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

        if (target && target.parentElement) {
            const lineFocusId = target.parentElement.id
            if (lineFocusId) {
                lineFocus.update(n => Number(lineFocusId.slice(4)));
                console.log(`linefocus: ${get(lineFocus)}`)
            }     
        }
    }
</script>


<div id={id}>
    {#each $elements as row (row)}
        {#each row as element (element)}
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
                    class="outline-none p-1 text-sm font-medium w-full
                            {isChecked ? 'line-through text-slate-400' : 'text-black'}"
                    >{element.content}</p>
                </div>
            {/if}
        {/each}
    {/each}
</div>