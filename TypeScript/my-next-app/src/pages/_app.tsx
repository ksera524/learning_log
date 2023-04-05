import type { AppProps } from "next/app";
import "react-quill/dist/quill.snow.css";
import "../../styles/quill-custom.css";
export default function App({ Component, pageProps }: AppProps) {
  return <Component {...pageProps} />;
}
