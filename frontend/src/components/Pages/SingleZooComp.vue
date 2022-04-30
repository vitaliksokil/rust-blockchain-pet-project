<template>
<div>
  <div class="zoo" :style="{ backgroundImage: 'linear-gradient(125deg, rgba(25, 25, 25, 0.5) 0%, rgba(25, 25, 25, 0.5) 100%), url(' + zoo.banner_image + ')' }">
    <div class="container">
      <div class="row">
        <div class="col-lg-8 col-md-10 mx-auto">
          <div class="post-heading">
            <h1>{{ zoo.title }}</h1>
            <h2 class="subheading">{{ zoo.description }}</h2>
            <span class="meta">Address: {{zoo.address}}
            </span>
          </div>
        </div>
      </div>
    </div>
  </div>
  <div class="container">
    <div class="row">
      <section class="page-section clearfix">
        <div class="container">
          <div class="intro">
            <img class="intro-img img-fluid mb-3 mb-lg-0 rounded" :src="zoo.nft_media" alt="..." />
            <div class="intro-text left-0 text-center bg-warning p-5 rounded">
              <h2 class="section-heading mb-4">
                <span class="section-heading-upper">Buy NFT</span>
                <span class="section-heading-lower">Help Zoo</span>
              </h2>

              <p class="mb-3">Buy our Zoo NFT to help us! Every cent is important for us. Help us to feed our animals in this hard time. </p>
              <hr>
              <div class="intro-button mx-auto"><a class="btn btn-primary btn-xl fw-bold" href="#" @click.prevent="buyNftAction">Buy NFT for {{ $filters.transformYoctoNear(zoo.nft_price) }} Ⓝ</a></div>
            </div>
          </div>
        </div>
      </section>
    </div>
  </div>
  <section class=" bg-primary text-white text-center p-5" id="services">
    <div class="container px-4 px-lg-5">
      <div class="content-section-heading">
        <h2 class="mb-5">Statistics</h2>
      </div>
      <div class="row gx-4 gx-lg-5 justify-content-center">
        <div class="col-lg-3 col-md-6 mb-5 mb-lg-0">
          <span class="service-icon rounded-circle mx-auto mb-3">
            <img :src="require('@/assets/near-wallet-icon.png')" alt="" class="icon-image">
          </span>
          <h4><strong>{{ $filters.transformYoctoNear(zoo.total_collected) }} Ⓝ</strong></h4>
          <p class="text-faded mb-0">Collected money in NEAR</p>
        </div>
        <div class="col-lg-3 col-md-6 mb-5 mb-lg-0">
          <span class="service-icon rounded-circle mx-auto mb-3">
            <img :src="require('@/assets/nft.png')" alt="" class="icon-image">
          </span>
          <h4><strong>{{ zoo.nft_sold }}</strong></h4>
          <p class="text-faded mb-0">NFTs sold</p>
        </div>

      </div>
    </div>
  </section>

  <div class=" bg-warning">
    <section class="p-5 cta">
      <div class="container">
        <div class="row">
          <div class="col-xl-9 mx-auto">
            <div class="cta-inner text-center rounded p-4">
              <h2 class="section-heading mb-4">
                <span class="section-heading-upper">Our Promise</span>
                <span class="section-heading-lower">To You</span>
              </h2>
              <p class="mb-0 section-text-black">
                We promise to you that all money donated to our zoo will be used to help animals and zoo. We will spend this money for feeding animals, zoo restore after the war is over, veterinary for animals.
              </p>
            </div>
          </div>
        </div>
      </div>
    </section>

  </div>
</div>


</template>

<script>
import Big from "big.js";
import {getZooById} from "@/utils";

const BOATLOAD_OF_GAS = Big(3).times(10 ** 13).toFixed();

export default {
  name: "SingleZooComp",
  data() {
    return {
      zoo: {},
    }
  },
  methods: {

    async buyNft() {
      await window.walletSelector.signAndSendTransaction({
        signerId: window.nearAccount.accountId,
        actions: [
          {
            type: "FunctionCall",
            params: {
              methodName: "buy_nft",
              args: {
                "zoo_id": this.zoo.owner_id,
              },
              gas: BOATLOAD_OF_GAS,
              deposit: Big((this.zoo.nft_price / (10 ** 24)).toFixed(5)).times(10 ** 24).toFixed()
            },
          },
        ]
      });
    },

    async buyNftAction() {
      let loader = this.$loading.show();
      try {
        await this.buyNft();
        this.$swal.fire({
          icon: 'success',
          title: 'Success',
          text: 'Nft successfully bought!',
          footer: `To look at your NFTs go to your wallet collectibles!`,
        })
        this.zoo = await getZooById(this.$route.params.id);
      } catch (error) {
        this.$swal.fire({
          icon: 'error',
          title: 'Error',
          text: error.message,
          footer: ``,
        })
        console.log(error)
      }
      loader.hide();

    }
  },
  async mounted() {
    let loader = this.$loading.show();
    try {
      this.zoo = await getZooById(this.$route.params.id);
    }catch (error){
      this.$router.push({'name': 'home'});
    }
    loader.hide();

    let uri = window.location.search.substring(1);
    let params = new URLSearchParams(uri);
    const transactionHash = params.get('transactionHashes');
    if(transactionHash){
      this.$swal.fire({
        icon: 'success',
        title: 'Success',
        text: 'NFT was successfully bought!',
        footer: `To look at your ticket go to your wallet collectibles!`,
      });

      window.history.pushState({}, document.title, process.env.VUE_APP_APP_URL + '/#/zoos/' + this.$route.params.id);

    }
  },
}
</script>

<style scoped lang="scss">

.zoo {

  background: no-repeat center center;
  background-color: #212529;
  background-attachment: scroll;
  position: relative;
  background-size: cover;

  .post-heading, {
    padding: 150px 0 150px;
    color: white !important;
  }

  .post-heading {
    z-index: 100;
    h1 {
      font-size: 75px;
      color: #ffffff;
      font-weight: bold;
    }

    .meta,
    .subheading {
      line-height: 1.1;
      display: block;
    }

    .subheading {
      font-size: 24px;
      font-weight: 600;
      margin: 10px 0 30px;
    }

    .meta {
      font-size: 20px;
      font-weight: 300;
      font-style: italic;

      a {
        color: #fff;
      }
    }

  }
}


.intro {
  position: relative;
}
@media (min-width: 992px) {
  .intro .intro-img {
    width: 75%;
    float: right;
  }
  .intro .intro-text {
    color: #3b3b3b;
    left: -130px;
    width: 60%;
    margin-top: 3rem;
    position: absolute;
    border: 3px solid rgba(29,128,159,0.6);
  }
  .intro .intro-text .intro-button {
    width: 100%;
    left: 0;
    position: absolute;
    bottom: -2rem;
  }
}
@media (min-width: 1200px) {
  .intro .intro-text {
    width: 45%;
  }
}

.section-heading {
  text-transform: uppercase;
  color: #3b3b3b;

}
.section-heading .section-heading-upper {
  display: block;
  font-size: 1rem;
  font-weight: 800;
}
.section-heading .section-heading-lower {
  display: block;
  font-size: 3rem;
  font-weight: 100;
}

.img-fluid {
  max-width: 100%;
  height: auto;
}
.page-section {
  margin-top: 5rem;
  margin-bottom: 5rem;
}

.clearfix::after {
  display: block;
  clear: both;
  content: "";
}

</style>