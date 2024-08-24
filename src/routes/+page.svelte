<script lang="ts">
	import * as ContextMenu from "$lib/components/ui/context-menu/index.js";
	import Header from "$lib/components/elements/header.svelte";
	import Paragraph from "$lib/components/elements/paragraph.svelte";
	import List from "$lib/components/elements/list.svelte";
	import ToDoList from "$lib/components/elements/toDoList.svelte";
	import { get } from 'svelte/store';
	import { elements, component_no } from "$lib/store";
	import { headerCount, paraCount, listCount, toDoListCount } from "$lib/store";
    import { invoke } from "@tauri-apps/api";
	import { onDestroy } from "svelte";

	const Eunsubscribe = elements.subscribe((value) => {
		console.log('elements value changed:', value);

		//convert to string i think
		const elementsWithStringComponents = value.map((row) =>
			row.map((element) => ({
				...element,
				component: element.component.name,
			}))
		)
		//send to backend
		invoke("auto_save", {elements: elementsWithStringComponents});
	})

	const Hunsubscribe = headerCount.subscribe((value) => {
		console.log('headercount value changed:', value);
	})

	const Punsubscribe = paraCount.subscribe((value) => {
		console.log('paracount value changed:', value);
	})

	const Lunsubscribe = listCount.subscribe((value) => {
		console.log('listcount value changed:', value);
	})

	const TDLunsubscribe = toDoListCount.subscribe((value) => {
		console.log('todolistcount value changed:', value);
	})

	//define function to add the component
	function addComponent(type: String) {
		let component: any;
		let id: string = "";

		switch(type) {
			case 'header':
				component = Header;
				headerCount.update(n => {
					id = `header${n+1}`;
					component_no.update(n => n+1);
					return n + 1;
				});
				break;
			case 'paragraph':
				component = Paragraph;
				paraCount.update(n => {
					id = `para${n+1}`;
					component_no.update(n => n+1);
					return n + 1;
				});
				break;
			case 'list':
				component = List;
				listCount.update(n => {
					id = `list${n+1}`;
					component_no.update(n => n+1);
					return n + 1;
				});
				break;
			case 'toDoList':
				component = ToDoList;
				toDoListCount.update(n => {
					id = `toDoList${n+1}`;
					component_no.update(n => n+1);
					return n + 1;
				});
				break;
			default:
				console.log(`No component found for type ${type}`)
				return;
		}

		//updating the store
		//identify is its a list or a todolist
		//if it is, set the line no to 0
		if (component === List || component === ToDoList) {
			elements.update(currentElements => [
				...currentElements,
				[{ component, component_lineno: 0, content: `${type}`, id , component_no: get(component_no)}]
			]);
		} else {
			elements.update(currentElements => [
				...currentElements,
				[{ component, component_lineno: null, content: `${type}`, id , component_no: get(component_no)}]
			]);
		}
	}

	onDestroy(() => {
		Eunsubscribe();
		Hunsubscribe();
		Punsubscribe();
		Lunsubscribe();
		TDLunsubscribe();
	})
</script>

<body class="h-full m-0 p-[2.5%]">
	<ContextMenu.Root>
		<!--dummy element doesnt affect the layout-->
		<ContextMenu.Trigger class="content-page min-h-screen h-full w-full bg-[#FFC3D0]" >
			{#each $elements as row}
				{#each row as element}
					{#if row.length === 1}
						<svelte:component this={element.component} id={element.id}/>
					{/if}
				{/each}
				{#if row.length > 1}
					<svelte:component this={row[0].component} id={row[0].id}/>
				{/if}
			{:else}
				<h1>No elements here</h1>
			{/each}
		</ContextMenu.Trigger>
		<ContextMenu.Content class="w-64">
			<ContextMenu.Label inset>Bullet v1.0</ContextMenu.Label>
			<ContextMenu.Sub>
				<ContextMenu.SubTrigger inset>Add New Element</ContextMenu.SubTrigger>
				<ContextMenu.SubContent class="w-48">
					<ContextMenu.Item on:click={() => addComponent("header")}>Header</ContextMenu.Item>
					<ContextMenu.Item on:click={() => addComponent("paragraph")}>Paragraph</ContextMenu.Item>
					<ContextMenu.Item on:click={() => addComponent("list")}>List</ContextMenu.Item>
					<ContextMenu.Item on:click={() => addComponent("toDoList")}>To Do List</ContextMenu.Item>
					<ContextMenu.Separator />
					<ContextMenu.Item on:click={() => addComponent("notebooks")}>Notebooks</ContextMenu.Item>
					<ContextMenu.Item on:click={() => addComponent("flashcards")}>Flashcards</ContextMenu.Item>
					<ContextMenu.Item on:click={() => addComponent("calendar")}>Calendar</ContextMenu.Item>
					<ContextMenu.Item on:click={() => addComponent("pomodoro")}>Pomodoro Corner</ContextMenu.Item>
				</ContextMenu.SubContent>
			</ContextMenu.Sub>
			<ContextMenu.Item inset>Add New Page</ContextMenu.Item>
		</ContextMenu.Content>
	</ContextMenu.Root>
</body>