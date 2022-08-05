import { createApp } from 'vue'
import App from './App.vue'
import './assets/scss/app.scss'
import {init} from "@/utils";
import VueLoading from 'vue-loading-overlay';
import 'vue-loading-overlay/dist/vue-loading.css';

import VueSweetalert2 from 'vue-sweetalert2';
import 'sweetalert2/dist/sweetalert2.min.css';

import { library } from '@fortawesome/fontawesome-svg-core'
import { faShieldDog,
    faUserDoctor,
    faScrewdriverWrench,
    faBone,
    faCopy,
    faArrowRightFromBracket,
    faHouse,
    faCirclePlus,
    faPen,
} from '@fortawesome/free-solid-svg-icons'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
library.add(faShieldDog,
    faUserDoctor,
    faScrewdriverWrench,
    faBone,
    faCopy,
    faArrowRightFromBracket,
    faHouse,
    faCirclePlus,
    faPen,
)

import VueClipboard from 'vue3-clipboard'

import filters from "@/filters";

import 'bootstrap/dist/js/bootstrap.bundle';




await init();

import {router} from './router.js'

let app = createApp(App);
app.config.globalProperties.window = window;
app.config.globalProperties.$filters = filters;

app.use(VueLoading);
app.use(VueSweetalert2);
app.use(router);
app.use(VueClipboard,{
    autoSetContainer: true,
    appendToBody: true,
});
app.component('font-awesome-icon', FontAwesomeIcon);
app.mount('#app');
