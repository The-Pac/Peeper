import {createApp} from 'vue'
import App from './app/App.vue'
import {addIcons, OhVueIcon} from "oh-vue-icons";
import {CoMediaPlay, BiPeopleFill, CoExitToApp} from "oh-vue-icons/icons";
import router from "@/app/Router";

const app = createApp(App)

//ICONS
addIcons(CoMediaPlay, BiPeopleFill, CoExitToApp)
app.component("v-icon", OhVueIcon);

//ROUTES
app.use(router)


//MOUNT
app.mount('#app');
