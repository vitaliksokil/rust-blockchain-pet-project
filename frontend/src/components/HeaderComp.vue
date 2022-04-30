<template>
    <nav id="sidebar-wrapper" class="d-flex">
      <h1 class="sidebar-brand">
        <router-link :to="{name:'home'}">{{appName}}</router-link>
      </h1>
      <div class="d-flex justify-content-between w-100 container">
      <ul class="sidebar-nav d-flex align-content-center">
        <li class="sidebar-nav-item">
          <router-link
              :to="{name:'home'}"
              custom
              v-slot="{ href, navigate, isExactActive }">
            <a :href="href" @click="navigate"
               class="nav-link scrollto" :class="isExactActive ? 'active' : ''" >Home</a>
          </router-link>
        </li>
        <li class="sidebar-nav-item">
          <router-link
              :to="{name:'zoos'}"
              custom
              v-slot="{ href, navigate, isExactActive }">
            <a :href="href" @click="navigate"
               class="nav-link scrollto" :class="isExactActive ? 'active' : ''" >Zoos</a>
          </router-link>
        </li>

      </ul>
      <ul class="sidebar-nav d-flex align-content-center">
        <li class="dropdown" style="margin-left: 30px"  v-if="isSignedIn">
          <button class="btn btn-outline-light dropdown-toggle" type="button" id="dropdownMenuButton1" data-bs-toggle="dropdown" aria-expanded="false">
            {{ shortAddressId }}
          </button>
          <ul class="dropdown-menu" style="background: #1D809F;" aria-labelledby="dropdownMenuButton1">
            <li><a class="dropdown-item" href="#" v-clipboard:copy="accountId">{{ shortAddressId }} <font-awesome-icon icon="copy" /></a></li>
            <hr>
            <li><router-link
                :to="{name:'profile-dashboard'}"
                custom
                v-slot="{ href, navigate }">
              <a :href="href" @click="navigate"
                 class="dropdown-item" >Profile</a>
            </router-link>
            </li>
            <li v-if="isSignedIn"><a class="dropdown-item" href="#" @click.prevent="signOut">Sign Out <font-awesome-icon icon="arrow-right-from-bracket" /></a></li>
          </ul>
        </li>
        <li class="sidebar-nav-item" v-else><a href="#" class="text-decoration-underline m-2" @click.prevent="login" >Log In</a></li>
      </ul>
      </div>
    </nav>
</template>

<script>
import {signIn, signOut} from "@/utils";

export default {
  name: "HeaderComp",
  props:["accountId", "isSignedIn"],
  filters: {
    shortAddress: function (value) {
      if (!value) return ''
      if(value.length > 25){
        return value.substring(0, 5) + '...' + value.substring(value.length - 5);
      }
      return value;
    }
  },
  methods:{
    async login(){
      if(!this.isSignedIn){
        await signIn();
      }
    },
    async signOut(){
      await signOut();
      this.$parent.updateInfo();
    },
  },
  computed:{
    appName(){
      return process.env.VUE_APP_APP_NAME
    },
    shortAddressId(){
      return this.$options.filters.shortAddress(this.accountId)
    }
  }
}
</script>

<style scoped>
.dropdown-item{
  color: #fff;
}
</style>