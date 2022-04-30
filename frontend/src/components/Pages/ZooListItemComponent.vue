<template>
  <div class="col-lg-6">
    <router-link
        :to="{name:'zoo-single',params:{id: `${id}` }}"
        custom
        v-slot="{ href, navigate }">
      <a class="portfolio-item" :href="href" @click="navigate">
        <div class="caption">
          <div class="caption-content">
            <div class="h2">{{ title }}</div>
            <p class="mb-0">{{ description }}</p>
          </div>
        </div>
        <img class="img-fluid" :src="banner_image" :alt="title" style="height: 300px; width: 100%"/>
      </a>
    </router-link>
  </div>
</template>

<script>

export default {
  name: "ZooListItemComponent",
  props: [
    'id',
    'title',
    'banner_image',
    'description',
  ],
}
</script>

<style scoped>

</style>