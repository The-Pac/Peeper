<template>
  <router-view/>
</template>

<script>
</script>

<style lang="scss">
body {
  margin: 0;
  background-color: black;
}
#app{
  width: 100%;
  height: 100vh;
}

@font-face {
  font-family: 'Roboto';
  font-style: normal;
  font-weight: 400;
  src: url('../assets/font/Roboto-Medium.ttf') format('woff2');
  unicode-range: U+0000-00FF, U+0131, U+0152-0153, U+02BB-02BC, U+02C6, U+02DA, U+02DC, U+2000-206F, U+2074, U+20AC, U+2122, U+2191, U+2193, U+2212, U+2215, U+FEFF, U+FFFD;
}

@keyframes gradiant {
  0% {
    background-position: 50% 0
  }
  100% {
    background-position: 50% 100%
  }
}

.text-design {
  color: white;
  display: flex;
  justify-content: center;
  align-content: center;
  margin: 0;
  transform-style: preserve-3d;
}

.text-with-font {
  font-weight: bold;
  font-family: Roboto, sans-serif;
}

.scroll_bar_smooth {
  overflow-x: clip;
  overflow-y: scroll;
  scroll-behavior: smooth;
}

.scroll_bar_smooth::-webkit-scrollbar {
  border-radius: 25px;
  width: 8px;
  height: 8px;
}

.scroll_bar_smooth::-webkit-scrollbar-thumb {
  background: #B3AFB3;
  border-radius: 20px;
}

.scroll_bar_smooth::-webkit-scrollbar-thumb:hover {
  background: #696969;
}

.scroll_bar_smooth::-webkit-scrollbar-track {
  background: transparent;
  border-radius: 0;
  box-shadow: inset 0 0 0 0 #F0F0F0;
  margin-top: 10px;
}

</style>
