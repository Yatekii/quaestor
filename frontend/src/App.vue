<template>
  <div id="app">
    <b-container fluid id="nav" style="padding-left: 10em; padding-right: 10em;">
      <b-row>
        <b-col>
          <router-link to="/">Create</router-link>&nbsp;
          <router-link to="/list">List</router-link>
        </b-col>
      </b-row>
    </b-container>
    <router-view />
  </div>
</template>

<style lang="scss">
html,
body {
  height: 100%;
}

#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: #2c3e50;
  height: 100%;
}

#nav {
  padding: 30px;

  a {
    font-weight: bold;
    color: #2c3e50;

    &.router-link-exact-active {
      color: #42b983;
    }
  }
}
</style>
