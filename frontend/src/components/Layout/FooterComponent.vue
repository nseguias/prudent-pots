<template>
  <div class="container-fluid bg-pp-purple-1">
    <footer class="d-flex flex-wrap justify-content-between align-items-center py-3 border-top text-white">
      <p class="col-md-4 mb-0">© {{ new Date().getFullYear() }} Prudent Pots</p>

      <div
        class="col-md-4 d-flex flex-column align-items-center justify-content-center mb-3 mb-md-0 me-md-auto link-dark text-decoration-none">
        <img class="logo" src="favicon.ico" alt="Prudent Pots">
      </div>

      <ul class="nav col-md-4 justify-content-end list-unstyled d-flex">
        <li class="ms-3">
          <a href="https://github.com/magiodev/prudent-pots" target="_blank" rel="noopener noreferrer">
            <b-icon-github class="text-white"></b-icon-github>
          </a>
        </li>
        <li class="ms-3">
          <a href="https://twitter.com/magiodev/status/1781957200446922947" target="_blank" rel="noopener noreferrer">
            <b-icon-twitter class="text-white"></b-icon-twitter>
          </a>
        </li>
      </ul>
    </footer>
  </div>
</template>

<script>
export default {
  name: "FooterComponent",
}
</script>

<style scoped lang="scss">
.logo {
  height: 32px;
}
</style>