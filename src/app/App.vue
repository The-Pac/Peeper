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

</style>
