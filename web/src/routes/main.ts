import '../app.css'
import Page from './page.svelte'

const target = document.getElementById('app');
let page: Page | undefined;
if (target) {
  page = new Page({ target });
}

export default page
