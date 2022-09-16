import '../styles/globals.css'
import useStore from '../store';

function MyApp({ Component, pageProps }) {
  let store: any = useStore();
  return <Component {...pageProps} />
}

export default MyApp
