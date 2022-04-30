<template>
  <section class="site-section">
    <div class="container">

      <div class="row">
        <section class="content-section" id="portfolio">
          <div class="container px-4 px-lg-5">
            <div class="content-section-heading text-center">
              <h2 class="mb-5">Zoos</h2>
            </div>
            <div class="row gx-0">
              <zoo-list-item-component v-for="(item, key) in zoos" :key="key"
                  :id="item.owner_id"
                  :title="item.title"
                  :banner_image="item.banner_image"
                  :description="item.description"
              ></zoo-list-item-component>
            </div>
          </div>
        </section>

      </div>
    </div>
  </section>
</template>

<script>
import ZooListItemComponent from "@/components/Pages/ZooListItemComponent";
export default {
  name: "ZoosComponent",
  components:{
    ZooListItemComponent
  },
  data(){
    return {
      zoos : {}
    }
  },
  methods:{
    async getZoos(){
      let result = await window.provider.query({
        request_type: "call_function",
        account_id: window.walletSelector.getContractId(),
        method_name: "get_all_zoos",
        args_base64: '',
        finality: "optimistic",
      })
      this.zoos = JSON.parse(Buffer.from(result.result).toString())
    }
  },
  async mounted() {
    let loader = this.$loading.show();
    await this.getZoos()
    loader.hide()
  },
}
</script>

<style scoped>


</style>