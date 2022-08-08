<template>
  <div class="container m-auto p-5">
    <div class="card card-form">
      <div class="card-heading">
        <h4 class="title">Event Registration Form</h4>
      </div>
      <div class="card-body">
        <form>
          <div class="mb-3">
            <label for="validationTooltipUsername">Enter your desired wallet name</label>
            <div class="input-group">
              <input type="text" class="form-control" id="validationTooltipUsername" placeholder="WalletName" aria-describedby="validationTooltipUsernamePrepend" required>
              <div class="input-group-prepend">
                <span class="input-group-text" id="validationTooltipUsernamePrepend">.{{window.__RUNTIME_CONFIG__.VUE_APP_CREATOR_NAME}}</span>
              </div>
              <div class="invalid-tooltip">
                Please choose a unique and valid username.
              </div>
            </div>
          </div>
          <div class="mt-3 text-end">
            <button type="submit" class="btn btn-primary">Create</button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script>

export default {
  name: "RegisterComponent",
  methods: {

  }
}
</script>

<style scoped>

</style>