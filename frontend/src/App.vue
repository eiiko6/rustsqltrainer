<script setup>
</script>

<template>
  <div class="app">
    <header>
      <h1>SQL Trainer</h1>
      <nav>
        <router-link to="/">Home</router-link>
      </nav>
    </header>

    <main>
      <router-view />
    </main>
  </div>
</template>

<style>
body {
  margin: 0;
  font-family: system-ui, sans-serif;
  background: #f5f5f5;
  color: #222;
}

.app {
  max-width: 800px;
  padding: 1rem;
}

header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
}

nav a {
  margin-left: 1rem;
  text-decoration: none;
  color: #007acc;
}

nav a.router-link-exact-active {
  font-weight: bold;
}

main {
  background: white;
  padding: 2rem;
  border-radius: 12px;
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.1);
}
</style>
