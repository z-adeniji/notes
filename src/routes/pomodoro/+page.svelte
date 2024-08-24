<script lang="ts">
    import GradientAnimation from "$lib/components/ui/gradientAnimation/gradientAnimation.svelte";

    let active: string | null = null;
</script>

<div class="container relative w-full h-screen">
    <GradientAnimation>
      <div class="pointer-events-none absolute inset-0 z-50 flex items-center justify-center px-4 text-center text-3xl font-bold text-white md:text-4xl lg:text-7xl">
        <p class="bg-gradient-to-b from-white/80 to-white/20 bg-clip-text text-transparent drop-shadow-2xl">
          Spaces
        </p>
      </div>
    </GradientAnimation>
</div>