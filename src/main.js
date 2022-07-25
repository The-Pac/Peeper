import {createApp} from 'vue'
import App from './app/App.vue'
import {addIcons, OhVueIcon} from "oh-vue-icons";
import {CoExitToApp} from "oh-vue-icons/icons/co";
import {MdDeleteoutline} from "oh-vue-icons/icons/md";
import router from "@/app/Router";

const app = createApp(App)

//ICONS
addIcons(CoExitToApp, MdDeleteoutline)
app.component("v-icon", OhVueIcon);

//ROUTES
app.use(router)


//MOUNT
app.mount('#app');
