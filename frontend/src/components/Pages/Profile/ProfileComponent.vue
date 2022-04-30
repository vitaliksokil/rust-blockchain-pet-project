<template>
<div>
  <div class="container-fluid" style="border-top: 1px solid #444">
    <div class="row">
      <profile-sidebar></profile-sidebar>
      <main class="col-md-9 ms-sm-auto col-lg-10 p-0">
        <router-view></router-view>
      </main>
    </div>
  </div>
</div>
</template>

<script>
import ProfileSidebar from "@/components/Pages/Profile/ProfileSidebar";
export default {
  name: "ProfileComponent",
  components:{
    ProfileSidebar
  }
}
</script>

<style scoped>

</style>