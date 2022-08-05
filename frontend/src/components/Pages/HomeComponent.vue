<template>
  <div>
    <section class="masthead d-flex align-items-center">
      <div class="container px-4 px-lg-5 text-center">
        <h1 class="mb-1">{{ appName }}</h1>
        <h3 class="mb-5"><em>Help Ukrainian Zoos</em></h3>
      </div>
    </section>

    <!-- About-->
    <section class="content-section bg-light" id="about">
      <div class="container px-4 px-lg-5 text-center">
        <div class="row gx-4 gx-lg-5 justify-content-center">
          <div class="col-lg-10">
            <h2>About</h2>
            <hr>
            <p class="lead mb-5">
              The war in Ukraine has affected each Ukrainian and a lot of people around the globe. At the moment the war is being actively waged and is in the hot phase, but when Ukraine wins, a new stage will begin - the stage of restoration of our country. Ukrainian zoos are in severe need of help now and will be in need after the war ends. There are no visitors in the zoos, which results in no budgeting
            </p>
          </div>
        </div>
      </div>
    </section>
    <!-- Services-->
    <section class="content-section bg-primary text-white text-center" id="services">
      <div class="container px-4 px-lg-5">
        <div class="content-section-heading">
          <h2 class="mb-5">Help zoos</h2>
        </div>
        <div class="row gx-4 gx-lg-5">
          <div class="col-lg-3 col-md-6 mb-5 mb-lg-0">
            <span class="service-icon rounded-circle mx-auto mb-3"><font-awesome-icon icon="bone" /></span>
            <h4><strong>Food</strong></h4>
            <p class="text-faded mb-0">Zoos need money for feeding animals</p>
          </div>
          <div class="col-lg-3 col-md-6 mb-5 mb-lg-0">
            <span class="service-icon rounded-circle mx-auto mb-3"><font-awesome-icon icon="screwdriver-wrench" /></span>
            <h4><strong>Zoos restore</strong></h4>
            <p class="text-faded mb-0">Help zoos restore their work after the war is over</p>
          </div>
          <div class="col-lg-3 col-md-6 mb-5 mb-md-0">
            <span class="service-icon rounded-circle mx-auto mb-3">
             <font-awesome-icon icon="shield-dog" />
            </span>
            <h4><strong>Veterinary medicine</strong></h4>
            <p class="text-faded mb-0">
              Help collect money for veterinary
            </p>
          </div>
          <div class="col-lg-3 col-md-6">
            <span class="service-icon rounded-circle mx-auto mb-3"><font-awesome-icon icon="user-doctor" /></span>
            <h4><strong>Staff workers</strong></h4>
            <p class="text-faded mb-0">Help staff workers to do their work better</p>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script>
export default {
  name: "HomeComponent",
  computed:{
    appName(){
      return window.__RUNTIME_CONFIG__.VUE_APP_APP_NAME
    }
  }
}
</script>

<style scoped>

/*--------------------------------------------------------------
# Hero Section
--------------------------------------------------------------*/
#hero {
  width: 100%;
  height: calc(100vh - 70px);
  background-color: rgba(9, 9, 9, 0.8);
  overflow: hidden;
  position: relative;
}
#hero .carousel, #hero .carousel-inner, #hero .carousel-item, #hero .carousel-item::before {
  background-size: cover;
  background-position: center;
  background-repeat: no-repeat;
  position: absolute;
  top: 0;
  right: 0;
  left: 0;
  bottom: 0;
}
#hero .carousel-item::before {
  content: "";
  background-color: rgba(45, 103, 60, 0.8);
}
#hero .carousel-container {
  display: flex;
  justify-content: center;
  align-items: center;
  position: absolute;
  bottom: 0;
  top: 0;
  left: 50px;
  right: 50px;
}
#hero .container {
  text-align: center;
}
#hero h2 {
  color: #fff;
  margin-bottom: 20px;
  font-size: 48px;
  font-weight: 700;
}
#hero p {
  -webkit-animation-delay: 0.4s;
  animation-delay: 0.4s;
  margin: 0 auto 30px auto;
  color: #fff;
}
#hero .carousel-inner .carousel-item {
  transition-property: opacity;
  background-position: center top;
}
#hero .carousel-inner .carousel-item,
#hero .carousel-inner .active.carousel-item-start,
#hero .carousel-inner .active.carousel-item-end {
  opacity: 0;
}
#hero .carousel-inner .active,
#hero .carousel-inner .carousel-item-next.carousel-item-start,
#hero .carousel-inner .carousel-item-prev.carousel-item-end {
  opacity: 1;
  transition: 0.5s;
}
#hero .carousel-control-next-icon, #hero .carousel-control-prev-icon {
  background: none;
  font-size: 30px;
  line-height: 0;
  width: auto;
  height: auto;
  background: rgba(255, 255, 255, 0.2);
  border-radius: 50px;
  transition: 0.3s;
  color: rgba(255, 255, 255, 0.5);
  width: 54px;
  height: 54px;
  display: flex;
  align-items: center;
  justify-content: center;
}
#hero .carousel-control-next-icon:hover, #hero .carousel-control-prev-icon:hover {
  background: rgba(255, 255, 255, 0.3);
  color: rgba(255, 255, 255, 0.8);
}
#hero .carousel-indicators li {
  cursor: pointer;
  background: #fff;
  overflow: hidden;
  border: 0;
  width: 12px;
  height: 12px;
  border-radius: 50px;
  opacity: 0.6;
  transition: 0.3s;
}
#hero .carousel-indicators li.active {
  opacity: 1;
  background: #5cb874;
}
#hero .btn-get-started {
  font-family: "Raleway", sans-serif;
  font-weight: 500;
  font-size: 14px;
  letter-spacing: 1px;
  display: inline-block;
  padding: 14px 32px;
  border-radius: 4px;
  transition: 0.5s;
  line-height: 1;
  color: #fff;
  -webkit-animation-delay: 0.8s;
  animation-delay: 0.8s;
  background: #5cb874;
}
#hero .btn-get-started:hover {
  background: #6ec083;
}
@media (max-width: 992px) {
  #hero {
    height: 100vh;
  }
  #hero .carousel-container {
    top: 66px;
  }
}
@media (max-width: 768px) {
  #hero h2 {
    font-size: 28px;
  }
}
@media (min-width: 1024px) {
  #hero .carousel-control-prev, #hero .carousel-control-next {
    width: 5%;
  }
}
@media (max-height: 500px) {
  #hero {
    height: 120vh;
  }
}
</style>