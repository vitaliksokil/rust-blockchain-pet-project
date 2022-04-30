<template>
  <nav id="sidebarMenu" class="col-md-3 col-lg-2 d-md-block bg-light sidebar collapse"
       style="height: calc(100vh - 71px)">
    <div class="position-sticky pt-3">
      <ul class="nav flex-column">
        <li class="nav-item">
          <router-link
              :to="{name:'profile-dashboard'}"
              custom
              v-slot="{ href, navigate, isExactActive }">
            <a :href="href" @click="navigate"
               class="nav-link scrollto" :class="isExactActive ? 'active' : ''">
              <font-awesome-icon icon="house"/>
              Dashboard
            </a>
          </router-link>
        </li>
        <li class="nav-item">
          <router-link
              :to="{name:'create-zoo'}"
              custom
              v-slot="{ href, navigate, isExactActive }">
            <a :href="href" @click="navigate"
               class="nav-link scrollto" :class="isExactActive ? 'active' : ''">
              <font-awesome-icon icon="circle-plus" />
              Create Zoo
            </a>
          </router-link>
        </li>
        <li class="nav-item">
          <router-link
              :to="{name:'update-zoo'}"
              custom
              v-slot="{ href, navigate, isExactActive }">
            <a :href="href" @click="navigate"
               class="nav-link scrollto" :class="isExactActive ? 'active' : ''">
              <font-awesome-icon icon="pen" />
              Update Zoo Info
            </a>
          </router-link>
        </li>

      </ul>

    </div>
  </nav>
</template>

<script>
export default {
  name: "ProfileSidebar"
}
</script>

<style scoped>
.nav-item a {
  color: #222222;
}

.nav-item a:hover {
  color: #5cb874;
}
</style>