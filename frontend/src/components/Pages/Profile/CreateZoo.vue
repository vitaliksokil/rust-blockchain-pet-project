<template>
  <div class=" p-5 rounded">
    <form class="bg-white p-3 rounded" v-if="!myZoo">
      <div class="mb-3">
        <label for="title" class="form-label">Title</label>
        <input type="text" class="form-control" id="title" v-model='form.title'>
        <div class="form-text text-danger" v-if="form.errors().has('title')" v-text="form.errors().get('title')"></div>
      </div>
      <div class="mb-3">
        <label for="description" class="form-label">Description</label>
        <input type="text" class="form-control" id="description" v-model='form.description'>
        <div class="form-text text-danger" v-if="form.errors().has('description')"
             v-text="form.errors().get('description')"></div>
      </div>
      <div class="mb-3">
        <label for="address" class="form-label">Address</label>
        <input type="text" class="form-control" id="address" v-model='form.address'>
        <div class="form-text text-danger" v-if="form.errors().has('address')"
             v-text="form.errors().get('address')"></div>
      </div>
      <div class="mb-3">
        <label for="banner_image" class="form-label">Banner Image</label>
        <input type="file" class="form-control" id="banner_image" @change="bannerImageOnChange" required>
        <div class="form-text text-danger" v-if="form.errors().has('banner_image')"
             v-text="form.errors().get('banner_image')"></div>
        <img class="my-4" width="350" :src="form.banner_image"/>
      </div>
      <div class="mb-3">
        <label for="nft_media" class="form-label">NFT Media</label>
        <input type="file" class="form-control" id="nft_media" @change="nftMediaOnChange" required>
        <div class="form-text text-danger" v-if="form.errors().has('nft_media')"
             v-text="form.errors().get('nft_media')"></div>
        <img class="my-4" width="350" :src="form.nft_media"/>
      </div>
      <div class="mb-3">
        <label for="nft_price" class="form-label">NFT price</label>
        <input type="text" class="form-control" id="nft_price" placeholder="Ⓝ" v-model='form.nft_price'>
        <div class="form-text text-danger" v-if="form.errors().has('nft_price')"
             v-text="form.errors().get('nft_price')"></div>
      </div>

      <button type="submit" class="btn btn-primary" :disabled='form.empty()' @click.prevent='submit'>Submit</button>
    </form>
    <div class="alert alert-warning " role="alert" v-if="myZoo">
      <p class="mb-4 text-center">You have been already create your zoo! You can update its info or look at this.</p>
      <div class="row justify-content-center">
        <zoo-list-item-component
            :id="myZoo.owner_id"
            :title="myZoo.title"
            :banner_image="myZoo.banner_image"
            :description="myZoo.description"
        ></zoo-list-item-component>
      </div>
    </div>
  </div>
</template>

<script>
import {getZooById} from "@/utils";
import form from 'vuejs-form'
import Big from "big.js";
import {NFTStorage} from 'nft.storage'
import ZooListItemComponent from "@/components/Pages/ZooListItemComponent";
const BOATLOAD_OF_GAS = Big(3).times(10 ** 13).toFixed();

export default {
  name: "CreateZoo",
  components:{ZooListItemComponent},
  data() {
    return {
      myZoo:{},
      form: form({
        title: '',
        description: '',
        address: '',
        banner_image: '',
        nft_media: '',
        nft_price: '',
      }).rules({
        title: 'min:3|required|max:1000|string',
        description: 'required|min:5|max:2000|string',
        address: 'required|min:5|max:2000|string',
        banner_image: 'required|string',
        nft_media: 'required|string',
        nft_price: 'required|numeric',
      })
    }
  },
  methods: {
    async fileOnChange(e) {
      let loader = this.$loading.show();
      /* upload image to IPFS */
      const file = e.target.files[0];
      const client = new NFTStorage({token: window.__RUNTIME_CONFIG__.VUE_APP_NFT_STORAGE_TOKEN});
      const metadataCid = await client.storeBlob(file)
      const metadataUrl = "https://ipfs.io/ipfs/" + metadataCid;
      loader.hide();
      return metadataUrl;
    },
    async nftMediaOnChange(e) {
      this.form.nft_media = await this.fileOnChange(e)
    },
    async bannerImageOnChange(e) {
      this.form.banner_image = await this.fileOnChange(e);
    },
    cleanUpForm() {
      this.form.title = '';
      this.form.description = '';
      this.form.address = '';
      this.form.banner_image = '';
      this.form.nft_media = '';
      this.form.nft_price = '';
    },
    submit() {
      this.form.validate()
      return this.form.errors().any() ? this.failed() : this.passed();
    },
    failed() {
      console.log('errors: ', this.form.errors().all());
    },

    async createZoo(
        title,
        description,
        address,
        banner_image,
        nft_media,
        nft_price,
    ) {
      await window.walletSelector.signAndSendTransaction({
        signerId: window.nearAccount.accountId,
        actions: [
          {
            type: "FunctionCall",
            params: {
              methodName: "add_new_zoo",
              args: {
                "title": title,
                "description": description,
                "address": address,
                "banner_image": banner_image,
                "nft_media": nft_media,
                "nft_price": Big(nft_price).times(10 ** 24).toFixed(),
              },
              gas: BOATLOAD_OF_GAS,
              deposit: 0
            },
          },
        ]
      }).catch((err) => {
        console.log("Failed");
        throw err;
      });
    },
    async passed() {
      let loader = this.$loading.show();
      try {
        await this.createZoo(
            this.form.title,
            this.form.description,
            this.form.address,
            this.form.banner_image,
            this.form.nft_media,
            this.form.nft_price,
        );
        this.myZoo = await getZooById(window.nearAccount.accountId);
        this.$swal.fire({
          icon: 'success',
          title: 'Success',
          text: 'Your Zoo has been created!',
          footer: ``,
        })
        this.cleanUpForm();
      } catch (error) {
        console.log(error.message);
        this.$swal.fire({
          icon: 'error',
          title: 'Error',
          text: error.message,
        })

      }
      loader.hide();
    },

  },
  async mounted() {
    let loader = this.$loading.show();
    try {
      this.myZoo = await getZooById(window.nearAccount.accountId);
    } catch (error) {
      this.myZoo = false;
    }
    loader.hide();
  }
}
</script>

<style scoped>

</style>