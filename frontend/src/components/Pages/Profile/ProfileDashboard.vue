<template>
  <section class=" bg-warning text-white text-center p-5 section-text-black" id="services">
    <div class="container px-4 px-lg-5">
      <div class="content-section-heading">
        <h2 class="mb-5">Your Zoo Statistics</h2>
      </div>
      <div class="row gx-4 gx-lg-5 justify-content-center">
        <div class="col-lg-3 col-md-6 mb-5 mb-lg-0">
          <span class="service-icon rounded-circle mx-auto mb-3">
            <img :src="require('@/assets/near-wallet-icon.png')" alt="" class="icon-image">
          </span>
          <h4><strong>{{ $filters.transformYoctoNear(myZoo.total_collected) }} Ⓝ</strong></h4>
          <p class=" mb-0">Collected money in NEAR</p>
        </div>
        <div class="col-lg-3 col-md-6 mb-5 mb-lg-0">
          <span class="service-icon rounded-circle mx-auto mb-3">
            <img :src="require('@/assets/nft.png')" alt="" class="icon-image">
          </span>
          <h4><strong>{{myZoo.nft_sold}}</strong></h4>
          <p class=" mb-0">NFTs sold</p>
        </div>

      </div>
    </div>
  </section>
</template>

<script>
import {getZooById} from "@/utils";

export default {
  name: "ProfileDashboard",
  data(){
    return {
      myZoo: {
        total_collected:0,
        nft_sold: 0
      }
    }
  },

  async mounted() {
    let loader = this.$loading.show();
    try {
      this.myZoo = await getZooById(window.nearAccount.accountId);
    } catch (error) {
      console.log(error)
    }
    loader.hide();
  }
}
</script>

<style scoped>

</style>