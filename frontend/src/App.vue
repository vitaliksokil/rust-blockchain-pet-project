<template>
  <div>
    <header-comp :is-signed-in="isSignedIn" :account-id="accountId"></header-comp>
    <router-view></router-view>
  </div>
</template>

<script>
import HeaderComp from './components/HeaderComp.vue'

export default {
  name: 'App',
  data(){
    return {
      accountId: '',
      isSignedIn: false,
    }
  },
  methods:{
    updateInfo(){
      this.accountId = window.nearAccount.accountId;
      this.isSignedIn = window.nearAccount.isSignedIn;
    },
  },
  async mounted(){
    this.updateInfo();
  },
  components: {
    'header-comp':HeaderComp,
  },

}
</script>

<style>

</style>
