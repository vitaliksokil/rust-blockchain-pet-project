import { createApp } from 'vue'
import App from './App.vue'
import { createWebHashHistory, createRouter } from "vue-router";
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


const publicPath = process.env.VUE_APP_NODE_ENV === 'production'
    ? '/ch6-zoo-nft-by-near-ukraine/'
    : '/';
const routes = [
    { path: publicPath, component: () => import('./components/Pages/HomeComponent'), name:'home'  },
    { path: publicPath + 'zoos', component: () => import('./components/Pages/ZoosComponent'), name:'zoos' },
    { path: publicPath + 'zoos/:id', component: () => import('./components/Pages/SingleZooComp'), name:'zoo-single' },
    {
        path: publicPath + 'profile',
        component: () => import('./components/Pages/Profile/ProfileComponent'),
        name:'profile',
        beforeEnter: () => {
            if (!window.nearAccount.accountId){
                return { name:'home' }
            }
        },
        children: [
            {
                path: '',
                component: () => import('./components/Pages/Profile/ProfileDashboard'),
                name: 'profile-dashboard',
            },
            {
                path: 'create-zoo',
                component: () => import('./components/Pages/Profile/CreateZoo'),
                name: 'create-zoo',
            },
            {
                path: 'update-zoo',
                component: () => import('./components/Pages/Profile/UpdateZoo'),
                name: 'update-zoo',
            },
        ]
    },
]
const router = createRouter({
    history: createWebHashHistory(),
    routes, // short for `routes: routes`
})


await init();


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
