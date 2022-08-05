import {createRouter, createWebHashHistory, createWebHistory} from "vue-router";

const publicPath = window.__RUNTIME_CONFIG__.VUE_APP_NODE_ENV === 'production'
    ? '/ch6-zoo-nft-by-near-ukraine/'
    : '/';
const routes = [
    { path: publicPath, component: () => import('./components/Pages/HomeComponent'), name:'home'  },
    { path: publicPath + 'register', component: () => import('./components/Pages/Auth/RegisterComponent'), name:'register'  },
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
    history: window.__RUNTIME_CONFIG__.VUE_APP_NODE_ENV === 'production' ? createWebHashHistory () : createWebHistory(),
    routes, // short for `routes: routes`
})

export { router }